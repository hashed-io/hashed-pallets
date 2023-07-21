use super::*;
use frame_support::pallet_prelude::*;
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec; // vec primitive
use scale_info::prelude::vec; // vec![] macro

use crate::types::*;

impl<T: Config> Pallet<T> {

  pub fn do_add_record(
    project_id: ProjectId,
    table: Table,
    cid: CID,
    description: Description,
  ) -> DispatchResult{
    // Get timestamp
    let creation_date = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

    let record_id = (project_id, creation_date).using_encoded(blake2_256);
    // Ensure the generated id is unique
    ensure!(!Records::<T>::contains_key((project_id, table), record_id), Error::<T>::IdAlreadyExists);

    let record_data = RecordData {
      cid,
      description,
      creation_date,
      updated_date: None,
    };

    // Insert the record into the storage
    <Records<T>>::insert(
      (project_id, table),
      &record_id,
      record_data
    );

    Self::deposit_event(Event::RecordAdded(record_id));

    Ok(())
  }

  fn get_timestamp_in_milliseconds() -> Option<u64> {
    let timestamp: u64 = T::Timestamp::now().into();

    Some(timestamp)
  }

}