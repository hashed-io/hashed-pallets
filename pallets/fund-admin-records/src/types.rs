use super::*;
use frame_support::pallet_prelude::*;
use sp_runtime::sp_std::vec::Vec;
use sp_core::crypto::KeyTypeId;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"fund");
pub const UNSIGNED_TXS_PRIORITY: u64 = 100;

pub mod crypto {
	use super::KEY_TYPE;
	use sp_core::sr25519::Signature as Sr25519Signature;
	use sp_runtime::{
		app_crypto::{app_crypto, sr25519},
		traits::Verify, MultiSignature, MultiSigner
	};
	app_crypto!(sr25519, KEY_TYPE);

	pub struct TestAuthId;

	// implemented for runtime
	impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for TestAuthId {
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}

	// implemented for mock runtime in test
	impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature>
		for TestAuthId
	{
		type RuntimeAppPublic = Public;
		type GenericSignature = sp_core::sr25519::Signature;
		type GenericPublic = sp_core::sr25519::Public;
	}
}

pub type CID = BoundedVec<u8, ConstU32<100>>;
pub type Description = BoundedVec<u8, ConstU32<400>>;
pub type Id = [u8; 32];
pub type ProjectId = [u8; 32];
pub type CreationDate = u64;
pub type UpdateDate = u64;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[codec(mel_bound())]
pub struct RecordsPayload<Public> {
	pub records_payload: Vec<RecordData>,
	pub public: Public,
}

#[derive(Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(MaxLen))]
#[codec(mel_bound())]
pub struct RecordData {
  pub cid: CID,
  pub description: Description,
  pub creation_date: CreationDate,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum Table {
  Drawdown,
  RecoveryDrawdown,
  Revenue,
  RecoveryRevenue,
}

