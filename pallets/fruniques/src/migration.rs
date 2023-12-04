use crate::{
	types::{CollectionId, ItemId},
	*,
};
use frame_support::{pallet_prelude::*, traits::OnRuntimeUpgrade};

#[cfg(feature = "try-runtime")]
use sp_runtime::TryRuntimeError;

/// The log target.
const TARGET: &'static str = "runtime::fruniques::migration";

pub mod v0 {
	use super::*;
	/// The actual type isn't important, as we only delete the key in the state.
	#[frame_support::storage_alias]
	pub(super) type FruniqueCnt<T: Config> = StorageValue<Pallet<T>, (), ValueQuery>;

	/// The actual type isn't important, as we only delete the key in the state.
	#[frame_support::storage_alias]
	pub(super) type FruniqueParent<T: Config> = StorageDoubleMap<
		Pallet<T>,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		ItemId, // FruniqueId
		(),     // ParentId and flag if it inherit attributes
		ValueQuery,
	>;

	/// The actual type isn't important, as we only delete the key in the state.
	#[frame_support::storage_alias]
	pub(super) type FruniqueChild<T: Config> = StorageDoubleMap<
		Pallet<T>,
		Blake2_128Concat,
		CollectionId,
		Blake2_128Concat,
		ItemId,
		(),
		ValueQuery,
	>;

	pub struct MigrateToV1<T>(sp_runtime::sp_std::marker::PhantomData<T>);

	impl<T: Config> OnRuntimeUpgrade for MigrateToV1<T> {
		fn on_runtime_upgrade() -> Weight {
			let onchain_version = Pallet::<T>::on_chain_storage_version();
			let mut writes = 0;
			if onchain_version < 1 {
				FruniqueCnt::<T>::kill();
				let result = FruniqueParent::<T>::clear(1, None); //Assuming that the storage is empty
				if result.maybe_cursor.is_some() {
					log::info!(target: TARGET, "Failed to kill FruniqueParent storage item");
				}
				let result = FruniqueChild::<T>::clear(1, None); //Assuming that the storage is empty
				if result.maybe_cursor.is_some() {
					log::info!(target: TARGET, "Failed to kill FruniqueChild storage item");
				}
				StorageVersion::new(1).put::<Pallet<T>>();
				writes = 4;
				log::info!(target: TARGET, "Migrated storage to version 1");
			} else {
				log::info!(target: TARGET, "Upgrade not run as pallet version is: {:?}", onchain_version);
			}
			T::DbWeight::get().reads_writes(writes, 1)
		}
	}
}
