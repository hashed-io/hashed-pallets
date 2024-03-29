use crate as pallet_rbac;
use frame_support::parameter_types;
use frame_system as system;
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
frame_support::construct_runtime!(
  pub enum Test
  {
	System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
	RBAC: pallet_rbac::{Pallet, Call, Storage, Event<T>},
  }
);

parameter_types! {
  pub const BlockHashCount: u64 = 250;
  pub const SS58Prefix: u8 = 42;
}

impl system::Config for Test {
	type BaseCallFilter = frame_support::traits::Everything;
	type BlockWeights = ();
	type BlockLength = ();
	type DbWeight = ();
	type RuntimeOrigin = RuntimeOrigin;
	type RuntimeCall = RuntimeCall;
	type Nonce = u64;
	type Hash = H256;
	type Hashing = BlakeTwo256;
	type AccountId = u64;
	type Lookup = IdentityLookup<Self::AccountId>;
	type Block = Block;
	type RuntimeEvent = RuntimeEvent;
	type BlockHashCount = BlockHashCount;
	type Version = ();
	type PalletInfo = PalletInfo;
	type AccountData = ();
	type OnNewAccount = ();
	type OnKilledAccount = ();
	type SystemWeightInfo = ();
	type SS58Prefix = SS58Prefix;
	type OnSetCode = ();
	type MaxConsumers = frame_support::traits::ConstU32<16>;
}

parameter_types! {
  pub const MaxScopesPerPallet: u32 = 2;
  pub const MaxRolesPerPallet: u32 = 3;
  pub const RoleMaxLen: u32 = 10;
  pub const PermissionMaxLen: u32 = 15;
  pub const MaxPermissionsPerRole: u32 = 3;
  pub const MaxRolesPerUser: u32 = 2;
  pub const MaxUsersPerRole: u32 = 2;
}
impl pallet_rbac::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type MaxScopesPerPallet = MaxScopesPerPallet;
	type MaxRolesPerPallet = MaxRolesPerPallet;
	type RoleMaxLen = RoleMaxLen;
	type PermissionMaxLen = PermissionMaxLen;
	type MaxPermissionsPerRole = MaxPermissionsPerRole;
	type MaxRolesPerUser = MaxRolesPerUser;
	type MaxUsersPerRole = MaxUsersPerRole;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
}
// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	system::GenesisConfig::<Test>::default().build_storage().unwrap().into()
}
