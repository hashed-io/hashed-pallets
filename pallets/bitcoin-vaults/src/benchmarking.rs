//! Bitcoin Vaults pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{types::*, Pallet as BitcoinVaults};
use scale_info::prelude::*;

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, pallet_prelude::*, traits::Get};
use frame_system::RawOrigin;
use sp_io::hashing::blake2_256;

const SEED: u32 = 0;

pub fn generate_xpub_sized<T: Config>(id: u8, size: u32) -> XPUB<T> {
	generate_vector(0, id, size).try_into().unwrap()
}

pub fn generate_description_sized<T: Config>(id: u8, size: u32) -> Description<T> {
	generate_vector(1, id, size).try_into().unwrap()
}

pub fn generate_output_descriptor_sized<T: Config>(id: u8, size: u32) -> OutputDescriptor<T> {
	generate_vector(2, id, size).try_into().unwrap()
}

pub fn generate_url_sized(id: u8, size: u32) -> URL {
	generate_vector(3, id, size).try_into().unwrap()
}

pub fn generate_psbt_sized<T: Config>(id: u8, size: u32) -> PSBT<T> {
	generate_vector(4, id, size).try_into().unwrap()
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
) -> (Description<T>, Cosigners<T>) {
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
) -> VaultId {
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
	vault_id
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

pub fn generate_insert_output_descriptors_payload_sized<T: Config>(
	descriptor_size: u32,
) -> Vec<SingleVaultPayload> {
	let mut payload = Vec::<SingleVaultPayload>::new();
	for (vault_id, vault) in <Vaults<T>>::iter() {
		if vault.offchain_status == BDKStatus::Pending {
			payload.push(SingleVaultPayload {
				vault_id,
				output_descriptor: generate_output_descriptor_sized::<T>(1, descriptor_size).into(),
				change_descriptor: generate_output_descriptor_sized::<T>(2, descriptor_size).into(),
				status: OffchainStatus::Valid,
			});
		}
	}
	payload
}

pub fn setup_xpub_sized<T: Config>(owner: T::AccountId, id: u8, size: u32) {
	// println!("xpub id: {:?}", id);
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

pub fn get_proposal_sized<T: Config>(
	id: u8,
	xpub_size: u32,
	desc_size: u32,
) -> (XPUB<T>, u64, Description<T>) {
	let recipient_address = generate_xpub_sized::<T>(id, xpub_size);
	let desc = generate_description_sized::<T>(id, desc_size);
	let amount_in_sats = id as u64 + 1000;
	(recipient_address, amount_in_sats, desc)
}

pub fn setup_proposal_sized<T: Config>(
	proposer: T::AccountId,
	vault_id: VaultId,
	id: u8,
	xpub_size: u32,
	desc_size: u32,
	psbt_size: u32,
) -> ProposalId {
	let (recipient_address, amount_in_sats, description) =
		get_proposal_sized::<T>(id, xpub_size, desc_size);
	assert_ok!(BitcoinVaults::<T>::propose(
		RawOrigin::Signed(proposer.clone()).into(),
		vault_id,
		recipient_address,
		amount_in_sats,
		description
	));
	let proposal_id = BitcoinVaults::<T>::proposals_by_vault(vault_id).pop().unwrap();
	if psbt_size > 0 {
		sign_proposal_sized::<T>(proposal_id, psbt_size, false, true);
	}
	proposal_id
}

pub fn sign_proposal_sized<T: Config>(
	proposal_id: ProposalId,
	psbt_size: u32,
	skip_last: bool,
	finalize: bool,
) -> Option<T::AccountId> {
	let proposal = BitcoinVaults::<T>::proposals(proposal_id).unwrap();
	let vault = BitcoinVaults::<T>::vaults(proposal.vault_id).unwrap();
	let mut cosigners = vault.cosigners;
	let mut last_cosigner = None;
	if skip_last {
		last_cosigner = cosigners.pop();
	}
	cosigners.iter().for_each(|cosigner| {
		assert_ok!(BitcoinVaults::<T>::save_psbt(
			RawOrigin::Signed(cosigner.clone()).into(),
			proposal_id,
			generate_psbt_sized::<T>(1, psbt_size)
		));
	});
	if finalize {
		prepare_proposal_for_finalization::<T>(proposal_id);
		assert_ok!(BitcoinVaults::<T>::finalize_psbt(
			RawOrigin::Signed(cosigners[0].clone()).into(),
			proposal_id,
			false
		));
		<Proposals<T>>::mutate(proposal_id, |p_option| {
			let p = p_option.as_mut().unwrap();
			p.psbt = generate_psbt_sized::<T>(2, psbt_size);
		});
	}
	last_cosigner
}

pub fn sign_proof_sized<T: Config>(
	vault_id: VaultId,
	psbt_size: u32,
	skip_last: bool,
	finalize: bool,
) -> Option<T::AccountId> {
	let vault = BitcoinVaults::<T>::vaults(vault_id).unwrap();
	let mut cosigners = vault.cosigners;
	let mut last_cosigner = None;
	if skip_last {
		last_cosigner = cosigners.pop();
	}
	cosigners.iter().for_each(|cosigner| {
		assert_ok!(BitcoinVaults::<T>::save_proof_psbt(
			RawOrigin::Signed(cosigner.clone()).into(),
			vault_id,
			generate_psbt_sized::<T>(1, psbt_size)
		));
	});
	if finalize {
		assert_ok!(BitcoinVaults::<T>::finalize_proof(
			RawOrigin::Signed(cosigners[0].clone()).into(),
			vault_id,
			generate_psbt_sized::<T>(2, psbt_size)
		));
	}
	last_cosigner
}

pub fn prepare_proposal_for_finalization<T: Config>(proposal_id: ProposalId) {
	<Proposals<T>>::mutate(proposal_id, |p_option| {
		let p = p_option.as_mut().unwrap();
		p.status.clone_from(&ProposalStatus::Pending);
		p.offchain_status.clone_from(&BDKStatus::Valid);
	});
}

pub fn setup_proposals_sized<T: Config>(
	proposer: T::AccountId,
	vault_id: VaultId,
	num_proposals: u32,
	xpub_size: u32,
	desc_size: u32,
	psbt_size: u32,
) {
	for p in 1..=num_proposals {
		setup_proposal_sized::<T>(
			proposer.clone(),
			vault_id,
			p as u8,
			xpub_size,
			desc_size,
			psbt_size,
		);
	}
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
	fn remove_xpub(
		x: Linear<2, { T::XPubLen::get() }>,
		d: Linear<1, { T::VaultDescriptionMaxLen::get() }>,
		c: Linear<2, { T::MaxCosignersPerVault::get() }>,
		v: Linear<0, { T::MaxVaultsPerUser::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, x);
		// setup_vaults_sized::<T>(owner.clone(), v, x, d, c);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()));
		assert_eq!(BitcoinVaults::<T>::xpubs_by_owner(owner), None);
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
		assert_eq!(BitcoinVaults::<T>::vaults_by_signer(owner).len() as u32, v + 1);
	}

	#[benchmark]
	fn remove_vault(
		x: Linear<2, { T::XPubLen::get() }>,
		d: Linear<1, { T::VaultDescriptionMaxLen::get() }>,
		c: Linear<2, { T::MaxCosignersPerVault::get() }>,
		v: Linear<1, { T::MaxVaultsPerUser::get() }>,
		p: Linear<0, { T::MaxProposalsPerVault::get() }>,
		s: Linear<2, { T::PSBTMaxLen::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, x);
		setup_vaults_sized::<T>(owner.clone(), v, x, d, c);
		let vault_id = BitcoinVaults::<T>::vaults_by_signer(owner.clone()).pop().unwrap();
		setup_proposals_sized::<T>(owner.clone(), vault_id, p, x, d, s);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), vault_id);
		assert_eq!(BitcoinVaults::<T>::vaults(vault_id), None);
	}

	#[benchmark]
	fn propose(
		x: Linear<2, { T::XPubLen::get() }>,
		d: Linear<1, { T::VaultDescriptionMaxLen::get() }>,
		p: Linear<0, { T::MaxProposalsPerVault::get() - 1 }>,
		s: Linear<2, { T::PSBTMaxLen::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, x);
		let vault_id =
			setup_vault_sized::<T>(owner.clone(), 0, x, d, T::MaxCosignersPerVault::get());

		setup_proposals_sized::<T>(owner.clone(), vault_id, p, x, d, s);

		let (recipient_address, amount_in_sats, description) = get_proposal_sized::<T>(0, x, d);
		#[extrinsic_call]
		_(
			RawOrigin::Signed(owner.clone()),
			vault_id,
			recipient_address,
			amount_in_sats,
			description,
		);
		assert_eq!(BitcoinVaults::<T>::proposals_by_vault(vault_id).len() as u32, p + 1);
	}

	#[benchmark]
	fn remove_proposal(
		x: Linear<2, { T::XPubLen::get() }>,
		d: Linear<1, { T::VaultDescriptionMaxLen::get() }>,
		p: Linear<1, { T::MaxProposalsPerVault::get() }>,
		s: Linear<2, { T::PSBTMaxLen::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, x);
		let vault_id =
			setup_vault_sized::<T>(owner.clone(), 0, x, d, T::MaxCosignersPerVault::get());

		setup_proposals_sized::<T>(owner.clone(), vault_id, p, x, d, s);
		let proposal_id = BitcoinVaults::<T>::proposals_by_vault(vault_id).pop().unwrap();

		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), proposal_id);
		assert!(BitcoinVaults::<T>::proposals(proposal_id).is_none());
	}

	#[benchmark]
	fn set_bdk_url(u: Linear<10, { <URLSize as sp_core::TypedGet>::get() }>) {
		let url = generate_url_sized(1, u);
		#[extrinsic_call]
		_(RawOrigin::Root, url.clone());
		assert_eq!(<BDKServicesURL<T>>::get(), url);
	}

	#[benchmark]
	fn save_psbt(
		c: Linear<2, { T::MaxCosignersPerVault::get() }>,
		s: Linear<2, { T::PSBTMaxLen::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
		let vault_id = setup_vault_sized::<T>(
			owner.clone(),
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			c,
		);
		let proposal_id = setup_proposal_sized::<T>(
			owner.clone(),
			vault_id,
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			0,
		);
		let cosigner = sign_proposal_sized::<T>(proposal_id, s, true, false).unwrap();
		let psbt = generate_psbt_sized::<T>(1, s);
		#[extrinsic_call]
		_(RawOrigin::Signed(cosigner.clone()), proposal_id, psbt);
		let proposal = BitcoinVaults::<T>::proposals(proposal_id).unwrap();
		let vault = BitcoinVaults::<T>::vaults(vault_id).unwrap();
		assert_eq!(proposal.signed_psbts.len(), vault.cosigners.len());
	}

	#[benchmark]
	fn finalize_psbt(
		c: Linear<2, { T::MaxCosignersPerVault::get() }>,
		s: Linear<2, { T::PSBTMaxLen::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
		let vault_id = setup_vault_sized::<T>(
			owner.clone(),
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			c,
		);
		let proposal_id = setup_proposal_sized::<T>(
			owner.clone(),
			vault_id,
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			0,
		);
		sign_proposal_sized::<T>(proposal_id, s, false, false);
		prepare_proposal_for_finalization::<T>(proposal_id);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), proposal_id, false);
		let proposal = BitcoinVaults::<T>::proposals(proposal_id).unwrap();
		assert_eq!(proposal.status, ProposalStatus::ReadyToFinalize(false));
	}

	#[benchmark]
	fn broadcast_psbt(s: Linear<2, { T::PSBTMaxLen::get() }>) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
		let vault_id = setup_vault_sized::<T>(
			owner.clone(),
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			T::MaxCosignersPerVault::get(),
		);
		let proposal_id = setup_proposal_sized::<T>(
			owner.clone(),
			vault_id,
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			0,
		);
		sign_proposal_sized::<T>(proposal_id, s, false, false);
		prepare_proposal_for_finalization::<T>(proposal_id);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), proposal_id);
		let proposal = BitcoinVaults::<T>::proposals(proposal_id).unwrap();
		assert_eq!(proposal.status, ProposalStatus::ReadyToFinalize(true));
	}

	#[benchmark]
	fn create_proof(
		d: Linear<1, { T::VaultDescriptionMaxLen::get() }>,
		s: Linear<2, { T::PSBTMaxLen::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
		let vault_id = setup_vault_sized::<T>(
			owner.clone(),
			0,
			T::XPubLen::get(),
			d,
			T::MaxCosignersPerVault::get(),
		);
		let message = generate_description_sized::<T>(1, d);
		let psbt = generate_psbt_sized::<T>(2, s);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), vault_id, message, psbt);
		assert!(BitcoinVaults::<T>::proof_of_reserve(vault_id).is_some());
	}

	#[benchmark]
	fn save_proof_psbt(s: Linear<2, { T::PSBTMaxLen::get() }>) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
		let vault_id = setup_vault_sized::<T>(
			owner.clone(),
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			T::MaxCosignersPerVault::get(),
		);
		assert_ok!(BitcoinVaults::<T>::create_proof(
			RawOrigin::Signed(owner.clone()).into(),
			vault_id,
			generate_description_sized::<T>(0, T::VaultDescriptionMaxLen::get()),
			generate_psbt_sized::<T>(1, s)
		));
		let cosigner = sign_proof_sized::<T>(vault_id, s, true, false).unwrap();
		let psbt = generate_psbt_sized::<T>(2, s);
		#[extrinsic_call]
		_(RawOrigin::Signed(cosigner.clone()), vault_id, psbt);
		let proof = BitcoinVaults::<T>::proof_of_reserve(vault_id).unwrap();
		let vault = BitcoinVaults::<T>::vaults(vault_id).unwrap();
		assert_eq!(vault.cosigners.len(), proof.signed_psbts.len());
	}

	#[benchmark]
	fn finalize_proof(s: Linear<2, { T::PSBTMaxLen::get() }>) {
		let owner: T::AccountId = account("owner", 0, SEED);
		setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
		let vault_id = setup_vault_sized::<T>(
			owner.clone(),
			0,
			T::XPubLen::get(),
			T::VaultDescriptionMaxLen::get(),
			T::MaxCosignersPerVault::get(),
		);
		assert_ok!(BitcoinVaults::<T>::create_proof(
			RawOrigin::Signed(owner.clone()).into(),
			vault_id,
			generate_description_sized::<T>(0, T::VaultDescriptionMaxLen::get()),
			generate_psbt_sized::<T>(1, s)
		));
		sign_proof_sized::<T>(vault_id, s, true, false).unwrap();
		let psbt = generate_psbt_sized::<T>(2, s);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner.clone()), vault_id, psbt);
		let proof = BitcoinVaults::<T>::proof_of_reserve(vault_id).unwrap();
		assert_eq!(proof.status, ProposalStatus::Broadcasted);
	}

	// #[benchmark]
	// fn ocw_insert_descriptors(
	// 	o: Linear<2, { T::OutputDescriptorMaxLen::get() }>,
	// 	v: Linear<1, { 1000 }>,
	// ) {
	// 	let owner: T::AccountId = account("owner", 0, SEED);
	// 	setup_xpub_sized::<T>(owner.clone(), 0, T::XPubLen::get());
	// 	setup_vaults_sized::<T>(
	// 		owner.clone(),
	// 		v,
	// 		T::XPubLen::get(),
	// 		T::VaultDescriptionMaxLen::get(),
	// 		T::MaxCosignersPerVault::get(),
	// 	);
	// 	let payload = generate_insert_output_descriptors_payload_sized(o);

	// 	#[extrinsic_call]
	// 	_(RawOrigin::Unsigned, vault_id, psbt);
	// 	let proof = BitcoinVaults::<T>::proof_of_reserve(vault_id).unwrap();
	// 	assert_eq!(proof.status, ProposalStatus::Broadcasted);
	// }

	impl_benchmark_test_suite! {
		BitcoinVaults,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
