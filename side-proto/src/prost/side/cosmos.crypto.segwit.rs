// @generated
/// PubKey defines a secp256k1 public key for SegWit.
/// Key is the compressed form of the pubkey. The first byte depends is a 0x02 byte
/// if the y-coordinate is the lexicographically largest of the two associated with
/// the x-coordinate. Otherwise the first byte is a 0x03.
/// This prefix is followed with the x-coordinate.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PubKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PubKey {
    const NAME: &'static str = "PubKey";
    const PACKAGE: &'static str = "cosmos.crypto.segwit";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.segwit.{}", Self::NAME)
    }
}
/// PrivKey defines a SegWit private key.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PrivKey {
    #[prost(bytes = "vec", tag = "1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
}
impl ::prost::Name for PrivKey {
    const NAME: &'static str = "PrivKey";
    const PACKAGE: &'static str = "cosmos.crypto.segwit";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("cosmos.crypto.segwit.{}", Self::NAME)
    }
}
include!("cosmos.crypto.segwit.serde.rs");
// @@protoc_insertion_point(module)
