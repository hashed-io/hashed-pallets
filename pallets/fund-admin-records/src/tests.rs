use crate::{mock::*, Error, types::*, Config, Records};
use frame_support::{assert_ok, BoundedVec, traits::ConstU32, assert_noop, error::BadOrigin, bounded_vec};
use sp_io::hashing::blake2_256;
use sp_runtime::DispatchResult;
use std::vec;


fn return_cid(name: &str) -> CID {
  let name: BoundedVec<u8, ConstU32<100>> = name.as_bytes().to_vec().try_into().unwrap_or_default();
  name
}

fn return_description(description: &str) -> Description {
  let description: BoundedVec<u8, ConstU32<400>> = description.as_bytes().to_vec().try_into().unwrap_or_default();
  description
}


#[test]
fn add_drawdown_record_works() {
  new_test_ext().execute_with(|| {
    let project_id = [0u8; 36];
    let table = Table::Drawdown;
    let cid = return_cid("cid");
    let description = return_description("description");

    assert_ok!(FundAdminRecords::do_add_record(
      project_id,
      table,
      cid.clone(),
      description.clone()
    ));

    let record_id = Records::<Test>::iter_keys().next().unwrap();

    // Get record data
    let record_data = FundAdminRecords::records(record_id, (project_id, table)).unwrap();

    // assert_eq!(record_data.cid, cid);
    // assert_eq!(record_data.description, description);
    // assert!(record_data.creation_date > 0);
    // assert_eq!(record_data.updated_date, None);
  });
}