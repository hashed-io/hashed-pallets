//! The confidential docs pallet provides the backend services and metadata
//! storage for the confidential docs solution

#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

// #[cfg(feature = "runtime-benchmarks")]
// mod benchmarking;

pub mod types;
mod functions;

#[frame_support::pallet]
pub mod pallet {
	//! Provides the backend services and metadata storage for the confidential docs solution
	use frame_support::{pallet_prelude::*, transactional};
	use frame_system::pallet_prelude::*;
	use crate::types::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		
		type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

		type RemoveOrigin: EnsureOrigin<Self::Origin>;

		/// Maximum number of confidential documents that a user can own
		#[pallet::constant]
		type MaxOwnedDocs: Get<u32>;
		/// Maximum number of confidential documents that can be shared to a user
		#[pallet::constant]
		type MaxSharedToDocs: Get<u32>;
		/// Minimum length for a document name
		#[pallet::constant]
		type DocNameMinLen: Get<u32>;
		/// Maximum length for a document name
		#[pallet::constant]
		type DocNameMaxLen: Get<u32>;
		/// Minimum length for a document description
		#[pallet::constant]
		type DocDescMinLen: Get<u32>;
		/// Maximum length for a document description
		#[pallet::constant]
		type DocDescMaxLen: Get<u32>;
	}

	#[pallet::pallet]
	#[pallet::generate_store(pub(super) trait Store)]
	pub struct Pallet<T>(_);

	
	#[pallet::storage]
	#[pallet::getter(fn vaults)]
	pub(super) type Vaults<T: Config> = StorageMap<
		_,
		Blake2_256,
		UserId, //user identifier
		Vault<T>,
		OptionQuery
	>;

  #[pallet::storage]
	#[pallet::getter(fn public_keys)]
	pub(super) type PublicKeys<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		PublicKey,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn owned_docs)]
	pub(super) type OwnedDocs<T: Config> = StorageMap<
		_, 
		Blake2_256,
		CID,
		OwnedDoc<T>,
		OptionQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn owned_docs_by_owner)]
	pub(super) type OwnedDocsByOwner<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		BoundedVec<CID, T::MaxOwnedDocs>,
		ValueQuery
	>;

	#[pallet::storage]
	#[pallet::getter(fn shared_docs)]
	pub(super) type SharedDocs<T: Config> = StorageMap<
		_, 
		Blake2_256,
		CID,
		SharedDoc<T>,
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn shared_docs_by_to)]
	pub(super) type SharedDocsByTo<T: Config> = StorageMap<
		_,
		Blake2_256,
		T::AccountId,
		BoundedVec<CID, T::MaxSharedToDocs>,
		ValueQuery
	>;


	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Vault stored 
		VaultStored(UserId, PublicKey, Vault<T>),
		/// Owned confidential document stored
		OwnedDocStored(OwnedDoc<T>),
		/// Shared confidential document stored
		SharedDocStored(SharedDoc<T>),
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Empty CID
		CIDNoneValue,
		/// Document Name is too short
		DocNameTooShort,
		/// Document Desc is too short
		DocDescTooShort,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// The user already has a vault
		UserAlreadyHasVault,
		/// The user already has a public key
		AccountAlreadyHasPublicKey,
		/// User is not document owner
		NotDocOwner,
		/// The document has already been shared
		DocAlreadyShared,
		/// Shared with self
		DocSharedWithSelf,
		/// Account has no public key
		AccountHasNoPublicKey,
		/// Max owned documents has been exceeded
		ExceedMaxOwnedDocs,
		/// Max documents shared with the "to" account has been exceeded
		ExceedMaxSharedToDocs,
		
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		
		/// Create a vault
		/// 
		/// Creates the calling user's vault and sets their public key
		/// .
		/// ### Parameters:
		/// - `origin`: The user that is configuring their vault
		/// - `user_id`: User identifier generated from their login method, their address if using 
		/// native login or user id if using SSO
		/// - `public key`: The users cipher public key
		/// - `cid`: The IPFS CID that contains the vaults data
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_vault(origin: OriginFor<T>, user_id: UserId, public_key: PublicKey, cid: CID) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_vault(who, user_id, public_key, cid)
		}

		/// Create/Update an owned document
		/// 
		/// Creates a new owned document or updates an existing owned document's metadata
		/// .
		/// ### Parameters:
		/// - `origin`: The user that is creating/updating an owned document
		/// - `owned_doc`: Metadata related to the owned document
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn set_owned_document(origin: OriginFor<T>, owned_doc: OwnedDoc<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_set_owned_document(who, owned_doc)
		}

		/// Share a document
		/// 
		/// Creates a shared document
		/// .
		/// ### Parameters:
		/// - `origin`: The user that is creating the shared document
		/// - `shared_doc`: Metadata related to the shared document
		#[transactional]
		#[pallet::weight(10_000 + T::DbWeight::get().writes(1))]
		pub fn share_document(origin: OriginFor<T>, shared_doc: SharedDoc<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_share_document(who, shared_doc)
		}

		/// Kill all the stored data.
		/// 
		/// This function is used to kill ALL the stored data.
		/// Use with caution!
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
			<Vaults<T>>::remove_all(None);
			<PublicKeys<T>>::remove_all(None);
			<OwnedDocs<T>>::remove_all(None);
			<OwnedDocsByOwner<T>>::remove_all(None);
			<SharedDocs<T>>::remove_all(None);
			<SharedDocsByTo<T>>::remove_all(None);
			Ok(())
		}
	}
}