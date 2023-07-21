#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{pallet_prelude::{*, ValueQuery}, BoundedVec};
	use frame_system::pallet_prelude::*;
	use frame_support::transactional;
	use sp_runtime::traits::Scale;
	use frame_support::traits::{Time};

	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type Moment: Parameter
		+ Default
		+ Scale<Self::BlockNumber, Output = Self::Moment>
		+ Copy
		+ MaxEncodedLen
		+ scale_info::StaticTypeInfo
		+ Into<u64>;

		type Timestamp: Time<Moment = Self::Moment>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;

	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	/*--- Onchain storage section ---*/
	#[pallet::storage]
	#[pallet::getter(fn records)]
	pub(super) type Records<T: Config> = StorageDoubleMap<
		_,
		Identity,
		(ProjectId, Table), //K1: record id
		Identity,
		Id, //K2: (projectId, Table)
		RecordData, // Value transactions
		OptionQuery,
	>;

  // E V E N T S
	// --------------------------------------------------------------------
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    /// A record was added
    RecordAdded(Id),
	}

	// E R R O R S
	// --------------------------------------------------------------------
	#[pallet::error]
	pub enum Error<T> {
    /// The record id already exists
    IdAlreadyExists,
    /// Timestamp was not genereated correctly
		TimestampError,
  }

  // E X T R I N S I C S
	// --------------------------------------------------------------------
	#[pallet::call]
	impl<T: Config> Pallet<T> {
    #[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
    pub fn add_record(
      origin: OriginFor<T>,
      project_id: ProjectId,
      table: Table,
      cid: CID,
      description: Description,
    ) -> DispatchResult {
      let _who = ensure_signed(origin)?;

      Self::do_add_record(project_id, table, cid, description)
    }

    /// Kill all the stored data.
		///
		/// This function is used to kill ALL the stored data.
		/// Use it with caution!
		///
		/// ### Parameters:
		/// - `origin`: The user who performs the action.
		///
		/// ### Considerations:
		/// - This function is only available to the `admin` with sudo access.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn kill_storage(
			origin: OriginFor<T>,
		) -> DispatchResult{
			T::RemoveOrigin::ensure_origin(origin.clone())?;

			Ok(())
		}
  }
//
}