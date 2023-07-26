use super::*;
use frame_support::pallet_prelude::*;
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;

use crate::types::*;

impl<T: Config> Pallet<T> {
  /*---- Offchain extrinsics ----*/
  pub fn do_add_record(
    project_id: ProjectId,
    cid: CID,
    description: Description,
    table: Table,
    record_type: RecordType,
  ) -> DispatchResult{
    // Get timestamp
    let creation_date: CreationDate = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

    let record_id: Id = (project_id, creation_date).using_encoded(blake2_256);
    // Ensure the generated id is unique
    ensure!(!Records::<T>::contains_key((project_id, table), record_id), Error::<T>::IdAlreadyExists);

    let record_data = RecordData {
      project_id,
      cid,
      description,
      table,
      record_type,
      creation_date,
    };

    // Insert the record into the storage
    <Records<T>>::insert(
      (project_id, table),
      &record_id,
      record_data
    );

    Self::deposit_event(Event::RecordAdded(project_id, table, record_id));

    Ok(())
  }

  fn get_timestamp_in_milliseconds() -> Option<u64> {
    let timestamp: u64 = T::Timestamp::now().into();

    Some(timestamp)
  }

}