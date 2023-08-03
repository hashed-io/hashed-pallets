use super::*;
use frame_support::pallet_prelude::*;
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;

use crate::types::*;

impl<T: Config> Pallet<T> {
  /*---- Offchain extrinsics ----*/
  pub fn do_add_record(
    records: RecordCollection<T>,
  ) -> DispatchResult{
    for record in records.iter().cloned() {
      // Validations
      ensure!(!record.0.is_empty(), Error::<T>::ProjectIdIsEmpty);
      ensure!(!record.1.is_empty(), Error::<T>::HashedInfoIsEmpty);

      let project_id_validated = ProjectId::try_from(record.0.clone())
        .map_err(|_| Error::<T>::ProjectIdExceededMaxLength)?;

      let hashed_info_validated = HashedInfo::try_from(record.1.clone())
        .map_err(|_| Error::<T>::HashedInfoExceededMaxLength)?;

      // Get timestamp
      let creation_date: CreationDate = Self::get_timestamp_in_milliseconds().ok_or(Error::<T>::TimestampError)?;

      let record_id: Id = (record.0.clone(), creation_date).using_encoded(blake2_256);

      // Ensure the generated id is unique
      ensure!(!Records::<T>::contains_key((record.0.clone(), record.2), record_id), Error::<T>::IdAlreadyExists);
      
      let record_data = RecordData {
        project_id: project_id_validated,
        hashed_info: hashed_info_validated,
        table_type: record.2,
        record_type: record.3,
        creation_date,
      };
  
      // Insert the record into the storage
      <Records<T>>::insert(
        (record.0.clone(), record.2),
        &record_id,
        record_data
      );
  
      Self::deposit_event(Event::RecordAdded(record.0, record.2, record.3, record_id));
    }
    Ok(())
  }

  fn get_timestamp_in_milliseconds() -> Option<u64> {
    let timestamp: u64 = T::Timestamp::now().into();

    Some(timestamp)
  }

}