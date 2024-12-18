// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcOracle {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub participants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "4")]
    pub threshold: u32,
    #[prost(string, tag = "5")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(enumeration = "DlcOracleStatus", tag = "7")]
    pub status: i32,
    #[prost(uint64, tag = "8")]
    pub nonce_index: u64,
}
impl ::prost::Name for DlcOracle {
    const NAME: &'static str = "DLCOracle";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcNonce {
    #[prost(uint64, tag = "1")]
    pub index: u64,
    #[prost(string, tag = "2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub oracle_pubkey: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for DlcNonce {
    const NAME: &'static str = "DLCNonce";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcAnnouncement {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub oracle_event: ::core::option::Option<DlcEvent>,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
    #[prost(enumeration = "AnnouncementStatus", tag = "4")]
    pub status: i32,
}
impl ::prost::Name for DlcAnnouncement {
    const NAME: &'static str = "DLCAnnouncement";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcAttestation {
    #[prost(uint64, tag = "1")]
    pub announcement_id: u64,
    #[prost(message, optional, tag = "2")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(string, tag = "3")]
    pub outcome: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for DlcAttestation {
    const NAME: &'static str = "DLCAttestation";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcEvent {
    #[prost(uint32, tag = "1")]
    pub maturity_epoch: u32,
    #[prost(string, tag = "2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub index: u64,
    #[prost(string, tag = "4")]
    pub descriptor: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub pubkey: ::prost::alloc::string::String,
}
impl ::prost::Name for DlcEvent {
    const NAME: &'static str = "DLCEvent";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AnnouncementStatus {
    AnnouncementUnspecified = 0,
    AnnouncementPending = 1,
    AnnouncementReady = 2,
}
impl AnnouncementStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AnnouncementStatus::AnnouncementUnspecified => "Announcement_Unspecified",
            AnnouncementStatus::AnnouncementPending => "Announcement_Pending",
            AnnouncementStatus::AnnouncementReady => "Announcement_Ready",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Announcement_Unspecified" => Some(Self::AnnouncementUnspecified),
            "Announcement_Pending" => Some(Self::AnnouncementPending),
            "Announcement_Ready" => Some(Self::AnnouncementReady),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DlcOracleStatus {
    OracleStatusPending = 0,
    OracleStatusEnable = 1,
    OracleStatusDisable = 2,
}
impl DlcOracleStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DlcOracleStatus::OracleStatusPending => "Oracle_Status_Pending",
            DlcOracleStatus::OracleStatusEnable => "Oracle_status_Enable",
            DlcOracleStatus::OracleStatusDisable => "Oracle_status_Disable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Oracle_Status_Pending" => Some(Self::OracleStatusPending),
            "Oracle_status_Enable" => Some(Self::OracleStatusEnable),
            "Oracle_status_Disable" => Some(Self::OracleStatusDisable),
            _ => None,
        }
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PriceInterval {
    #[prost(string, tag = "1")]
    pub price_pair: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub interval: i32,
}
impl ::prost::Name for PriceInterval {
    const NAME: &'static str = "PriceInterval";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(uint32, tag = "1")]
    pub nonce_queue_size: u32,
    #[prost(message, repeated, tag = "2")]
    pub price_interval: ::prost::alloc::vec::Vec<PriceInterval>,
    #[prost(string, repeated, tag = "3")]
    pub recommended_oracles: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// GenesisState defines the dlc module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub announcements: ::prost::alloc::vec::Vec<DlcAnnouncement>,
    #[prost(message, repeated, tag = "3")]
    pub attestations: ::prost::alloc::vec::Vec<DlcAttestation>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOraclesRequest {
    #[prost(enumeration = "DlcOracleStatus", tag = "1")]
    pub status: i32,
}
impl ::prost::Name for QueryOraclesRequest {
    const NAME: &'static str = "QueryOraclesRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryOraclesResponse {
    #[prost(message, repeated, tag = "1")]
    pub oracles: ::prost::alloc::vec::Vec<DlcOracle>,
}
impl ::prost::Name for QueryOraclesResponse {
    const NAME: &'static str = "QueryOraclesResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCountNoncesRequest {}
impl ::prost::Name for QueryCountNoncesRequest {
    const NAME: &'static str = "QueryCountNoncesRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryCountNoncesResponse is response type for the Query/CountNonces RPC method.
/// counts should use the same order as recommende oracles in Params
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCountNoncesResponse {
    /// qty of nonce in the cache queue
    #[prost(uint32, repeated, tag = "1")]
    pub counts: ::prost::alloc::vec::Vec<u32>,
}
impl ::prost::Name for QueryCountNoncesResponse {
    const NAME: &'static str = "QueryCountNoncesResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoncesRequest {}
impl ::prost::Name for QueryNoncesRequest {
    const NAME: &'static str = "QueryNoncesRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoncesResponse {
    #[prost(message, repeated, tag = "1")]
    pub nonces: ::prost::alloc::vec::Vec<DlcNonce>,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryNoncesResponse {
    const NAME: &'static str = "QueryNoncesResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryAnnouncementsRequest is request type for the Query/Announcements RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnouncementsRequest {
    #[prost(enumeration = "AnnouncementStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAnnouncementsRequest {
    const NAME: &'static str = "QueryAnnouncementsRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryAnnouncementsResponse is response type for the Query/Announcements RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAnnouncementsResponse {
    #[prost(message, repeated, tag = "1")]
    pub announcements: ::prost::alloc::vec::Vec<DlcAnnouncement>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAnnouncementsResponse {
    const NAME: &'static str = "QueryAnnouncementsResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryPriceRequest is request type for the Query/Price RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPriceRequest {
    #[prost(string, tag = "1")]
    pub symbol: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPriceRequest {
    const NAME: &'static str = "QueryPriceRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryPriceResponse is response type for the Query/Price RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPriceResponse {
    #[prost(uint32, tag = "1")]
    pub price: u32,
}
impl ::prost::Name for QueryPriceResponse {
    const NAME: &'static str = "QueryPriceResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitOraclePubkey {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitOraclePubkey {
    const NAME: &'static str = "MsgSubmitOraclePubkey";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitOraclePubkeyResponse {}
impl ::prost::Name for MsgSubmitOraclePubkeyResponse {
    const NAME: &'static str = "MsgSubmitOraclePubkeyResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitNonce {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitNonce {
    const NAME: &'static str = "MsgSubmitNonce";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitNonceResponse {}
impl ::prost::Name for MsgSubmitNonceResponse {
    const NAME: &'static str = "MsgSubmitNonceResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAnnouncementNonce {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub announcement_id: u64,
    #[prost(string, tag = "3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitAnnouncementNonce {
    const NAME: &'static str = "MsgSubmitAnnouncementNonce";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAnnouncementNonceResponse {}
impl ::prost::Name for MsgSubmitAnnouncementNonceResponse {
    const NAME: &'static str = "MsgSubmitAnnouncementNonceResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAttestation {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub announcement_id: u64,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitAttestation {
    const NAME: &'static str = "MsgSubmitAttestation";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitAttestationResponse {}
impl ::prost::Name for MsgSubmitAttestationResponse {
    const NAME: &'static str = "MsgSubmitAttestationResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
include!("side.dlc.serde.rs");
include!("side.dlc.tonic.rs");
// @@protoc_insertion_point(module)
