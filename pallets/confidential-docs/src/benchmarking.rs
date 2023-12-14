//! Confidential Docs pallet benchmarking.

#![cfg(feature = "runtime-benchmarks")]

use super::*;
use crate::{
	test_util::*,
	types::{Vault, *},
	Pallet as ConfidentialDocs,
};

use frame_benchmarking::v2::*;
use frame_support::{assert_ok, traits::Get};
use frame_system::RawOrigin;

const SEED: u32 = 0;

#[benchmarks(where T: Config)]
mod benchmarks {
	use super::*;

	#[benchmark]
	fn set_vault(c: Linear<2, { CIDSize::get() }>) {
		let owner: T::AccountId = account("owner", 0, SEED);
		let (user_id, public_key, cid) = generate_vault_sized(1, c);
		let vault = Vault::<T> { cid: cid.clone(), owner: owner.clone() };

		#[extrinsic_call]
		_(RawOrigin::Signed(owner), user_id, public_key, cid.clone());
		assert_eq!(ConfidentialDocs::vaults(user_id), Some(vault.clone()));
	}

	#[benchmark]
	fn set_owned_document(
		c: Linear<2, { CIDSize::get() }>,
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		o: Linear<1, { T::MaxOwnedDocs::get() - 1 }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		let (user_id, public_key, cid) = generate_vault_sized(1, 5);
		assert_ok!(ConfidentialDocs::<T>::set_vault(
			RawOrigin::Signed(owner.clone()).into(),
			user_id,
			public_key,
			cid.clone(),
		));
		// Add "o" docs to owned docs
		for i in 1..=o {
			let doc = generate_owned_doc_sized(i as u8, owner.clone(), c, n, d);
			assert_ok!(ConfidentialDocs::<T>::set_owned_document(
				RawOrigin::Signed(owner.clone()).into(),
				doc.clone()
			));
		}
		let doc = generate_owned_doc_sized((o + 1) as u8, owner.clone(), c, n, d);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner), doc.clone());
		assert_eq!(ConfidentialDocs::owned_docs(&doc.cid), Some(doc.clone()));
	}

	#[benchmark]
	fn remove_owned_document(
		c: Linear<2, { CIDSize::get() }>,
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		o: Linear<1, { T::MaxOwnedDocs::get() }>,
	) {
		let owner: T::AccountId = account("owner", 0, SEED);
		let (user_id, public_key, cid) = generate_vault_sized(1, 5);
		assert_ok!(ConfidentialDocs::<T>::set_vault(
			RawOrigin::Signed(owner.clone()).into(),
			user_id,
			public_key,
			cid.clone(),
		));
		// Add "o" docs to owned docs
		for i in 1..=o {
			let doc = generate_owned_doc_sized(i as u8, owner.clone(), c, n, d);
			assert_ok!(ConfidentialDocs::<T>::set_owned_document(
				RawOrigin::Signed(owner.clone()).into(),
				doc.clone()
			));
		}
		let cid = generate_cid_sized(o as u8, c);
		#[extrinsic_call]
		_(RawOrigin::Signed(owner), cid.clone());
		assert_eq!(ConfidentialDocs::<T>::owned_docs(cid), None);
	}

	#[benchmark]
	fn share_document(
		c: Linear<2, { CIDSize::get() }>, /* starts in to so that document type id and doc id
		                                   * fit in the vector */
		n: Linear<{ T::DocNameMinLen::get() }, { T::DocNameMaxLen::get() }>,
		d: Linear<{ T::DocDescMinLen::get() }, { T::DocDescMaxLen::get() }>,
		s: Linear<1, { T::MaxSharedFromDocs::get() - 1 }>,
	) {
		let from: T::AccountId = account("from", 0, SEED);
		let to: T::AccountId = account("to", 0, SEED);
		// Setup from vault
		let (user_id, public_key, cid) = generate_vault_sized(1, 5);
		assert_ok!(ConfidentialDocs::<T>::set_vault(
			RawOrigin::Signed(from.clone()).into(),
			user_id,
			public_key,
			cid.clone(),
		));
		// Setup to vault
		let (user_id, public_key, cid) = generate_vault_sized(2, 5);
		assert_ok!(ConfidentialDocs::<T>::set_vault(
			RawOrigin::Signed(to.clone()).into(),
			user_id,
			public_key,
			cid.clone(),
		));
		// Add "s" docs to shared docs
		for i in 1..=s {
			let doc = generate_shared_doc_sized(i as u8, from.clone(), to.clone(), c, n, d);
			assert_ok!(ConfidentialDocs::<T>::share_document(
				RawOrigin::Signed(from.clone()).into(),
				doc.clone()
			));
		}
		let doc = generate_shared_doc_sized((s + 1) as u8, from.clone(), to.clone(), c, n, d);
		#[extrinsic_call]
		_(RawOrigin::Signed(from), doc.clone());
		assert_eq!(ConfidentialDocs::shared_docs(&doc.cid), Some(doc.clone()));
	}

	impl_benchmark_test_suite! {
		ConfidentialDocs,
		crate::mock::new_test_ext(),
		crate::mock::Test,
	}
}
