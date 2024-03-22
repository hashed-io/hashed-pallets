
//! Autogenerated weights for `pallet_rbac`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-02-19, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `sebastian-XPS-13-9310`, CPU: `11th Gen Intel(R) Core(TM) i7-1185G7 @ 3.00GHz`
//! WASM-EXECUTION: `Compiled`, CHAIN: `Some("dev")`, DB CACHE: `1024`

// Executed Command:
// ./target/release/hashed-parachain
// benchmark
// pallet
// --wasm-execution=compiled
// --chain
// dev
// --pallet
// pallet_rbac
// --extrinsic
// tx_create_and_set_roles,tx_remove_role_from_user,tx_create_and_set_permissions,tx_assign_role_to_user,revoke_permission_from_role,remove_permission_from_pallet
// --steps
// 50
// --repeat
// 20
// --output
// ../hashed-pallets/pallets/rbac/src/weights.rs
// --template
// /home/sebastian/vsc-workspace/polkadot-sdk/substrate/.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for `pallet_rbac`.
pub trait WeightInfo {
	fn tx_create_and_set_roles(i: u32, l: u32, r: u32, ) -> Weight;
	fn tx_remove_role_from_user(i: u32, r: u32, u: u32, ) -> Weight;
	fn tx_create_and_set_permissions(i: u32, l: u32, p: u32, ) -> Weight;
	fn tx_assign_role_to_user(i: u32, s: u32, r: u32, u: u32, ) -> Weight;
	fn revoke_permission_from_role(i: u32, l: u32, p: u32, ) -> Weight;
	fn remove_permission_from_pallet(i: u32, l: u32, p: u32, m: u32, r: u32, ) -> Weight;
	fn remove_pallet_permissions(i: u32, s: u32, u: u32, ) -> Weight;
}

/// Weights for `pallet_rbac` using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `RBAC::Roles` (r:50 w:50)
	/// Proof: `RBAC::Roles` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:1 w:1)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `r` is `[1, 50]`.
	fn tx_create_and_set_roles(_i: u32, l: u32, r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5098 + r * (2558 ±0)`
		// Minimum execution time: 28_625_000 picoseconds.
		Weight::from_parts(10_614_753, 5098)
			// Standard Error: 56_444
			.saturating_add(Weight::from_parts(317_911, 0).saturating_mul(l.into()))
			// Standard Error: 56_366
			.saturating_add(Weight::from_parts(5_601_016, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2558).saturating_mul(r.into()))
	}
	/// Storage: `RBAC::RolesByUser` (r:1 w:1)
	/// Proof: `RBAC::RolesByUser` (`max_values`: None, `max_size`: Some(433), added: 2908, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::UsersByScope` (r:1 w:1)
	/// Proof: `RBAC::UsersByScope` (`max_values`: None, `max_size`: Some(16098), added: 18573, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `u` is `[1, 500]`.
	fn tx_remove_role_from_user(i: u32, r: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `782 + r * (61 ±0) + u * (33 ±0)`
		//  Estimated: `19563`
		// Minimum execution time: 41_517_000 picoseconds.
		Weight::from_parts(55_383_876, 19563)
			// Standard Error: 8_469
			.saturating_add(Weight::from_parts(10_464, 0).saturating_mul(i.into()))
			// Standard Error: 87_635
			.saturating_add(Weight::from_parts(975_224, 0).saturating_mul(r.into()))
			// Standard Error: 1_682
			.saturating_add(Weight::from_parts(82_973, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `RBAC::PalletRoles` (r:1 w:0)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::Permissions` (r:100 w:100)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:1 w:1)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `p` is `[1, 100]`.
	fn tx_create_and_set_permissions(_i: u32, l: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1737`
		//  Estimated: `6731 + p * (2590 ±0)`
		// Minimum execution time: 42_695_000 picoseconds.
		Weight::from_parts(43_151_000, 6731)
			// Standard Error: 92_695
			.saturating_add(Weight::from_parts(283_746, 0).saturating_mul(l.into()))
			// Standard Error: 46_054
			.saturating_add(Weight::from_parts(6_188_777, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 2590).saturating_mul(p.into()))
	}
	/// Storage: `RBAC::Scopes` (r:1 w:0)
	/// Proof: `RBAC::Scopes` (`max_values`: None, `max_size`: Some(3234), added: 5709, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:1 w:0)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::RolesByUser` (r:1 w:1)
	/// Proof: `RBAC::RolesByUser` (`max_values`: None, `max_size`: Some(433), added: 2908, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::UsersByScope` (r:1 w:1)
	/// Proof: `RBAC::UsersByScope` (`max_values`: None, `max_size`: Some(16098), added: 18573, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `u` is `[1, 499]`.
	fn tx_assign_role_to_user(i: u32, s: u32, r: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2552 + r * (62 ±0) + s * (32 ±0) + u * (33 ±0)`
		//  Estimated: `19563`
		// Minimum execution time: 54_017_000 picoseconds.
		Weight::from_parts(58_811_667, 19563)
			// Standard Error: 9_859
			.saturating_add(Weight::from_parts(49_726, 0).saturating_mul(i.into()))
			// Standard Error: 9_859
			.saturating_add(Weight::from_parts(56_844, 0).saturating_mul(s.into()))
			// Standard Error: 101_871
			.saturating_add(Weight::from_parts(911_702, 0).saturating_mul(r.into()))
			// Standard Error: 1_963
			.saturating_add(Weight::from_parts(100_764, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `RBAC::Permissions` (r:1 w:0)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:1 w:0)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:1 w:1)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `p` is `[1, 100]`.
	fn revoke_permission_from_role(_i: u32, l: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2286 + p * (35 ±0)`
		//  Estimated: `6731`
		// Minimum execution time: 41_252_000 picoseconds.
		Weight::from_parts(49_544_932, 6731)
			// Standard Error: 14_429
			.saturating_add(Weight::from_parts(54_049, 0).saturating_mul(l.into()))
			// Standard Error: 7_113
			.saturating_add(Weight::from_parts(63_399, 0).saturating_mul(p.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `RBAC::Permissions` (r:1 w:1)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:51 w:50)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[2, 50]`.
	/// The range of component `r` is `[1, 50]`.
	fn remove_permission_from_pallet(_i: u32, _l: u32, p: u32, _m: u32, r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + p * (1604 ±0) + r * (3241 ±0)`
		//  Estimated: `6731 + r * (5741 ±0)`
		// Minimum execution time: 53_922_000 picoseconds.
		Weight::from_parts(54_679_000, 6731)
			// Standard Error: 37_688
			.saturating_add(Weight::from_parts(2_071_084, 0).saturating_mul(p.into()))
			// Standard Error: 76_195
			.saturating_add(Weight::from_parts(13_534_746, 0).saturating_mul(r.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 5741).saturating_mul(r.into()))
	}

	/// Storage: `RBAC::Scopes` (r:1 w:1)
	/// Proof: `RBAC::Scopes` (`max_values`: None, `max_size`: Some(3234), added: 5709, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::UsersByScope` (r:1100 w:1000)
	/// Proof: `RBAC::UsersByScope` (`max_values`: None, `max_size`: Some(16098), added: 18573, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:2 w:1)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:50 w:50)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::Permissions` (r:100 w:100)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::Roles` (r:0 w:50)
	/// Proof: `RBAC::Roles` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::RolesByUser` (r:0 w:50000)
	/// Proof: `RBAC::RolesByUser` (`max_values`: None, `max_size`: Some(433), added: 2908, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `u` is `[1, 500]`.
	fn remove_pallet_permissions(_i: u32, s: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + s * (160490 ±0) + u * (32000 ±0)`
		//  Estimated: `288040 + s * (204303 ±0)`
		// Minimum execution time: 4_292_658_000 picoseconds.
		Weight::from_parts(4_366_435_000, 288040)
			// Standard Error: 221_159_668
			.saturating_add(Weight::from_parts(3_973_731_500, 0).saturating_mul(s.into()))
			// Standard Error: 44_058_618
			.saturating_add(Weight::from_parts(767_998_348, 0).saturating_mul(u.into()))
			.saturating_add(T::DbWeight::get().reads(153_u64))
			.saturating_add(T::DbWeight::get().reads((11_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes(712_u64))
			.saturating_add(T::DbWeight::get().writes((311_u64).saturating_mul(s.into())))
			.saturating_add(T::DbWeight::get().writes((61_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 204303).saturating_mul(s.into()))
	}
}

// For backwards compatibility and tests.
impl WeightInfo for () {
	/// Storage: `RBAC::Roles` (r:50 w:50)
	/// Proof: `RBAC::Roles` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:1 w:1)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `r` is `[1, 50]`.
	fn tx_create_and_set_roles(_i: u32, l: u32, r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `5098 + r * (2558 ±0)`
		// Minimum execution time: 28_625_000 picoseconds.
		Weight::from_parts(10_614_753, 5098)
			// Standard Error: 56_444
			.saturating_add(Weight::from_parts(317_911, 0).saturating_mul(l.into()))
			// Standard Error: 56_366
			.saturating_add(Weight::from_parts(5_601_016, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 2558).saturating_mul(r.into()))
	}
	/// Storage: `RBAC::RolesByUser` (r:1 w:1)
	/// Proof: `RBAC::RolesByUser` (`max_values`: None, `max_size`: Some(433), added: 2908, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::UsersByScope` (r:1 w:1)
	/// Proof: `RBAC::UsersByScope` (`max_values`: None, `max_size`: Some(16098), added: 18573, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `u` is `[1, 500]`.
	fn tx_remove_role_from_user(i: u32, r: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `782 + r * (61 ±0) + u * (33 ±0)`
		//  Estimated: `19563`
		// Minimum execution time: 41_517_000 picoseconds.
		Weight::from_parts(55_383_876, 19563)
			// Standard Error: 8_469
			.saturating_add(Weight::from_parts(10_464, 0).saturating_mul(i.into()))
			// Standard Error: 87_635
			.saturating_add(Weight::from_parts(975_224, 0).saturating_mul(r.into()))
			// Standard Error: 1_682
			.saturating_add(Weight::from_parts(82_973, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `RBAC::PalletRoles` (r:1 w:0)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::Permissions` (r:100 w:100)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:1 w:1)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `p` is `[1, 100]`.
	fn tx_create_and_set_permissions(_i: u32, l: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1737`
		//  Estimated: `6731 + p * (2590 ±0)`
		// Minimum execution time: 42_695_000 picoseconds.
		Weight::from_parts(43_151_000, 6731)
			// Standard Error: 92_695
			.saturating_add(Weight::from_parts(283_746, 0).saturating_mul(l.into()))
			// Standard Error: 46_054
			.saturating_add(Weight::from_parts(6_188_777, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(p.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(p.into())))
			.saturating_add(Weight::from_parts(0, 2590).saturating_mul(p.into()))
	}
	/// Storage: `RBAC::Scopes` (r:1 w:0)
	/// Proof: `RBAC::Scopes` (`max_values`: None, `max_size`: Some(3234), added: 5709, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:1 w:0)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::RolesByUser` (r:1 w:1)
	/// Proof: `RBAC::RolesByUser` (`max_values`: None, `max_size`: Some(433), added: 2908, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::UsersByScope` (r:1 w:1)
	/// Proof: `RBAC::UsersByScope` (`max_values`: None, `max_size`: Some(16098), added: 18573, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `r` is `[1, 10]`.
	/// The range of component `u` is `[1, 499]`.
	fn tx_assign_role_to_user(i: u32, s: u32, r: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2552 + r * (62 ±0) + s * (32 ±0) + u * (33 ±0)`
		//  Estimated: `19563`
		// Minimum execution time: 54_017_000 picoseconds.
		Weight::from_parts(58_811_667, 19563)
			// Standard Error: 9_859
			.saturating_add(Weight::from_parts(49_726, 0).saturating_mul(i.into()))
			// Standard Error: 9_859
			.saturating_add(Weight::from_parts(56_844, 0).saturating_mul(s.into()))
			// Standard Error: 101_871
			.saturating_add(Weight::from_parts(911_702, 0).saturating_mul(r.into()))
			// Standard Error: 1_963
			.saturating_add(Weight::from_parts(100_764, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `RBAC::Permissions` (r:1 w:0)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:1 w:0)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:1 w:1)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `p` is `[1, 100]`.
	fn revoke_permission_from_role(_i: u32, l: u32, p: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2286 + p * (35 ±0)`
		//  Estimated: `6731`
		// Minimum execution time: 41_252_000 picoseconds.
		Weight::from_parts(49_544_932, 6731)
			// Standard Error: 14_429
			.saturating_add(Weight::from_parts(54_049, 0).saturating_mul(l.into()))
			// Standard Error: 7_113
			.saturating_add(Weight::from_parts(63_399, 0).saturating_mul(p.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `RBAC::Permissions` (r:1 w:1)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:51 w:50)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `l` is `[2, 50]`.
	/// The range of component `p` is `[1, 100]`.
	/// The range of component `m` is `[2, 50]`.
	/// The range of component `r` is `[1, 50]`.
	fn remove_permission_from_pallet(_i: u32, _l: u32, p: u32, _m: u32, r: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + p * (1604 ±0) + r * (3241 ±0)`
		//  Estimated: `6731 + r * (5741 ±0)`
		// Minimum execution time: 53_922_000 picoseconds.
		Weight::from_parts(54_679_000, 6731)
			// Standard Error: 37_688
			.saturating_add(Weight::from_parts(2_071_084, 0).saturating_mul(p.into()))
			// Standard Error: 76_195
			.saturating_add(Weight::from_parts(13_534_746, 0).saturating_mul(r.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(r.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((1_u64).saturating_mul(r.into())))
			.saturating_add(Weight::from_parts(0, 5741).saturating_mul(r.into()))
	}

	/// Storage: `RBAC::Scopes` (r:1 w:1)
	/// Proof: `RBAC::Scopes` (`max_values`: None, `max_size`: Some(3234), added: 5709, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::UsersByScope` (r:1100 w:1000)
	/// Proof: `RBAC::UsersByScope` (`max_values`: None, `max_size`: Some(16098), added: 18573, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PalletRoles` (r:2 w:1)
	/// Proof: `RBAC::PalletRoles` (`max_values`: None, `max_size`: Some(1633), added: 4108, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::PermissionsByRole` (r:50 w:50)
	/// Proof: `RBAC::PermissionsByRole` (`max_values`: None, `max_size`: Some(3266), added: 5741, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::Permissions` (r:100 w:100)
	/// Proof: `RBAC::Permissions` (`max_values`: None, `max_size`: Some(115), added: 2590, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::Roles` (r:0 w:50)
	/// Proof: `RBAC::Roles` (`max_values`: None, `max_size`: Some(83), added: 2558, mode: `MaxEncodedLen`)
	/// Storage: `RBAC::RolesByUser` (r:0 w:50000)
	/// Proof: `RBAC::RolesByUser` (`max_values`: None, `max_size`: Some(433), added: 2908, mode: `MaxEncodedLen`)
	/// The range of component `i` is `[1, 100]`.
	/// The range of component `s` is `[1, 100]`.
	/// The range of component `u` is `[1, 500]`.
	fn remove_pallet_permissions(_i: u32, s: u32, u: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0 + s * (160490 ±0) + u * (32000 ±0)`
		//  Estimated: `288040 + s * (204303 ±0)`
		// Minimum execution time: 4_292_658_000 picoseconds.
		Weight::from_parts(4_366_435_000, 288040)
			// Standard Error: 221_159_668
			.saturating_add(Weight::from_parts(3_973_731_500, 0).saturating_mul(s.into()))
			// Standard Error: 44_058_618
			.saturating_add(Weight::from_parts(767_998_348, 0).saturating_mul(u.into()))
			.saturating_add(RocksDbWeight::get().reads(153_u64))
			.saturating_add(RocksDbWeight::get().reads((11_u64).saturating_mul(s.into())))
			.saturating_add(RocksDbWeight::get().writes(712_u64))
			.saturating_add(RocksDbWeight::get().writes((311_u64).saturating_mul(s.into())))
			.saturating_add(RocksDbWeight::get().writes((61_u64).saturating_mul(u.into())))
			.saturating_add(Weight::from_parts(0, 204303).saturating_mul(s.into()))
	}
}