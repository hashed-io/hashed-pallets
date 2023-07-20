use super::*;
use frame_support::{pallet_prelude::*};
use frame_support::traits::Time;
use frame_support::sp_io::hashing::blake2_256;
use sp_runtime::sp_std::vec::Vec; // vec primitive
use scale_info::prelude::vec; // vec![] macro
// use frame_system::Config;
// use frame_system::Pallet;

use crate::types::*;

impl<T: Config> Pallet<T> {

  pub fn do_initial_setup() -> DispatchResult{
    Ok(())
  }

}