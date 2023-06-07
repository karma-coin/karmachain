use sp_runtime::{
	app_crypto::{app_crypto, sr25519},
	KeyTypeId, MultiSignature, MultiSigner,
};

/// Defines application identifier for crypto keys of this module.
///
/// Every module that deals with signatures needs to declare its unique identifier for
/// its crypto keys.
/// When offchain worker is signing transactions it's going to request keys of type
/// `KeyTypeId` from the keystore and use the ones it finds to sign the transaction.
/// The keys can be inserted manually via RPC (see `author_insertKey`).
pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"rewa");

// Based on the above `KeyTypeId` we need to generate a pallet-specific crypto type wrappers.
// We can use from supported crypto kinds (`sr25519`, `ed25519` and `ecdsa`) and augment
// the types with this pallet-specific identifier.
app_crypto!(sr25519, KEY_TYPE);

pub struct AuthorityId;

impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthorityId {
	type RuntimeAppPublic = Public;
	type GenericPublic = sr25519::Public;
	type GenericSignature = sr25519::Signature;
}
