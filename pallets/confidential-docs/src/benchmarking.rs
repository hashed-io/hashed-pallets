//! Confidential Docs pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{
	test_util::*,
	types::{Vault, *},
	Pallet as ConfidentialDocs,
};

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, traits::Get};
use frame_system::RawOrigin;

const SEED: u32 = 0;

pub fn setup_vault_sized<T: Config>(owner: T::AccountId, id: u8, size: u32) {
	let (user_id, public_key, cid) = generate_vault_sized(id, size);
	assert_ok!(ConfidentialDocs::<T>::set_vault(
		RawOrigin::Signed(owner.clone()).into(),
		user_id,
		public_key,
		cid.clone(),
	));
}

pub fn setup_owned_doc_sized<T: Config>(
	id: u8,
	owner: T::AccountId,
	cid_size: u32,
	name_size: u32,
	desc_size: u32,
) -> OwnedDoc<T> {
	let doc = generate_owned_doc_sized(id, owner.clone(), cid_size, name_size, desc_size);
	assert_ok!(ConfidentialDocs::<T>::set_owned_document(
		RawOrigin::Signed(owner.clone()).into(),
		doc.clone()
	));
	doc
}

pub fn setup_shared_doc_sized<T: Config>(
	id: u8,
	from: T::AccountId,
	to: T::AccountId,
	cid_size: u32,
	name_size: u32,
	desc_size: u32,
) -> SharedDoc<T> {
	let doc =
		generate_shared_doc_sized(id, from.clone(), to.clone(), cid_size, name_size, desc_size);
	assert_ok!(ConfidentialDocs::<T>::share_document(
		RawOrigin::Signed(from.clone()).into(),
		doc.clone()
	));
	doc
}

pub fn setup_group_sized<T: Config>(
	id: u8,
	creator: T::AccountId,
	group_id: T::AccountId,
	cid_size: u32,
	name_size: u32,
) {
	let group_name = generate_group_name_sized::<T>(id, name_size);
	let public_key = generate_public_key(id);
	let cid = generate_cid_sized(id, cid_size);
	assert_ok!(ConfidentialDocs::<T>::create_group(
		RawOrigin::Signed(creator.clone()).into(),
		group_id.clone(),
		group_name.clone(),
		public_key,
		cid.clone(),
	));
}

pub fn setup_group_member_sized<T: Config>(
	id: u8,
	authorizer: T::AccountId,
	group_id: T::AccountId,
	member: T::AccountId,
	cid_size: u32,
) -> GroupMember<T> {
	let group_member = GroupMember {
		authorizer: authorizer.clone(),
		cid: generate_cid_sized(id, cid_size),
		group: group_id,
		member,
		role: GroupRole::Admin,
	};
	assert_ok!(ConfidentialDocs::<T>::add_group_member(
		RawOrigin::Signed(authorizer).into(),
		group_member.clone()
	));
	group_member
}
#[benchmarks(where T: Config)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_vault(c: Linear<2, { CIDSize::get() }>) {
		let owner: T::AccountId = account("owner", 0, SEED);
		let (user_id, public_key, cid) = generate_vault_sized(1, c);
		let vault = Vault::<T> { cid: cid.clone(), owner: owner.clone() };

		#[extrinsic_call]
		_(RawOrigin::Signed(owner), user_id, public_key, cid.clone());
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(vault.clone()));
	}

	#[benchmark]
	fn set_owned_document(
		c: Linear<2, { CIDSize::get() }>,
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		o: Linear<1, { T::MaxOwnedDocs::get() - 1 }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_vault_sized::<T>(owner.clone(), 1, 5);
		// Add "o" docs to owned docs
		for i in 1..=o {
			setup_owned_doc_sized::<T>(i as u8, owner.clone(), c, n, d);
		}
		let doc = generate_owned_doc_sized((o + 1) as u8, owner.clone(), c, n, d);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner), doc.clone());
		assert_eq!(ConfidentialDocs::owned_docs(&doc.cid), Some(doc.clone()));
	}

	#[benchmark]
	fn remove_owned_document(
		c: Linear<2, { CIDSize::get() }>,
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		o: Linear<1, { T::MaxOwnedDocs::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_vault_sized::<T>(owner.clone(), 1, 5);
		// Add "o" docs to owned docs
		for i in 1..=o {
			setup_owned_doc_sized::<T>(i as u8, owner.clone(), c, n, d);
		}
		let cid = generate_cid_sized(o as u8, c);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner), cid.clone());
		assert_eq!(ConfidentialDocs::<T>::owned_docs(cid), None);
	}

	#[benchmark]
	fn share_document(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		s: Linear<1, { T::MaxSharedFromDocs::get() - 1 }>,
	) {
		let from: T::AccountId = account("from", 0, SEED);
		let to: T::AccountId = account("to", 0, SEED);
		// Setup from vault
		setup_vault_sized::<T>(from.clone(), 1, 5);
		// Setup to vault
		setup_vault_sized::<T>(to.clone(), 2, 5);
		// Add "s" docs to shared docs
		for i in 1..=s {
			setup_shared_doc_sized::<T>(i as u8, from.clone(), to.clone(), c, n, d);
		}
		let doc = generate_shared_doc_sized((s + 1) as u8, from.clone(), to.clone(), c, n, d);
		#[extrinsic_call]
		_(RawOrigin::Signed(from), doc.clone());
		assert_eq!(ConfidentialDocs::shared_docs(&doc.cid), Some(doc.clone()));
	}

	#[benchmark]
	fn update_shared_document_metadata(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
	) {
		let from: T::AccountId = account("from", 0, SEED);
		let to: T::AccountId = account("to", 0, SEED);
		// Setup from vault
		setup_vault_sized::<T>(from.clone(), 1, 5);
		// Setup to vault
		setup_vault_sized::<T>(to.clone(), 2, 5);
		// Create doc to update
		let mut doc = setup_shared_doc_sized::<T>(1, from.clone(), to.clone(), c, n, d);
		doc.name = generate_doc_name_sized::<T>(2, n);
		doc.description = generate_doc_desc_sized::<T>(2, d);
		#[extrinsic_call]
		_(RawOrigin::Signed(to), doc.clone());
		assert_eq!(ConfidentialDocs::shared_docs(&doc.cid), Some(doc.clone()));
	}

	#[benchmark]
	fn remove_shared_document(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		s: Linear<1, { T::MaxSharedFromDocs::get() }>,
	) {
		let from: T::AccountId = account("from", 0, SEED);
		let to: T::AccountId = account("to", 0, SEED);
		// Setup from vault
		setup_vault_sized::<T>(from.clone(), 1, 5);
		// Setup to vault
		setup_vault_sized::<T>(to.clone(), 2, 5);
		// Add "s" docs to shared docs
		for i in 1..=s {
			setup_shared_doc_sized::<T>(i as u8, from.clone(), to.clone(), c, n, d);
		}
		let cid = generate_cid_sized(s as u8, c);
		#[extrinsic_call]
		_(RawOrigin::Signed(to), cid.clone());
		assert_eq!(ConfidentialDocs::<T>::shared_docs(&cid), None);
	}

	#[benchmark]
	fn create_group(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::GroupNameMinLen::get() }, { T::GroupNameMaxLen::get() }>,
	) {
		let creator: T::AccountId = account("creator", 0, SEED);
		// Setup creator vault
		setup_vault_sized::<T>(creator.clone(), 1, 5);
		let group_name = generate_group_name_sized::<T>(1, n);
		let group_id: T::AccountId = account("group", 0, SEED);
		let public_key = generate_public_key(2);
		let cid = generate_cid_sized(2, c);
		#[extrinsic_call]
		_(
			RawOrigin::Signed(creator.clone()),
			group_id.clone(),
			group_name.clone(),
			public_key,
			cid.clone(),
		);
		let group = Group { group: group_id.clone(), creator, name: group_name };
		assert_eq!(ConfidentialDocs::<T>::groups(group_id), Some(group.clone()));
	}

	#[benchmark]
	fn add_group_member(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::GroupNameMinLen::get() }, { T::GroupNameMaxLen::get() }>,
		s: Linear<1, { T::MaxMemberGroups::get() - 1 }>,
	) {
		let creator: T::AccountId = account("creator", 0, SEED);
		// Setup creator vault
		setup_vault_sized::<T>(creator.clone(), 0, 5);
		let group_id: T::AccountId = account("group", 0, SEED);
		setup_group_sized::<T>(2, creator.clone(), group_id.clone(), c, n);
		// Add "s" members to group
		for i in 1..=s {
			// Setup member vault
			let member: T::AccountId = account("member", i, SEED);
			setup_vault_sized::<T>(member.clone(), i as u8, 5);
			setup_group_member_sized::<T>(
				i as u8,
				creator.clone(),
				group_id.clone(),
				member.clone(),
				c,
			);
		}
		// Setup member vault
		let member: T::AccountId = account("member", s + 1, SEED);
		setup_vault_sized::<T>(member.clone(), (s + 1) as u8, 5);
		let group_member = GroupMember {
			authorizer: creator.clone(),
			cid: generate_cid_sized((s + 1) as u8, c),
			group: group_id.clone(),
			member,
			role: GroupRole::Admin,
		};
		#[extrinsic_call]
		_(RawOrigin::Signed(creator.clone()), group_member.clone());
		assert_eq!(
			ConfidentialDocs::<T>::group_members(group_id, group_member.member.clone()),
			Some(group_member.clone())
		);
	}

	#[benchmark]
	fn remove_group_member(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::GroupNameMinLen::get() }, { T::GroupNameMaxLen::get() }>,
		s: Linear<1, { T::MaxMemberGroups::get() }>,
	) {
		let creator: T::AccountId = account("creator", 0, SEED);
		// Setup creator vault
		setup_vault_sized::<T>(creator.clone(), 0, 5);
		let group_id: T::AccountId = account("group", 0, SEED);
		setup_group_sized::<T>(2, creator.clone(), group_id.clone(), c, n);
		// Add "s" members to group
		for i in 1..=s {
			// Setup member vault
			let member: T::AccountId = account("member", i, SEED);
			setup_vault_sized::<T>(member.clone(), i as u8, 5);
			setup_group_member_sized::<T>(
				i as u8,
				creator.clone(),
				group_id.clone(),
				member.clone(),
				c,
			);
		}
		// Setup member vault
		let member: T::AccountId = account("member", s, SEED);
		#[extrinsic_call]
		_(RawOrigin::Signed(creator.clone()), group_id.clone(), member.clone());
		assert_eq!(ConfidentialDocs::<T>::group_members(group_id, member), None);
	}

	impl_benchmark_test_suite! {
		ConfidentialDocs,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
