use super::*;
use frame_support::pallet_prelude::*;

pub type HashedInfo = BoundedVec<u8, ConstU32<400>>;
pub type Id = [u8; 32];
pub type ProjectId = BoundedVec<u8, ConstU32<70>>;
pub type CreationDate = u64;
pub type RecordCollection<T> =
	BoundedVec<(ProjectId, HashedInfo, TableType, RecordType), <T as Config>::MaxRecordsAtTime>;

#[derive(Encode, Decode, RuntimeDebugNoBound, TypeInfo, MaxEncodedLen)]
#[scale_info(skip_type_params(MaxLen))]
#[codec(mel_bound())]
pub struct RecordData {
	pub project_id: ProjectId,
	pub hashed_info: HashedInfo,
	pub table_type: TableType,
	pub record_type: RecordType,
	pub creation_date: CreationDate,
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum TableType {
	Drawdown,
	RecoveryDrawdown,
	Revenue,
	RecoveryRevenue,
	Rebalance,
	DrawdownSources
}

#[derive(
	Encode, Decode, Clone, Eq, PartialEq, RuntimeDebugNoBound, MaxEncodedLen, TypeInfo, Copy,
)]
pub enum RecordType {
	Creation,
	Submit,
	Approve,
	Reject,
	Recovery,
	Cancel,
	Confirm
}
