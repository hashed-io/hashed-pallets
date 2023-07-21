use super::*;
use frame_support::pallet_prelude::*;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec;

pub type CID = BoundedVec<u8, ConstU32<100>>;
pub type Description = BoundedVec<u8, ConstU32<400>>;
pub type Id = [u8; 32];
pub type ProjectId = [u8; 32];
pub type CreationDate = u64;
pub type UpdateDate = u64;

#[derive(CloneNoBound, Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen)]
pub struct RecordData {
  pub cid: CID,
  pub description: Description,
  pub creation_date: CreationDate,
  pub updated_date: Option<UpdateDate>,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum Table {
  Drawdown,
  RecoveryDrawdown,
  Revenue,
  RecoveryRevenue,
}