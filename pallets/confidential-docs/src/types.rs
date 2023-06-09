//! Defines the types required by the confidential docs pallet
use super::*;
use frame_support::pallet_prelude::*;

/// Defines the type used by fields that store an IPFS CID
pub type CID = BoundedVec<u8,ConstU32<100>>;
/// Defines the type used by fields that store a public key
pub type PublicKey = [u8;32];
/// Defines the type used by fields that store a UserId
pub type UserId = [u8;32];
/// Defines the type used by fields that store a document name
pub type DocName<T> = BoundedVec<u8,<T as Config>::DocNameMaxLen>;
/// Defines the type used by fields that store a document description
pub type DocDesc<T> = BoundedVec<u8,<T as Config>::DocDescMaxLen>;

/// User vault, the vault stores the cipher private key used to cipher the user documents.
/// The way the user vault is ciphered depends on the login method used by the user
#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct Vault<T: Config>{
    /// IPFS CID where the vault data is stored 
    pub cid: CID,
    /// Owner of the vault
    pub owner: T::AccountId,
}

/// Owned confidential document
#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct OwnedDoc<T: Config>{
    /// IPFS CID where the document data is stored 
    pub cid: CID,
    /// User provided name for the document
    pub name: DocName<T>,
    /// User provided description for the document
    pub description: DocDesc<T>,
    /// Owner of the document
    pub owner: T::AccountId,
}

/// Shared confidential document
#[derive(CloneNoBound,Encode, Decode, RuntimeDebugNoBound, Default, TypeInfo, MaxEncodedLen, PartialEq)]
#[scale_info(skip_type_params(T))]
#[codec(mel_bound())]
pub struct SharedDoc<T: Config>{
    /// IPFS CID where the document data is stored 
    pub cid: CID,
    /// User provided name for the document
    pub name: DocName<T>,
    /// User provided description for the document
    pub description: DocDesc<T>,
    /// User that shared the document
    pub from: T::AccountId,
    /// User to which the document was shared
    pub to: T::AccountId,
}