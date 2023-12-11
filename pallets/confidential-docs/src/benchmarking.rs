//! Confidential Docs pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::Pallet as ConfidentialDocs;

use frame_benchmarking::v2::*;
use frame_system::RawOrigin;
use sp_runtime::traits::Bounded;
use types::ExtraFlags;

const SEED: u32 = 0;

#[instance_benchmarks]
mod benchmarks {
	use super::*;

	// Benchmark `transfer` extrinsic with the worst possible conditions:
	// * Transfer will kill the sender account.
	// * Transfer will create the recipient account.
	#[benchmark]
	fn set_vault() {
		let existential_deposit = T::ExistentialDeposit::get();
		let caller = whitelisted_caller();

		// Give some multiple of the existential deposit
		let balance = existential_deposit.saturating_mul(ED_MULTIPLIER.into());
		let _ = <Balances<T, I> as Currency<_>>::make_free_balance_be(&caller, balance);

		// Transfer `e - 1` existential deposits + 1 unit, which guarantees to create one account,
		// and reap this user.
		let recipient: T::AccountId = account("recipient", 0, SEED);
		let recipient_lookup = T::Lookup::unlookup(recipient.clone());
		let transfer_amount =
			existential_deposit.saturating_mul((ED_MULTIPLIER - 1).into()) + 1u32.into();

		#[extrinsic_call]
		_(RawOrigin::Signed(caller.clone()), recipient_lookup, transfer_amount);

		assert_eq!(Balances::<T, I>::free_balance(&caller), Zero::zero());
		assert_eq!(Balances::<T, I>::free_balance(&recipient), transfer_amount);
	}

	impl_benchmark_test_suite! {
		Balances,
		crate::tests::ExtBuilder::default().build(),
		crate::tests::Test,
	}
}
