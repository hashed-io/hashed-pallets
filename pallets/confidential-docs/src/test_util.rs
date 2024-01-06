use crate::{types::*, Config};
use codec::Encode;
use scale_info::{self, prelude::*};
use sp_io::hashing::blake2_256;

pub fn generate_user_id(id: u8) -> UserId {
	format!("user id: {}", id).using_encoded(blake2_256)
}

pub fn generate_public_key(id: u8) -> PublicKey {
	format!("public key: {}", id).using_encoded(blake2_256)
}

pub fn generate_cid(id: u8) -> CID {
	generate_cid_sized(id, 3)
}

pub fn generate_cid_sized(id: u8, size: u32) -> CID {
	generate_vector(1, id, size).try_into().unwrap()
}

pub fn generate_doc_name<T: Config>(id: u8) -> DocName<T> {
	generate_doc_name_sized::<T>(id, 4)
}

pub fn generate_doc_name_sized<T: Config>(id: u8, size: u32) -> DocName<T> {
	generate_vector(2, id, size).try_into().unwrap()
}

pub fn generate_doc_desc<T: Config>(id: u8) -> DocDesc<T> {
	generate_doc_desc_sized::<T>(id, 5)
}

pub fn generate_doc_desc_sized<T: Config>(id: u8, size: u32) -> DocDesc<T> {
	generate_vector(3, id, size).try_into().unwrap()
}

pub fn generate_group_name<T: Config>(id: u8) -> GroupName<T> {
	generate_group_name_sized::<T>(id, 3)
}

pub fn generate_group_name_sized<T: Config>(id: u8, size: u32) -> GroupName<T> {
	generate_vector(4, id, size).try_into().unwrap()
}

pub fn generate_vector(prefix: u8, id: u8, size: u32) -> scale_info::prelude::vec::Vec<u8> {
	assert!(size > 0, "vector size must be greater than 0");
	let mut v = vec![id; size as usize];
	v[0] = prefix;
	v
}

// fn generate_doc_name(id: &str) -> DocName<T> {
// 	format!("doc name:{}", id).encode().try_into().unwrap()
// }

// fn generate_doc_desc(id: &str) -> DocDesc<T> {
// 	format!("doc desc:{}", id).encode().try_into().unwrap()
// }

// fn generate_group_name(id: &str) -> GroupName<T> {
// 	format!("group name:{}", id).encode().try_into().unwrap()
// }

pub fn generate_vault_sized(id: u8, size: u32) -> (UserId, PublicKey, CID) {
	(generate_user_id(id), generate_public_key(id), generate_cid_sized(id, size))
}

pub fn generate_owned_doc<T: Config>(id: u8, owner: T::AccountId) -> OwnedDoc<T> {
	generate_owned_doc_sized(id, owner, 5, 5, 5)
}

pub fn generate_owned_doc_sized<T: Config>(
	id: u8,
	owner: T::AccountId,
	cid_size: u32,
	name_size: u32,
	desc_size: u32,
) -> OwnedDoc<T> {
	OwnedDoc {
		cid: generate_cid_sized(id, cid_size),
		name: generate_doc_name_sized::<T>(id, name_size),
		description: generate_doc_desc_sized::<T>(id, desc_size),
		owner,
	}
}

pub fn generate_shared_doc<T: Config>(
	id: u8,
	from: T::AccountId,
	to: T::AccountId,
) -> SharedDoc<T> {
	generate_shared_doc_sized(id, from, to, 5, 5, 5)
}

pub fn generate_shared_doc_sized<T: Config>(
	id: u8,
	from: T::AccountId,
	to: T::AccountId,
	cid_size: u32,
	name_size: u32,
	desc_size: u32,
) -> SharedDoc<T> {
	SharedDoc {
		cid: generate_cid_sized(id, cid_size),
		name: generate_doc_name_sized::<T>(id, name_size),
		description: generate_doc_desc_sized::<T>(id, desc_size),
		from,
		to,
	}
}
