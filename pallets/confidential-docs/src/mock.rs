use crate as pallet_confidential_docs;
use frame_support::{construct_runtime, parameter_types};
use frame_system as system;
use frame_system::EnsureRoot;
use sp_core::H256;
use sp_runtime::{
	traits::{BlakeTwo256, IdentityLookup},
	BuildStorage,
};
type Block = frame_system::mocking::MockBlock<Test>;

// Configure a mock runtime to test the pallet.
construct_runtime!(
	pub enum Test
	{
		System: frame_system::{Pallet, Call, Config<T>, Storage, Event<T>},
		ConfidentialDocs: pallet_confidential_docs::{Pallet, Call, Storage, Event<T>},
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
	pub const MaxOwnedDocs: u32 = 100;
	pub const MaxSharedToDocs: u32 = 100;
	pub const MaxSharedFromDocs: u32 = 100;
	pub const DocNameMinLen: u32 = 4;
	pub const DocNameMaxLen: u32 = 30;
	pub const DocDescMinLen: u32 = 5;
	pub const DocDescMaxLen: u32 = 100;
	pub const GroupNameMinLen: u32 = 3;
	pub const GroupNameMaxLen: u32 = 30;
	pub const MaxMemberGroups: u32 = 100;
}

impl pallet_confidential_docs::Config for Test {
	type RuntimeEvent = RuntimeEvent;
	type RemoveOrigin = EnsureRoot<Self::AccountId>;
	type MaxOwnedDocs = MaxOwnedDocs;
	type MaxSharedToDocs = MaxSharedToDocs;
	type MaxSharedFromDocs = MaxSharedFromDocs;
	type DocNameMinLen = DocNameMinLen;
	type DocNameMaxLen = DocNameMaxLen;
	type DocDescMinLen = DocDescMinLen;
	type DocDescMaxLen = DocDescMaxLen;
	type GroupNameMinLen = GroupNameMinLen;
	type GroupNameMaxLen = GroupNameMaxLen;
	type MaxMemberGroups = MaxMemberGroups;
}

// Build genesis storage according to the mock runtime.
pub fn new_test_ext() -> sp_io::TestExternalities {
	let storage = frame_system::GenesisConfig::<Test>::default().build_storage().unwrap();
	let mut ext: sp_io::TestExternalities = storage.into();
	ext.execute_with(|| System::set_block_number(1));
	ext
}
