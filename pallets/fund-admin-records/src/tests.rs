use crate::{mock::*, types::*, Records, Error};
use frame_support::{assert_ok, assert_noop, BoundedVec, traits::ConstU32};

fn make_project_id(v: &str) -> ProjectId {
  let v: BoundedVec<u8, ConstU32<50>> = v.as_bytes().to_vec().try_into().unwrap_or_default();
  v
}

fn make_hashed_info(v: &str) -> HashedInfo {
  let v: BoundedVec<u8, ConstU32<400>> = v.as_bytes().to_vec().try_into().unwrap_or_default();
  v
}

#[test]
fn set_signer_account_works() {
  new_test_ext().execute_with(|| {
    let signer_account = 1;
    assert_ok!(FundAdminRecords::set_signer_account(Origin::root(), signer_account));
    assert_eq!(FundAdminRecords::signer_account(), Some(signer_account));
  });
}

#[test]
fn cannot_add_record_if_signer_account_is_not_set() {
  new_test_ext().execute_with(|| {
    let project_id = make_project_id("project_id_testing");
    let hashed_info = make_hashed_info("hashed_info_testing");
    let table = Table::Drawdown;
    let record_type = RecordType::Creation;

    assert_noop!(
      FundAdminRecords::add_record(Origin::signed(1), project_id, hashed_info, table, record_type),
      Error::<Test>::SignerAccountNotSet
    );
  });
}

#[test]
fn add_drawdown_record_works() {
  new_test_ext().execute_with(|| {
    let signer_account = 1;
    let project_id = make_project_id("project_id_testing");
    let hashed_info = make_hashed_info("hashed_info_testing");
    let table = Table::Drawdown;
    let record_type = RecordType::Creation;

    assert_ok!(FundAdminRecords::set_signer_account(Origin::root(), signer_account));

    assert_ok!(FundAdminRecords::add_record(
      Origin::signed(signer_account),
      project_id.clone(),
      hashed_info.clone(),
      table,
      record_type,
    ));

    let record_id = Records::<Test>::iter_keys().next().unwrap().1;

    // Get record data
    let record_data = FundAdminRecords::records( (project_id.clone(), table), record_id).unwrap();

    assert_eq!(record_data.project_id, project_id);
    assert_eq!(record_data.hashed_info, hashed_info);
    assert_eq!(record_data.table, table);
    assert_eq!(record_data.record_type, record_type);
  });
}
