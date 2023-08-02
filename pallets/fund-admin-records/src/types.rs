// use super::*;
use frame_support::pallet_prelude::*;

pub type HashedInfo = BoundedVec<u8, ConstU32<400>>;
pub type Id = [u8; 32];
pub type ProjectId = BoundedVec<u8, ConstU32<50>>;
pub type CreationDate = u64;

#[derive(Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen,)]
#[scale_info(skip_type_params(MaxLen))]
#[codec(mel_bound())]
pub struct RecordData {
	pub project_id: ProjectId,
	pub hashed_info: HashedInfo,
	pub table: Table,
	pub record_type: RecordType,
	pub creation_date: CreationDate,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum Table {
  Drawdown,
  RecoveryDrawdown,
  Revenue,
  RecoveryRevenue,
}

#[derive(Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy)]
pub enum RecordType {
	Creation,
	Submit,
	Approve,
	Reject,
	Recovery,
	Reset,
}