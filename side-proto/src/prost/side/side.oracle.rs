// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// define how many block headers keep on side chain
    #[prost(uint32, tag = "1")]
    pub keep_bitcoin_blocks: u32,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// Oracle Price from Price Extention
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OraclePrice {
    /// id
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for OraclePrice {
    const NAME: &'static str = "OraclePrice";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// Bitcoin Block Header From Price Extention
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    #[prost(int32, tag = "1")]
    pub version: i32,
    #[prost(string, tag = "2")]
    pub hash: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub height: i32,
    #[prost(string, tag = "4")]
    pub previous_block_hash: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub merkle_root: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub nonce: u64,
    #[prost(string, tag = "7")]
    pub bits: ::prost::alloc::string::String,
    #[prost(int64, tag = "8")]
    pub time: i64,
    #[prost(int32, tag = "9")]
    pub ntx: i32,
}
impl ::prost::Name for BlockHeader {
    const NAME: &'static str = "BlockHeader";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// Oracle Vote Extenstion
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct OracleVoteExtension {
    #[prost(int64, tag = "1")]
    pub height: i64,
    #[prost(map = "string, string", tag = "2")]
    pub prices:
        ::std::collections::HashMap<::prost::alloc::string::String, ::prost::alloc::string::String>,
    #[prost(message, repeated, tag = "3")]
    pub blocks: ::prost::alloc::vec::Vec<BlockHeader>,
}
impl ::prost::Name for OracleVoteExtension {
    const NAME: &'static str = "OracleVoteExtension";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// GenesisState defines the lending module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub prices: ::prost::alloc::vec::Vec<OraclePrice>,
    #[prost(message, repeated, tag = "3")]
    pub blocks: ::prost::alloc::vec::Vec<BlockHeader>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPriceBySymbolRequest {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryGetPriceBySymbolRequest {
    const NAME: &'static str = "QueryGetPriceBySymbolRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryGetPriceBySymbolResponse {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryGetPriceBySymbolResponse {
    const NAME: &'static str = "QueryGetPriceBySymbolResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryPoolsRequest is request type for the Query/Pools RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListPricesRequest {}
impl ::prost::Name for QueryListPricesRequest {
    const NAME: &'static str = "QueryListPricesRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryPoolsResponse is response type for the Query/Pools RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryListPricesResponse {
    #[prost(message, repeated, tag = "1")]
    pub prices: ::prost::alloc::vec::Vec<OraclePrice>,
}
impl ::prost::Name for QueryListPricesResponse {
    const NAME: &'static str = "QueryListPricesResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    /// params holds all the parameters of this module.
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryChainTipRequest is request type for the Query/ChainTip RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChainTipRequest {}
impl ::prost::Name for QueryChainTipRequest {
    const NAME: &'static str = "QueryChainTipRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryChainTipResponse is response type for the Query/ChainTip RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryChainTipResponse {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub height: u64,
}
impl ::prost::Name for QueryChainTipResponse {
    const NAME: &'static str = "QueryChainTipResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryBlockHeaderByHeightRequest is the request type for the Query/BlockHeaderByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHeightRequest {
    #[prost(uint64, tag = "1")]
    pub height: u64,
}
impl ::prost::Name for QueryBlockHeaderByHeightRequest {
    const NAME: &'static str = "QueryBlockHeaderByHeightRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryBlockHeaderByHeightResponse is the response type for the Query/BlockHeaderByHeight RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHeightResponse {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
impl ::prost::Name for QueryBlockHeaderByHeightResponse {
    const NAME: &'static str = "QueryBlockHeaderByHeightResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryBlockHeaderByHashRequest is the request type for the Query/BlockHeaderByHash RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHashRequest {
    #[prost(string, tag = "1")]
    pub hash: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryBlockHeaderByHashRequest {
    const NAME: &'static str = "QueryBlockHeaderByHashRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryBlockHeaderByHashResponse is the response type for the Query/BlockHeaderByHash RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBlockHeaderByHashResponse {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
impl ::prost::Name for QueryBlockHeaderByHashResponse {
    const NAME: &'static str = "QueryBlockHeaderByHashResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryBestBlockHeaderRequest is the request type for the Query/BestBlockHeader RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBestBlockHeaderRequest {}
impl ::prost::Name for QueryBestBlockHeaderRequest {
    const NAME: &'static str = "QueryBestBlockHeaderRequest";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// QueryBestBlockHeaderResponse is the response type for the Query/BestBlockHeader RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBestBlockHeaderResponse {
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
}
impl ::prost::Name for QueryBestBlockHeaderResponse {
    const NAME: &'static str = "QueryBestBlockHeaderResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// MsgUpdateParams is the Msg/UpdateParams request type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParams {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// params defines the x/oracle parameters to be updated.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
/// MsgUpdateParamsResponse defines the Msg/UpdateParams response type.
///
/// Since: cosmos-sdk 0.47
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateParamsResponse {}
impl ::prost::Name for MsgUpdateParamsResponse {
    const NAME: &'static str = "MsgUpdateParamsResponse";
    const PACKAGE: &'static str = "side.oracle";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.oracle.{}", Self::NAME)
    }
}
include!("side.oracle.serde.rs");
include!("side.oracle.tonic.rs");
// @@protoc_insertion_point(module)
