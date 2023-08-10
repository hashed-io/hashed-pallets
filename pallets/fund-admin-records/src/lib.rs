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
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::Scale;
	use frame_support::traits::Time;

	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Moment: Parameter
		+ Default
		+ Scale<Self::BlockNumber, Output = Self::Moment>
		+ Copy
		+ MaxEncodedLen
		+ scale_info::StaticTypeInfo
		+ Into<u64>;

		type Timestamp: Time<Moment = Self::Moment>;

		type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;

		#[pallet::constant]
		type MaxRecordsAtTime: Get<u32>;

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
		(ProjectId, TableType), //K1: (projectId, TableType)
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
    RecordAdded(ProjectId, TableType, RecordType, Id),
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
		/// The project id is empty
		ProjectIdIsEmpty,
		/// Hashed info is empty
		HashedInfoIsEmpty,
		/// Project id exceeded max length
		ProjectIdExceededMaxLength,
		/// Hashed info exceeded max length
		HashedInfoExceededMaxLength,
		/// Maximun number of registrations at a time reached
		MaxRegistrationsAtATimeReached,
	}

  	// E X T R I N S I C S
	// --------------------------------------------------------------------
	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Sets the signer account.
		/// 
		/// # Parameters:
		/// * `origin` - The sender of the transaction
		/// * `signer_account` - The account id of the signer
		/// Returns `Ok` if the operation is successful, `Err` otherwise.
		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(10))]
		pub fn set_signer_account(
			origin: OriginFor<T>,
			account: T::AccountId,
		) -> DispatchResult {
			T::RemoveOrigin::ensure_origin(origin)?;
			<SignerAccount<T>>::put(account);
			Ok(())
		}

		/// An extrinsic method to add new records to storage.
		///
		/// # Parameters:
		///
		/// - `origin`: The origin of the call. Must be a signed extrinsic.
		/// - `records`: The collection of records to be added. It is a vector of tuples, where each tuple represents a single record.
		///
		/// # Returns:
		///
		/// - DispatchResult: This function will return an instance of `DispatchResult`. 
		///   If the function executes successfully without any error, it will return `Ok(())`. 
    	///   If there is an error, it will return `Err(error)`, where `error` is an instance of the `DispatchError` class.
		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(10))]
		pub fn add_record(
			origin: OriginFor<T>,
			records: RecordCollection<T>,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;

			// Ensure the signer account is set
			let signer_account = SignerAccount::<T>::get().ok_or(Error::<T>::SignerAccountNotSet)?;

			// Ensure the sender is the signer account
			ensure!(who == signer_account, Error::<T>::SenderIsNotTheSignerAccount);

			Self::do_add_record(records)
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
		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_ref_time(10_000) + T::DbWeight::get().writes(10))]
		pub fn kill_storage(
			origin: OriginFor<T>,
		) -> DispatchResult{
			T::RemoveOrigin::ensure_origin(origin.clone())?;

			Ok(())
		}
  }
}