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
	pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
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

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		/// Offchain worker entry point.
		///
		/// By implementing `fn offchain_worker` you declare a new offchain worker.
		/// This function will be called when the node is fully synced and a new best block is
		/// successfully imported.
		/// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
		/// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
		/// so the code should be able to handle that.
		fn offchain_worker(block_number: T::BlockNumber) {
			log::info!("Hello from pallet-ocw.");
			// The entry point of your code called by offchain worker
		}
		// ...
	}

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

	#[pallet::validate_unsigned]
	impl<T: Config> ValidateUnsigned for Pallet<T> {
		type Call = Call<T>;

			/// Validate unsigned call to this module.
			///
			/// By default unsigned transactions are disallowed, but implementing the validator
			/// here we make sure that some particular calls (the ones produced by offchain worker)
			/// are being whitelisted and marked as valid.
			fn validate_unsigned(source: TransactionSource, call: &Self::Call) -> TransactionValidity {
				let valid_tx = |provide| ValidTransaction::with_tag_prefix("my-pallet")
					.priority(UNSIGNED_TXS_PRIORITY) // please define `UNSIGNED_TXS_PRIORITY` before this line
					.and_provides([&provide])
					.longevity(3)
					.propagate(true)
					.build();
				// ...
				match call {
					Call::add_record { ref project_id, ref table , ref cid, ref description} => {
						if !SignedPayload::<T>::verify::<T::AuthorityId>(project_id, table, cid, description, signature.clone()) {
							return InvalidTransaction::BadProof.into();
						}
						valid_tx(b"unsigned_extrinsic_with_signed_payload".to_vec())
					},
					_ => InvalidTransaction::Call.into(),
				}
			}
	}
}