//! rbac pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{types::*, Pallet as RBAC};
use scale_info::prelude::*;

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, pallet_prelude::*, traits::Get};
use frame_system::RawOrigin;
use sp_io::hashing::blake2_256;

const SEED: u32 = 0;

pub fn generate_pallet_id_sized(id: u8, size: u32) -> IdOrVec {
	IdOrVec::Vec(generate_vector(0, id, size))
}

pub fn generate_role_sized(id: u8, size: u32) -> Vec<u8> {
	generate_vector(1, id, size)
}

pub fn generate_scope_sized(id: u8, size: u32) -> Vec<u8> {
	generate_vector(2, id, size)
}

pub fn generate_roles_sized(num_roles: u32, size: u32) -> Vec<Vec<u8>> {
	let mut roles = Vec::new();
	for r in 0..num_roles {
		roles.push(generate_role_sized(r as u8, size));
	}
	roles
}

pub fn generate_id(item: Vec<u8>) -> [u8; 32] {
	item.using_encoded(blake2_256)
}

pub fn setup_roles_sized<T: Config>(pallet_id: IdOrVec, num_roles: u32, size: u32) -> Vec<RoleId> {
	let roles = generate_roles_sized(num_roles, size);
	assert_ok!(RBAC::<T>::tx_create_and_set_roles(
		RawOrigin::Root.into(),
		pallet_id,
		roles.clone()
	));
	roles.into_iter().map(|role| generate_id(role)).collect()
}

pub fn setup_scopes_sized<T: Config>(pallet_id: IdOrVec, num_scopes: u32) -> Vec<ScopeId> {
	let mut scopes = Vec::new();
	for s in 0..num_scopes {
		let scope = generate_scope_sized(s as u8, 10);
		let scope_id = generate_id(scope);
		assert_ok!(RBAC::<T>::create_scope(pallet_id.clone(), scope_id));
		scopes.push(scope_id);
	}
	scopes
}

pub fn assign_roles_to_user<T: Config>(
	user: T::AccountId,
	pallet_id: IdOrVec,
	scope_id: ScopeId,
	role_ids: &[RoleId],
) {
	for role_id in role_ids {
		assert_ok!(RBAC::<T>::tx_assign_role_to_user(
			RawOrigin::Root.into(),
			user.clone(),
			pallet_id.clone(),
			scope_id,
			role_id.clone()
		));
	}
}

pub fn assign_roles_to_users<T: Config>(
	num_users: u32,
	pallet_id: IdOrVec,
	scope_id: ScopeId,
	role_ids: &[RoleId],
) -> Vec<T::AccountId> {
	let mut users = Vec::new();
	for u in 0..num_users {
		let user: T::AccountId = account("user", u, SEED);
		assign_roles_to_user::<T>(user.clone(), pallet_id.clone(), scope_id, role_ids);
		users.push(user);
	}
	users
}

pub fn generate_permission_sized(id: u8, size: u32) -> Vec<u8> {
	generate_vector(2, id, size)
}

pub fn generate_permissions_sized(num_permissions: u32, size: u32) -> Vec<Vec<u8>> {
	let mut permissions = Vec::new();
	for p in 0..num_permissions {
		permissions.push(generate_permission_sized(p as u8, size));
	}
	permissions
}

pub fn setup_permissions_sized<T: Config>(
	pallet_id: IdOrVec,
	role_id: RoleId,
	num_permissions: u32,
	size: u32,
) -> Vec<PermissionId> {
	let permissions = generate_permissions_sized(num_permissions, size);
	assert_ok!(RBAC::<T>::tx_create_and_set_permissions(
		RawOrigin::Root.into(),
		pallet_id.clone(),
		role_id.clone(),
		permissions.clone()
	));
	permissions.into_iter().map(|permission| generate_id(permission)).collect()
}

pub fn generate_vector(prefix: u8, id: u8, size: u32) -> scale_info::prelude::vec::Vec<u8> {
	assert!(size > 0, "vector size must be greater than 0");
	let mut v = vec![id; size as usize];
	v[0] = prefix;
	v
}

#[benchmarks(where T: Config)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn tx_create_and_set_roles(
		i: Linear<1, 400>,
		l: Linear<2, { T::RoleMaxLen::get() }>,
		r: Linear<1, { T::MaxRolesPerPallet::get() }>,
	) {
		let pallet_id = generate_pallet_id_sized(0, i);
		let roles = generate_roles_sized(r, l);
		#[extrinsic_call]
		_(RawOrigin::Root, pallet_id.clone(), roles);
		assert_eq!(RBAC::<T>::pallet_roles(pallet_id.to_id()).len() as u32, r);
	}

	#[benchmark]
	fn tx_remove_role_from_user(
		i: Linear<1, 400>,
		s: Linear<1, { T::MaxScopesPerPallet::get() }>,
		r: Linear<1, { T::MaxRolesPerUser::get() }>,
		u: Linear<1, { T::MaxUsersPerRole::get() }>,
	) {
		let pallet_id = generate_pallet_id_sized(0, i);
		let scope_id = setup_scopes_sized::<T>(pallet_id.clone(), s)[0];
		let role_ids = setup_roles_sized::<T>(
			pallet_id.clone(),
			T::MaxRolesPerPallet::get(),
			T::RoleMaxLen::get(),
		);
		let ru = r as usize;
		let user =
			assign_roles_to_users::<T>(u, pallet_id.clone(), scope_id, &role_ids[0..ru])[0].clone();
		let role_id = role_ids[ru - 1];
		#[extrinsic_call]
		_(RawOrigin::Root, user.clone(), pallet_id.clone(), scope_id, role_id);
		assert!(!RBAC::<T>::roles_by_user((user, pallet_id.to_id(), scope_id)).contains(&role_id));
	}

	#[benchmark]
	fn tx_create_and_set_permissions(
		i: Linear<1, 400>,
		l: Linear<2, { T::PermissionMaxLen::get() }>,
		p: Linear<1, { T::MaxPermissionsPerRole::get() }>,
	) {
		let pallet_id = generate_pallet_id_sized(0, i);
		let role_id = setup_roles_sized::<T>(
			pallet_id.clone(),
			T::MaxRolesPerPallet::get(),
			T::RoleMaxLen::get(),
		)[0];
		let permissions = generate_permissions_sized(p, l);
		#[extrinsic_call]
		_(RawOrigin::Root, pallet_id.clone(), role_id, permissions);
		assert_eq!(RBAC::<T>::permissions_by_role(pallet_id.to_id(), role_id).len() as u32, p);
	}

	#[benchmark]
	fn tx_assign_role_to_user(
		i: Linear<1, 400>,
		s: Linear<1, { T::MaxScopesPerPallet::get() }>,
		r: Linear<1, { T::MaxRolesPerUser::get() }>,
		u: Linear<1, { T::MaxUsersPerRole::get() - 1 }>,
	) {
		let user: T::AccountId = account("lastuser", 0, SEED);
		let pallet_id = generate_pallet_id_sized(0, i);
		let scope_id = setup_scopes_sized::<T>(pallet_id.clone(), s)[0];
		let role_ids = setup_roles_sized::<T>(
			pallet_id.clone(),
			T::MaxRolesPerPallet::get(),
			T::RoleMaxLen::get(),
		);
		let ru = r as usize;
		assign_roles_to_users::<T>(u, pallet_id.clone(), scope_id, &role_ids[0..ru]);
		assign_roles_to_user::<T>(
			user.clone(),
			pallet_id.clone(),
			scope_id,
			&role_ids[0..(ru - 1)],
		);
		let role_id = role_ids[ru - 1];
		#[extrinsic_call]
		_(RawOrigin::Root, user.clone(), pallet_id.clone(), scope_id, role_id);
		assert_eq!(RBAC::<T>::roles_by_user((user, pallet_id.to_id(), scope_id)).len() as u32, r);
		assert_eq!(
			RBAC::<T>::users_by_scope((pallet_id.to_id(), scope_id, role_id)).len() as u32,
			u + 1
		);
	}

	#[benchmark]
	fn revoke_permission_from_role(
		i: Linear<1, 400>,
		l: Linear<2, { T::PermissionMaxLen::get() }>,
		p: Linear<1, { T::MaxPermissionsPerRole::get() }>,
	) {
		let pallet_id = generate_pallet_id_sized(0, i);
		let role_id = setup_roles_sized::<T>(
			pallet_id.clone(),
			T::MaxRolesPerPallet::get(),
			T::RoleMaxLen::get(),
		)[0];
		let permission_id = setup_permissions_sized::<T>(pallet_id.clone(), role_id, p, l)[0];
		#[extrinsic_call]
		_(RawOrigin::Root, pallet_id.clone(), role_id, permission_id);
		assert!(
			!RBAC::<T>::permissions_by_role(pallet_id.to_id(), role_id).contains(&permission_id)
		);
	}

	impl_benchmark_test_suite! {
		RBAC,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
