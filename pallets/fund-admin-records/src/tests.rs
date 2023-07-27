use crate::{mock::*, types::*, Records, Error};
use frame_support::{assert_ok, assert_noop};

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
    let project_id: ProjectId = [0; 60];
    let hashed_info: HashedInfo = [0; 60];
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
    let project_id = [0; 60];
    let hashed_info = [0; 60];
    let table = Table::Drawdown;
    let record_type = RecordType::Creation;

    assert_ok!(FundAdminRecords::set_signer_account(Origin::root(), signer_account));

    assert_ok!(FundAdminRecords::add_record(
      Origin::signed(signer_account),
      project_id,
      hashed_info,
      table,
      record_type,
    ));

    let record_id = Records::<Test>::iter_keys().next().unwrap().1;

    // Get record data
    let record_data = FundAdminRecords::records( (project_id, table), record_id).unwrap();

    assert_eq!(record_data.project_id, project_id);
    assert_eq!(record_data.hashed_info, hashed_info);
    assert_eq!(record_data.table, table);
    assert_eq!(record_data.record_type, record_type);
  });
}
