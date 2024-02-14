//! Confidential Docs pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{types::*, Pallet as BitcoinVaults};
use scale_info::prelude::*;

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, pallet_prelude::*, traits::Get};
use frame_system::RawOrigin;
use sp_io::hashing::blake2_256;

const SEED: u32 = 0;

pub fn generate_xpub_sized<T: Config>(id: u8, size: u32) -> BoundedVec<u8, T::XPubLen> {
	generate_vector(0, id, size).try_into().unwrap()
}

pub fn generate_description_sized<T: Config>(
	id: u8,
	size: u32,
) -> BoundedVec<u8, T::VaultDescriptionMaxLen> {
	generate_vector(1, id, size).try_into().unwrap()
}

pub fn generate_output_descriptor_sized<T: Config>(
	id: u8,
	size: u32,
) -> BoundedVec<u8, T::OutputDescriptorMaxLen> {
	generate_vector(2, id, size).try_into().unwrap()
}

pub fn generate_vector(prefix: u8, id: u8, size: u32) -> scale_info::prelude::vec::Vec<u8> {
	assert!(size > 0, "vector size must be greater than 0");
	let mut v = vec![id; size as usize];
	v[0] = prefix;
	v
}

pub fn get_vault_sized<T: Config>(
	id: u8,
	xpub_size: u32,
	desc_size: u32,
	num_cosigners: u32,
) -> (BoundedVec<u8, T::VaultDescriptionMaxLen>, BoundedVec<T::AccountId, T::MaxCosignersPerVault>)
{
	let desc = generate_description_sized::<T>(id, desc_size);
	let mut cosigners: BoundedVec<T::AccountId, T::MaxCosignersPerVault> = BoundedVec::new();
	for c in 1..=num_cosigners {
		let cosigner_id = id + c as u8;
		let cosigner: T::AccountId = account("cosigner", cosigner_id.into(), SEED);
		setup_xpub_sized::<T>(cosigner.clone(), cosigner_id, xpub_size);
		cosigners.try_push(cosigner).unwrap();
	}
	(desc, cosigners)
}

pub fn setup_vault_sized<T: Config>(
	owner: T::AccountId,
	id: u8,
	xpub_size: u32,
	desc_size: u32,
	num_cosigners: u32,
) {
	let (desc, cosigners) = get_vault_sized::<T>(id, xpub_size, desc_size, num_cosigners);
	assert_ok!(BitcoinVaults::<T>::create_vault(
		RawOrigin::Signed(owner.clone()).into(),
		1,
		desc,
		false,
		cosigners
	));
	let vault_id = BitcoinVaults::<T>::vaults_by_signer(owner).pop().unwrap();
	finalize_vault::<T>(vault_id, num_cosigners * xpub_size);
}

pub fn setup_vaults_sized<T: Config>(
	owner: T::AccountId,
	num_vaults: u32,
	xpub_size: u32,
	desc_size: u32,
	num_cosigners: u32,
) {
	for v in 1..=num_vaults {
		setup_vault_sized::<T>(
			owner.clone(),
			(v * num_cosigners) as u8,
			xpub_size,
			desc_size,
			num_cosigners,
		);
	}
}

pub fn setup_xpub_sized<T: Config>(owner: T::AccountId, id: u8, size: u32) {
	println!("xpub id: {:?}", id);
	let xpub = generate_xpub_sized::<T>(id, size);
	assert_ok!(BitcoinVaults::<T>::set_xpub(RawOrigin::Signed(owner.clone()).into(), xpub.clone()));
}

pub fn finalize_vault<T: Config>(vault_id: [u8; 32], descriptor_size: u32) {
	Vaults::<T>::mutate(vault_id, |v_option| {
		let v = v_option.as_mut().unwrap();
		v.offchain_status.clone_from(&BDKStatus::Valid);
		v.descriptors.clone_from(&Descriptors {
			output_descriptor: generate_output_descriptor_sized::<T>(1, descriptor_size),
			change_descriptor: Some(generate_output_descriptor_sized::<T>(2, descriptor_size)),
		});
	});
}

#[benchmarks(where T: Config)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_xpub(x: Linear<1, { T::XPubLen::get() }>) {
		let owner: T::AccountId = account("owner", 0, SEED);
		let xpub = generate_xpub_sized::<T>(1, x);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), xpub.clone());
		assert_eq!(BitcoinVaults::<T>::xpubs_by_owner(owner), Some(xpub.using_encoded(blake2_256)));
	}

	#[benchmark]
	fn create_vault(
		x: Linear<2, { T::XPubLen::get() }>,
		d: Linear<1, { T::VaultDescriptionMaxLen::get() }>,
		c: Linear<2, { T::MaxCosignersPerVault::get() }>,
		v: Linear<0, { T::MaxVaultsPerUser::get() - 1 }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, x);
		setup_vaults_sized::<T>(owner.clone(), v, x, d, c);
		let (desc, cosigners) = get_vault_sized::<T>(0, x, d, c);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), 1, desc, false, cosigners);
		assert!(!BitcoinVaults::<T>::vaults_by_signer(owner).is_empty());
	}

	impl_benchmark_test_suite! {
		BitcoinVaults,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
