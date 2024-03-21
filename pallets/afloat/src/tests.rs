use super::*;
use crate::{mock::*, types::*, AfloatOffers, Error};
use frame_support::{assert_noop, assert_ok, traits::Currency, BoundedVec};
use frame_system::RawOrigin;

fn new_account(account_id: u64) -> <Test as frame_system::Config>::AccountId {
	account_id
}

fn dummy_description() -> BoundedVec<u8, StringLimit> {
	BoundedVec::<u8, StringLimit>::try_from(b"dummy description".to_vec()).unwrap()
}

//owner_id = 1
//admin_id = 2
//buy_fee = 2%
//sell_fee = 4%

#[test]
fn replicate_overflow_for_start_take_sell_order() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);
		let item_id = 0;

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
		assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

		assert_ok!(Afloat::set_afloat_balance(
			RuntimeOrigin::signed(1),
			other_user.clone(),
			100000
		));
		assert_eq!(Afloat::do_get_afloat_balance(other_user.clone()).unwrap(), 100000);

		assert_ok!(Afloat::create_tax_credit(
			RawOrigin::Signed(user.clone()).into(),
			dummy_description(),
			None,
			None,
		));

		assert_ok!(Afloat::create_offer(
			RawOrigin::Signed(user.clone()).into(),
			CreateOfferArgs::Sell {
				tax_credit_id: item_id,
				price_per_credit: 18446744073709551615,
				tax_credit_amount: 10,
				expiration_date: 1000
			}
		));

		let (offer_id, offer) = AfloatOffers::<Test>::iter().next().unwrap().clone();
		assert_ok!(Afloat::start_take_sell_order(
			RawOrigin::Signed(other_user.clone()).into(),
			offer_id,
			10
		));
	});
}

#[test]
fn sign_up_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		Balances::make_free_balance_be(&user, 100);
		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

		assert!(UserInfo::<Test>::contains_key(user));
	});
}

#[test]
fn update_user_info_edit_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		Balances::make_free_balance_be(&user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

		let update_args = UpdateUserArgs::Edit {
			cid: ShortString::try_from(b"New".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"User".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::update_user_info(
			RawOrigin::Signed(user.clone()).into(),
			user.clone(),
			update_args
		));

		let updated_user = UserInfo::<Test>::get(user).unwrap();
		assert_eq!(updated_user.cid, ShortString::try_from(b"New".to_vec()).unwrap());
		assert_eq!(updated_user.cid_creator, ShortString::try_from(b"User".to_vec()).unwrap());
	});
}

#[test]
fn update_other_user_info_by_not_admin_fails() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

		let update_args = UpdateUserArgs::Edit {
			cid: ShortString::try_from(b"New".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"User".to_vec()).unwrap(),
		};

		assert_noop!(
			Afloat::update_user_info(
				RawOrigin::Signed(other_user.clone()).into(),
				user.clone(),
				update_args
			),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn update_other_user_info_by_admin_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let owner = new_account(1);
		let admin = new_account(2);
		let user = new_account(3);
		let other_user = new_account(4);

		Balances::make_free_balance_be(&owner, 100);
		Balances::make_free_balance_be(&admin, 100);
		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

		let update_args = UpdateUserArgs::Edit {
			cid: ShortString::try_from(b"New".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"User".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::update_user_info(
			RawOrigin::Signed(admin.clone()).into(),
			user.clone(),
			update_args
		));

		let updated_user = UserInfo::<Test>::get(user).unwrap();
		assert_eq!(updated_user.cid, ShortString::try_from(b"New".to_vec()).unwrap());
		assert_eq!(updated_user.cid_creator, ShortString::try_from(b"User".to_vec()).unwrap());
	});
}

#[test]
fn update_user_info_delete_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		Balances::make_free_balance_be(&user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args));

		assert_ok!(Afloat::update_user_info(
			RawOrigin::Signed(2).into(),
			user.clone(),
			UpdateUserArgs::Delete
		));

		assert!(!UserInfo::<Test>::contains_key(user));
	});
}

#[test]
fn set_afloat_balance_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));

		assert_ok!(Afloat::set_afloat_balance(RawOrigin::Signed(1).into(), user.clone(), 10000));
		assert_eq!(Afloat::do_get_afloat_balance(user.clone()).unwrap(), 10000);
		assert_ok!(Afloat::set_afloat_balance(RawOrigin::Signed(1).into(), user.clone(), 1000));
		assert_eq!(Afloat::do_get_afloat_balance(user.clone()).unwrap(), 1000);
	});
}

#[test]
fn set_balance_by_other_than_owner_fails() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));

		assert_noop!(
			Afloat::set_afloat_balance(RawOrigin::Signed(3).into(), other_user.clone(), 10000),
			Error::<Test>::Unauthorized
		);
		assert_noop!(
			Afloat::set_afloat_balance(RawOrigin::Signed(4).into(), other_user.clone(), 10000),
			Error::<Test>::Unauthorized
		);
	});
}

#[test]
fn set_balance_without_initializing_afloat_asset_fails() {
	TestExternalitiesBuilder::new().initialize_roles().build().execute_with(|| {
		let other_user = new_account(4);
		assert_noop!(
			Afloat::set_afloat_balance(RawOrigin::Signed(1).into(), other_user.clone(), 10000),
			Error::<Test>::AfloatAssetNotSet
		);
	});
}

#[test]
fn create_tax_credit_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
		assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

		assert_ok!(Afloat::create_tax_credit(
			RawOrigin::Signed(user.clone()).into(),
			dummy_description(),
			None,
			None,
		));
	});
}

#[test]
fn create_sell_order_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);
		let item_id = 0;

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};
		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
		assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

		assert_ok!(Afloat::create_tax_credit(
			RawOrigin::Signed(3).into(),
			dummy_description(),
			None,
			None,
		));

		assert_ok!(Afloat::create_offer(
			RawOrigin::Signed(user.clone()).into(),
			CreateOfferArgs::Sell {
				tax_credit_id: item_id,
				price_per_credit: 10000,
				tax_credit_amount: 10,
				expiration_date: 1000
			}
		));
	});
}

#[test]
fn create_buy_order_works() {
	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
		let user = new_account(3);
		let other_user = new_account(4);
		let item_id = 0;

		Balances::make_free_balance_be(&user, 100);
		Balances::make_free_balance_be(&other_user, 100);

		let args = SignUpArgs::BuyerOrSeller {
			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
		};

		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
		assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

		assert_ok!(Afloat::set_afloat_balance(RuntimeOrigin::signed(1), 4, 100000));

		assert_ok!(Afloat::create_tax_credit(
			RawOrigin::Signed(user.clone()).into(),
			dummy_description(),
			None,
			None,
		));

		assert_ok!(Afloat::create_offer(
			RawOrigin::Signed(other_user.clone()).into(),
			CreateOfferArgs::Buy {
				tax_credit_id: item_id,
				price_per_credit: 10000,
				tax_credit_amount: 10,
				expiration_date: 1000
			}
		));
	});
}

// #[test]
// fn take_buy_order_works() {
// 	TestExternalitiesBuilder::new().initialize_all().build().execute_with(|| {
// 		let user = new_account(3);
// 		let other_user = new_account(4);
// 		let item_id = 0;

// 		Balances::make_free_balance_be(&user, 100);
// 		Balances::make_free_balance_be(&other_user, 100);

// 		let args = SignUpArgs::BuyerOrSeller {
// 			cid: ShortString::try_from(b"cid".to_vec()).unwrap(),
// 			cid_creator: ShortString::try_from(b"cid_creator".to_vec()).unwrap(),
// 			group: ShortString::try_from(b"Group".to_vec()).unwrap(),
// 		};

// 		assert_ok!(Afloat::sign_up(RawOrigin::Signed(user.clone()).into(), args.clone()));
// 		assert_ok!(Afloat::sign_up(RawOrigin::Signed(other_user.clone()).into(), args.clone()));

// 		assert_ok!(Afloat::set_afloat_balance(RuntimeOrigin::signed(1), 4, 100000));

// 		assert_ok!(Afloat::create_tax_credit(
// 			RawOrigin::Signed(user.clone()).into(),
// 			dummy_description(),
// 			None,
// 			None,
// 		));

// 		assert_ok!(Afloat::create_offer(
// 			RawOrigin::Signed(other_user.clone()).into()
// 			CreateOfferArgs::Sell {
// 				tax_credit_id: item_id,
// 				price_per_credit: 10000,
// 				tax_credit_amount: 10,
// 				expiration_date: 1000
// 			}
// 		));

// 		let offer_id = GatedMarketplace::offers_by_item(0, 0).iter().next().unwrap().clone();

// 		assert_ok!(Afloat::take_buy_order(RawOrigin::Signed(user.clone()).into(), offer_id,));

// 		assert_eq!(Afloat::do_get_afloat_balance(user.clone()), 9800); // 10000 - 200 (buy fee)
// 		assert_eq!(Afloat::do_get_afloat_balance(1), 200); // 200 (buy fee)
// 	});
// }
