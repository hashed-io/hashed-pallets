#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use frame_support::transactional;
	use sp_runtime::traits::Scale;
	use frame_support::traits::Time;

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
	#[pallet::getter(fn signer_account)]
	pub(super) type SignerAccount<T: Config> = StorageValue<
		_,
		T::AccountId,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn records)]
	pub(super) type Records<T: Config> = StorageDoubleMap<
		_,
		Identity,
		(ProjectId, Table), //K1: (projectId, Table)
		Identity,
		Id, //K2: record id 
		RecordData, // Value record data
		OptionQuery,
	>;

  // E V E N T S
	// --------------------------------------------------------------------
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
    /// A record was added
    RecordAdded(ProjectId, Table, RecordType, Id),
	}

	// E R R O R S
	// --------------------------------------------------------------------
	#[pallet::error]
	pub enum Error<T> {
    /// The record id already exists
    IdAlreadyExists,
    /// Timestamp was not genereated correctly
		TimestampError,
		/// Signer account is not set
		SignerAccountNotSet,
		/// The sender is not the signer account
		SenderIsNotTheSignerAccount,
  }

  // E X T R I N S I C S
	// --------------------------------------------------------------------
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the signer account.
		/// 
		/// # Arguments
		/// * `origin` - The sender of the transaction
		/// * `signer_account` - The account id of the signer
		/// Returns `Ok` if the operation is successful, `Err` otherwise.
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_signer_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin)?;
			<SignerAccount<T>>::put(account);
			Ok(())
		}

		/// Extrinsics to add a record
		/// 
		/// Meant to be unsigned with a signed payload and used by the offchain worker
		/// 
    #[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))] 		// TODO: test fees
    pub fn add_record(
      origin: OriginFor<T>,
			project_id: ProjectId,
			hashed_info: HashedInfo,
			table: Table,
			record_type: RecordType,
    ) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			// Ensure the signer account is set
			let signer_account = SignerAccount::<T>::get().ok_or(Error::<T>::SignerAccountNotSet)?;

			// Ensure the sender is the signer account
			ensure!(who == signer_account, Error::<T>::SenderIsNotTheSignerAccount);

			Self::do_add_record(
				project_id,
				hashed_info,
				table,
				record_type,
			)
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
}