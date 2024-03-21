#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

mod functions;
pub mod types;

#[frame_support::pallet]
pub mod pallet {
	use frame_support::{
		pallet_prelude::*,
		traits::{Currency, Time},
	};
	use frame_system::{pallet_prelude::*, RawOrigin};
	use pallet_fruniques::types::{Attributes, CollectionDescription, FruniqueRole, ParentInfo};
	use pallet_gated_marketplace::types::*;
	use sp_runtime::traits::Scale;
	const STORAGE_VERSION: StorageVersion = StorageVersion::new(0);

	use crate::types::*;
	use pallet_rbac::types::RoleBasedAccessControl;
	pub type BalanceOf<T> = <<T as pallet_uniques::Config>::Currency as Currency<
		<T as frame_system::Config>::AccountId,
	>>::Balance;

	/// Configure the pallet by specifying the parameters and types on which it depends.
	#[pallet::config]
	pub trait Config:
		frame_system::Config
		+ pallet_gated_marketplace::Config
		+ pallet_mapped_assets::Config
		+ pallet_uniques::Config
	{
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		type Rbac: RoleBasedAccessControl<Self::AccountId>;
		// type RemoveOrigin: EnsureOrigin<Self::RuntimeOrigin>;
		type Currency: Currency<Self::AccountId>;
		type ItemId: Parameter + Member + Default;
	}

	#[pallet::pallet]
	#[pallet::storage_version(STORAGE_VERSION)]

	pub struct Pallet<T>(_);

	// Pallets use events to inform users when important changes are made.
	// https://docs.substrate.io/v3/runtime/events-and-errors
	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Timestamp was not generated correctly
		TimestampError,
		// New user created
		NewUser(T::AccountId),
		// User edited
		UserEdited(T::AccountId),
		// User deleted
		UserDeleted(T::AccountId),
		// A sell created taken by an user
		SellOrderCreated(T::AccountId),
		// A buy created taken by an user
		BuyOrderCreated(T::AccountId),
		// A ell order taken by an user
		SellOrderTaken(T::AccountId),
		// A buy order taken by an user
		BuyOrderTaken(T::AccountId),
		// Updated balance to an account (who updated the balance, the account, the balance)
		AfloatBalanceSet(T::AccountId, T::AccountId, T::Balance),
		// A new admin is added (who added the admin, the admin)
		AdminAdded(T::AccountId, T::AccountId),
		// A user is assigned to a role (who assigned the role, the user, the role)
		UserAssignedToRoleAdded(T::AccountId, T::AccountId, AfloatRole),
	}

	// Errors inform users that something went wrong.
	#[pallet::error]
	pub enum Error<T> {
		/// Error names should be descriptive.
		NoneValue,
		/// Errors should have helpful documentation associated with them.
		StorageOverflow,
		/// Marketplace not initialized
		MarketplaceNotInitialized,
		// Marketplace id not found
		MarketPlaceIdNotFound,
		//Asset id not found
		AssetNotFound,
		// Collection id not found
		CollectionIdNotFound,
		/// User not found
		UserNotFound,
		/// User already exists
		UserAlreadyExists,
		/// Failed to edit user account
		FailedToEditUserAccount,
		// Failed to create fruniques collection
		FailedToCreateFruniquesCollection,
		// Failed to remove Fruniques role
		FailedToRemoveFruniquesRole,
		// User is not authorized to perform this action
		Unauthorized,
		// Pallet has not ben initialized yet
		NotInitialized,
		// Failed to remove afloat role
		FailedToRemoveAfloatRole,
		// Maximum number of transactions per offer reached
		MaxTransactionsReached,
		// Offer not found
		OfferNotFound,
		// Offer's type is not correct
		WrongOfferType,
		// Offer has expired
		OfferExpired,
		// Offer has been cancelled
		OfferCancelled,
		// Offer has been taken already
		OfferTaken,
		// Transaction not found
		TransactionNotFound,
		// Transaction has expired
		TransactionExpired,
		// Transaction has been cancelled
		TransactionCancelled,
		// Transaction has not been confirmed yet
		TransactionNotConfirmed,
		// Transaction already confirmed by buyer
		TransactionAlreadyConfirmedByBuyer,
		// Transaction already confirmed by seller
		TransactionAlreadyConfirmedBySeller,
		// Transaction not confirmed by buyer
		TransactionNotConfirmedByBuyer,
		// Not enough tax credits available for sale
		NotEnoughTaxCreditsAvailable,
		// Not enough afloat balance available
		NotEnoughAfloatBalanceAvailable,
		// Tax credit amount overflow
		TaxCreditAmountOverflow,
		// Child offer id not found
		ChildOfferIdNotFound,
		// Tax credit amount underflow
		Underflow,
		// Arithmetic multiplication overflow
		ArithmeticOverflow,
		// Afloat marketplace label too long
		LabelTooLong,
		// Afloat asset has not been set
		AfloatAssetNotSet,
	}

	#[pallet::storage]
	#[pallet::getter(fn user_info)]
	/// Keeps track of the number of fruniques in existence for a collection.
	pub(super) type UserInfo<T: Config> = StorageMap<
		_,
		Blake2_128Concat,
		T::AccountId,
		User<T>, // User<T> is a struct that contains all the user info
		OptionQuery,
	>;

	#[pallet::storage]
	#[pallet::getter(fn marketplace_id)]
	pub(super) type AfloatMarketPlaceId<T: Config> = StorageValue<
		_,
		MarketplaceId, // Afloat's marketplace id
	>;

	#[pallet::storage]
	#[pallet::getter(fn collection_id)]
	pub(super) type AfloatCollectionId<T: Config> = StorageValue<
		_,
		<T as pallet_uniques::Config>::CollectionId, // Afloat's frunique collection id
	>;

	#[pallet::storage]
	#[pallet::getter(fn asset_id)]
	pub(super) type AfloatAssetId<T: Config> = StorageValue<
		_,
		<T as pallet_mapped_assets::Config>::AssetId, // Afloat's frunique collection id
	>;

	#[pallet::storage]
	#[pallet::getter(fn afloat_offers)]
	pub(super) type AfloatOffers<T: Config> =
		StorageMap<_, Blake2_128Concat, StorageId, Offer<T>, OptionQuery>;

	#[pallet::storage]
	#[pallet::getter(fn afloat_transactions)]
	pub(super) type AfloatTransactions<T: Config> =
		StorageMap<_, Blake2_128Concat, StorageId, Transaction<T>, OptionQuery>;

	#[pallet::call]
	impl<T: Config> Pallet<T>
	where
		T: pallet_uniques::Config<CollectionId = CollectionId>,
		<T as pallet_uniques::Config>::ItemId: From<u32>,
	{
		#[pallet::call_index(0)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn initial_setup(origin: OriginFor<T>, args: InitialSetupArgs<T>) -> DispatchResult {
			// Ensure sudo origin
			let _ = T::RemoveOrigin::ensure_origin(origin.clone())?;
			match args {
				InitialSetupArgs::All { creator, admin, asset } => {
					pallet_fruniques::Pallet::<T>::do_initial_setup()?;
					pallet_gated_marketplace::Pallet::<T>::do_initial_setup()?;

					Self::do_create_afloat_frunique(
						RawOrigin::Signed(creator.clone()).into(),
						admin.clone(),
					)?;

					// add permissions to admin
					Self::do_add_account_to_afloat_frunique(admin.clone(), FruniqueRole::Admin)?;
					Self::do_create_afloat_asset(creator.clone(), asset)?;
					Self::do_create_afloat_marketplace(creator.clone(), admin.clone())?;
					Self::do_setup_roles(creator, admin)?;
					Ok(())
				},
				InitialSetupArgs::Roles { creator, admin } => {
					Self::do_setup_roles(creator, admin)?;
					Ok(())
				},
				InitialSetupArgs::AddAfloatRole { who, role } => {
					Self::give_role_to_user(who, role)?;
					Ok(())
				},
				InitialSetupArgs::AddFruniqueRole { who, role } => {
					Self::do_add_account_to_afloat_frunique(who, role)?;
					Ok(())
				},
			}
		}

		#[pallet::call_index(1)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn kill_storage(origin: OriginFor<T>, args: KillStorageArgs) -> DispatchResult {
			// ensure sudo origin
			T::RemoveOrigin::ensure_origin(origin.clone())?;
			match args {
				KillStorageArgs::All => {
					Self::do_delete_all_users()?;
					<AfloatCollectionId<T>>::kill();
					<AfloatAssetId<T>>::kill();
					let _ = <AfloatOffers<T>>::clear(1000, None);
					let _ = <AfloatTransactions<T>>::clear(1000, None);
					<AfloatMarketPlaceId<T>>::kill();
				},
				KillStorageArgs::UserInfo => {
					Self::do_delete_all_users()?;
				},
				KillStorageArgs::AfloatMarketPlaceId => {
					<AfloatMarketPlaceId<T>>::kill();
				},
				KillStorageArgs::AfloatCollectionId => {
					<AfloatCollectionId<T>>::kill();
				},
				KillStorageArgs::AfloatAssetId => {
					<AfloatAssetId<T>>::kill();
				},
				KillStorageArgs::AfloatOffers => {
					let _ = <AfloatOffers<T>>::clear(1000, None);
				},
				KillStorageArgs::AfloatTransactions => {
					let _ = <AfloatTransactions<T>>::clear(1000, None);
				},
			}

			Ok(())
		}

		#[pallet::call_index(2)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn sign_up(origin: OriginFor<T>, args: SignUpArgs) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_user(who.clone(), who, args)
		}

		#[pallet::call_index(3)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn update_user_info(
			origin: OriginFor<T>,
			address: T::AccountId,
			args: UpdateUserArgs,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			let is_admin_or_owner = Self::is_admin_or_owner(who.clone())?;
			ensure!(<UserInfo<T>>::contains_key(address.clone()), Error::<T>::UserNotFound);
			ensure!(who.clone() == address || is_admin_or_owner, Error::<T>::Unauthorized);

			match args {
				UpdateUserArgs::Edit { cid, cid_creator } => {
					Self::do_edit_user(who, address, cid, cid_creator)?;
				},
				UpdateUserArgs::AdminEdit { cid, cid_creator, group } => {
					ensure!(is_admin_or_owner, Error::<T>::Unauthorized);
					Self::do_admin_edit_user(who, address, cid, cid_creator, group)?;
				},
				UpdateUserArgs::Delete => {
					ensure!(is_admin_or_owner, Error::<T>::Unauthorized);
					Self::do_delete_user(who, address)?;
				},
			}

			Ok(())
		}

		#[pallet::call_index(4)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_offer(origin: OriginFor<T>, args: CreateOfferArgs<T>) -> DispatchResult {
			let who = ensure_signed(origin)?;
			match args {
				CreateOfferArgs::Sell {
					tax_credit_amount,
					tax_credit_id,
					price_per_credit,
					expiration_date,
				} => {
					Self::do_create_sell_order(
						who,
						tax_credit_id,
						price_per_credit,
						tax_credit_amount,
						expiration_date,
					)?;
				},
				CreateOfferArgs::Buy {
					tax_credit_amount,
					tax_credit_id,
					price_per_credit,
					expiration_date,
				} => {
					Self::do_create_buy_order(
						who,
						tax_credit_id,
						price_per_credit,
						tax_credit_amount,
						expiration_date,
					)?;
				},
			}
			Ok(())
		}

		#[pallet::call_index(5)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn start_take_sell_order(
			origin: OriginFor<T>,
			offer_id: [u8; 32],
			tax_credit_amount: T::Balance,
		) -> DispatchResult {
			ensure_signed(origin.clone())?;
			Self::do_start_take_sell_order(origin, offer_id, tax_credit_amount)
		}

		#[pallet::call_index(6)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn confirm_sell_transaction(
			origin: OriginFor<T>,
			transaction_id: [u8; 32],
		) -> DispatchResult {
			ensure_signed(origin.clone())?;
			Self::do_confirm_sell_transaction(origin, transaction_id)
		}

		#[pallet::call_index(7)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn finish_take_sell_transaction(
			origin: OriginFor<T>,
			transaction_id: [u8; 32],
		) -> DispatchResult {
			ensure_signed(origin.clone())?;
			Self::do_finish_take_sell_transaction(origin, transaction_id)
		}

		#[pallet::call_index(8)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn create_tax_credit(
			origin: OriginFor<T>,
			metadata: CollectionDescription<T>,
			attributes: Option<Attributes<T>>,
			parent_info: Option<ParentInfo<T>>,
		) -> DispatchResult {
			let who = ensure_signed(origin)?;
			Self::do_create_tax_credit(who, metadata, attributes, parent_info)
		}

		#[pallet::call_index(9)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn set_afloat_balance(
			origin: OriginFor<T>,
			beneficiary: T::AccountId,
			amount: T::Balance,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			let is_admin_or_owner = Self::is_admin_or_owner(who.clone())?;
			ensure!(is_admin_or_owner, Error::<T>::Unauthorized);
			Self::do_set_afloat_balance(origin, beneficiary, amount)
		}

		#[pallet::call_index(10)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn add_afloat_admin(origin: OriginFor<T>, admin: T::AccountId) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			let is_admin_or_owner = Self::is_admin_or_owner(who.clone())?;
			ensure!(is_admin_or_owner, Error::<T>::Unauthorized);
			Self::do_add_afloat_admin(who, admin)
		}

		#[pallet::call_index(11)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn cancel_offer(origin: OriginFor<T>, order_id: StorageId) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			Self::do_cancel_offer(who, order_id)
		}

		#[pallet::call_index(12)]
		#[pallet::weight(Weight::from_parts(10_000,0) + T::DbWeight::get().reads_writes(1,1))]
		pub fn assign_user_to_role(
			origin: OriginFor<T>,
			user_address: T::AccountId,
			role: AfloatRole,
		) -> DispatchResult {
			let who = ensure_signed(origin.clone())?;
			let is_admin_or_owner = Self::is_admin_or_owner(who.clone())?;
			ensure!(is_admin_or_owner, Error::<T>::Unauthorized);
			ensure!(UserInfo::<T>::contains_key(user_address.clone()), Error::<T>::UserNotFound);
			Self::give_role_to_user(user_address.clone(), role.clone())?;
			Self::deposit_event(Event::UserAssignedToRoleAdded(who, user_address, role));
			Ok(())
		}
	}
}
