// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bid {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(uint64, tag="2")]
    pub auction_id: u64,
    #[prost(string, tag="3")]
    pub bidder: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub bid_price: i64,
    #[prost(message, optional, tag="5")]
    pub bid_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(enumeration="BidStatus", tag="6")]
    pub status: i32,
}
impl ::prost::Name for Bid {
const NAME: &'static str = "Bid";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Auction {
    #[prost(uint64, tag="1")]
    pub id: u64,
    #[prost(message, optional, tag="2")]
    pub deposited_asset: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag="3")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub liquidated_price: i64,
    #[prost(message, optional, tag="5")]
    pub liquidated_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(int64, tag="6")]
    pub expected_value: i64,
    #[prost(int64, tag="7")]
    pub bidded_value: i64,
    #[prost(string, tag="8")]
    pub payment_tx_id: ::prost::alloc::string::String,
    #[prost(enumeration="AuctionStatus", tag="9")]
    pub status: i32,
}
impl ::prost::Name for Auction {
const NAME: &'static str = "Auction";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
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
            AssetType::Bitcoin => "Bitcoin",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Bitcoin" => Some(Self::Bitcoin),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuctionStatus {
    AuctionOpen = 0,
    AuctionClose = 1,
}
impl AuctionStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuctionStatus::AuctionOpen => "AuctionOpen",
            AuctionStatus::AuctionClose => "AuctionClose",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AuctionOpen" => Some(Self::AuctionOpen),
            "AuctionClose" => Some(Self::AuctionClose),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BidStatus {
    Bidding = 0,
    Accepted = 1,
    Rejected = 2,
}
impl BidStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BidStatus::Bidding => "Bidding",
            BidStatus::Accepted => "Accepted",
            BidStatus::Rejected => "Rejected",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Bidding" => Some(Self::Bidding),
            "Accepted" => Some(Self::Accepted),
            "Rejected" => Some(Self::Rejected),
            _ => None,
        }
    }
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(message, optional, tag="1")]
    pub price_drop_period: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    #[prost(uint32, tag="2")]
    pub initial_discount: u32,
    #[prost(uint32, tag="3")]
    pub fee_rate: u32,
    #[prost(uint64, tag="4")]
    pub min_bid_amount: u64,
}
impl ::prost::Name for Params {
const NAME: &'static str = "Params";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// GenesisState defines the auctioin module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag="2")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
    #[prost(message, repeated, tag="3")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
}
impl ::prost::Name for GenesisState {
const NAME: &'static str = "GenesisState";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {
}
impl ::prost::Name for QueryParamsRequest {
const NAME: &'static str = "QueryParamsRequest";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// QueryParamsResponse is response type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsResponse {
    #[prost(message, optional, tag="1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for QueryParamsResponse {
const NAME: &'static str = "QueryParamsResponse";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// QueryAuctionsRequest is request type for the Query/Auctions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionsRequest {
    #[prost(enumeration="AuctionStatus", tag="1")]
    pub status: i32,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAuctionsRequest {
const NAME: &'static str = "QueryAuctionsRequest";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// QueryAuctionsResponse is response type for the Query/Auctions RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAuctionsResponse {
    #[prost(message, repeated, tag="1")]
    pub auctions: ::prost::alloc::vec::Vec<Auction>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAuctionsResponse {
const NAME: &'static str = "QueryAuctionsResponse";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// QueryBidsRequest is request type for the Query/Bids RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsRequest {
    #[prost(enumeration="BidStatus", tag="1")]
    pub status: i32,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBidsRequest {
const NAME: &'static str = "QueryBidsRequest";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// QueryBidsResponse is response type for the Query/Bids RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryBidsResponse {
    #[prost(message, repeated, tag="1")]
    pub bids: ::prost::alloc::vec::Vec<Bid>,
    #[prost(message, optional, tag="2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryBidsResponse {
const NAME: &'static str = "QueryBidsResponse";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// MsgBid defines the Msg/Bid request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBid {
    #[prost(string, tag="1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag="2")]
    pub auction_id: u64,
    #[prost(int64, tag="3")]
    pub price: i64,
    #[prost(message, optional, tag="4")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgBid {
const NAME: &'static str = "MsgBid";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
/// MsgBidResponse defines the Msg/Bid response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgBidResponse {
}
impl ::prost::Name for MsgBidResponse {
const NAME: &'static str = "MsgBidResponse";
const PACKAGE: &'static str = "side.auction";
fn full_name() -> ::prost::alloc::string::String {
                ::prost::alloc::format!("side.auction.{}", Self::NAME)
            }}
include!("side.auction.serde.rs");
include!("side.auction.tonic.rs");
// @@protoc_insertion_point(module)