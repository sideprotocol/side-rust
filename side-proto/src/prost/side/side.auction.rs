// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(string, tag = "3")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub bid_price: i64,
    #[prost(message, optional, tag = "5")]
    pub bid_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub bidded_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(enumeration = "BidStatus", tag = "7")]
    pub status: i32,
}
impl ::prost::Name for Bid {
    const NAME: &'static str = "Bid";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auction {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub agency: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub deposited_asset: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "6")]
    pub liquidated_price: i64,
    #[prost(message, optional, tag = "7")]
    pub liquidated_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(int64, tag = "8")]
    pub expected_value: i64,
    #[prost(int64, tag = "9")]
    pub bidded_value: i64,
    #[prost(int64, tag = "10")]
    pub bidded_amount: i64,
    #[prost(string, tag = "11")]
    pub liquidation_cet: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub payment_tx: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub payment_tx_id: ::prost::alloc::string::String,
    #[prost(enumeration = "AuctionStatus", tag = "14")]
    pub status: i32,
}
impl ::prost::Name for Auction {
    const NAME: &'static str = "Auction";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    Bitcoin = 0,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Bitcoin => "ASSET_TYPE_BITCOIN",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASSET_TYPE_BITCOIN" => Some(Self::Bitcoin),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuctionStatus {
    Unspecified = 0,
    Open = 1,
    Closed = 2,
    Settled = 3,
}
impl AuctionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuctionStatus::Unspecified => "AUCTION_STATUS_UNSPECIFIED",
            AuctionStatus::Open => "AUCTION_STATUS_OPEN",
            AuctionStatus::Closed => "AUCTION_STATUS_CLOSED",
            AuctionStatus::Settled => "AUCTION_STATUS_SETTLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUCTION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "AUCTION_STATUS_OPEN" => Some(Self::Open),
            "AUCTION_STATUS_CLOSED" => Some(Self::Closed),
            "AUCTION_STATUS_SETTLED" => Some(Self::Settled),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidStatus {
    Unspecified = 0,
    Bidding = 1,
    Accepted = 2,
    Rejected = 3,
    Cancelled = 4,
}
impl BidStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BidStatus::Unspecified => "BID_STATUS_UNSPECIFIED",
            BidStatus::Bidding => "BID_STATUS_BIDDING",
            BidStatus::Accepted => "BID_STATUS_ACCEPTED",
            BidStatus::Rejected => "BID_STATUS_REJECTED",
            BidStatus::Cancelled => "BID_STATUS_CANCELLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BID_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "BID_STATUS_BIDDING" => Some(Self::Bidding),
            "BID_STATUS_ACCEPTED" => Some(Self::Accepted),
            "BID_STATUS_REJECTED" => Some(Self::Rejected),
            "BID_STATUS_CANCELLED" => Some(Self::Cancelled),
            _ => None,
        }
    }
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag = "1")]
    pub price_drop_period: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    #[prost(uint32, tag = "2")]
    pub initial_discount: u32,
    #[prost(uint32, tag = "3")]
    pub fee_rate: u32,
    #[prost(uint64, tag = "4")]
    pub min_bid_amount: u64,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// GenesisState defines the auctioin module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
    #[prost(message, repeated, tag = "3")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
    const NAME: &'static str = "QueryParamsResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryAuctionRequest is request type for the Query/Auction RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryAuctionRequest {
    const NAME: &'static str = "QueryAuctionRequest";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryAuctionResponse is response type for the Query/Auction RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionResponse {
    #[prost(message, optional, tag = "1")]
    pub auction: ::core::option::Option<Auction>,
}
impl ::prost::Name for QueryAuctionResponse {
    const NAME: &'static str = "QueryAuctionResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryAuctionsRequest is request type for the Query/Auctions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionsRequest {
    #[prost(enumeration = "AuctionStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAuctionsRequest {
    const NAME: &'static str = "QueryAuctionsRequest";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryAuctionsResponse is response type for the Query/Auctions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionsResponse {
    #[prost(message, repeated, tag = "1")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAuctionsResponse {
    const NAME: &'static str = "QueryAuctionsResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryBidRequest is request type for the Query/Bid RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryBidRequest {
    const NAME: &'static str = "QueryBidRequest";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryBidResponse is response type for the Query/Bid RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidResponse {
    #[prost(message, optional, tag = "1")]
    pub bid: ::core::option::Option<Bid>,
}
impl ::prost::Name for QueryBidResponse {
    const NAME: &'static str = "QueryBidResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryBidsRequest is request type for the Query/Bids RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsRequest {
    #[prost(uint64, tag = "1")]
    pub auction_id: u64,
    #[prost(enumeration = "BidStatus", tag = "2")]
    pub status: i32,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBidsRequest {
    const NAME: &'static str = "QueryBidsRequest";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// QueryBidsResponse is response type for the Query/Bids RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsResponse {
    #[prost(message, repeated, tag = "1")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryBidsResponse {
    const NAME: &'static str = "QueryBidsResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// MsgBid defines the Msg/Bid request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBid {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(int64, tag = "3")]
    pub price: i64,
    #[prost(message, optional, tag = "4")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgBid {
    const NAME: &'static str = "MsgBid";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// MsgBidResponse defines the Msg/Bid response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBidResponse {}
impl ::prost::Name for MsgBidResponse {
    const NAME: &'static str = "MsgBidResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// MsgCancelBid defines the Msg/CancelBid request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBid {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
impl ::prost::Name for MsgCancelBid {
    const NAME: &'static str = "MsgCancelBid";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// MsgCancelBidResponse defines the Msg/CancelBid response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelBidResponse {}
impl ::prost::Name for MsgCancelBidResponse {
    const NAME: &'static str = "MsgCancelBidResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// MsgSubmitPaymentTransactionSignatures defines the Msg/SubmitPaymentTransactionSignatures request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitPaymentTransactionSignatures {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub auction_id: u64,
    #[prost(string, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitPaymentTransactionSignatures {
    const NAME: &'static str = "MsgSubmitPaymentTransactionSignatures";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
/// MsgSubmitPaymentTransactionSignaturesResponse defines the Msg/SubmitPaymentTransactionSignatures response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitPaymentTransactionSignaturesResponse {}
impl ::prost::Name for MsgSubmitPaymentTransactionSignaturesResponse {
    const NAME: &'static str = "MsgSubmitPaymentTransactionSignaturesResponse";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
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
    /// params defines the x/btcbridge parameters to be updated.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.auction";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.auction.{}", Self::NAME)
    }
}
include!("side.auction.serde.rs");
include!("side.auction.tonic.rs");
// @@protoc_insertion_point(module)
