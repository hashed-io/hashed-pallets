use crate::{mock::*, types::*, Error, Records};
use frame_support::{assert_noop, assert_ok};
use sp_core::bounded_vec;

fn make_project_id(v: &str) -> ProjectId {
	let v: ProjectId = v.as_bytes().to_vec().try_into().unwrap_or_default();
	v
}

fn make_hashed_info(v: &str) -> HashedInfo {
	let v: HashedInfo = v.as_bytes().to_vec().try_into().unwrap_or_default();
	v
}

fn make_record_collection(
	project_id: ProjectId,
	hashed_info: HashedInfo,
	table: TableType,
	record_type: RecordType,
) -> RecordCollection<Test> {
	let mut record_collection: RecordCollection<Test> = bounded_vec![];
	record_collection
		.try_push((project_id, hashed_info, table, record_type))
		.unwrap_or_default();
	record_collection
}

#[test]
fn set_signer_account_works() {
	new_test_ext().execute_with(|| {
		let signer_account = 1;
		assert_ok!(FundAdminRecords::set_signer_account(RuntimeOrigin::root(), signer_account));
		assert_eq!(FundAdminRecords::signer_account(), Some(signer_account));
	});
}

#[test]
fn cannot_add_record_if_signer_account_is_not_set() {
	new_test_ext().execute_with(|| {
		let project_id = make_project_id("project_id_testing");
		let hashed_info = make_hashed_info("hashed_info_testing");
		let table = TableType::Drawdown;
		let record_type = RecordType::Creation;
		let recod_request = make_record_collection(project_id, hashed_info, table, record_type);

		assert_noop!(
			FundAdminRecords::add_record(RuntimeOrigin::signed(1), recod_request),
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
		let table_type = TableType::Drawdown;
		let record_type = RecordType::Creation;
		let recod_request = make_record_collection(
			project_id.clone(),
			hashed_info.clone(),
			table_type,
			record_type,
		);

		assert_ok!(FundAdminRecords::set_signer_account(RuntimeOrigin::root(), signer_account));

		assert_ok!(FundAdminRecords::add_record(
			RuntimeOrigin::signed(signer_account),
			recod_request
		));

		let record_id = Records::<Test>::iter_keys().next().unwrap().1;

		// Get record data
		let record_data =
			FundAdminRecords::records((project_id.clone(), table_type), record_id).unwrap();

		assert_eq!(record_data.project_id, project_id);
		assert_eq!(record_data.hashed_info, hashed_info);
		assert_eq!(record_data.table_type, table_type);
		assert_eq!(record_data.record_type, record_type);
	});
}
