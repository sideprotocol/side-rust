// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcOracle {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub participants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "4")]
    pub threshold: u32,
    #[prost(string, tag = "5")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(uint64, tag = "6")]
    pub nonce_index: u64,
    #[prost(message, optional, tag = "7")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "DlcOracleStatus", tag = "8")]
    pub status: i32,
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
pub struct Dcm {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub desc: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub participants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "4")]
    pub threshold: u32,
    #[prost(string, tag = "5")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "DcmStatus", tag = "7")]
    pub status: i32,
}
impl ::prost::Name for Dcm {
    const NAME: &'static str = "DCM";
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
pub struct DlcEvent {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(enumeration = "DlcEventType", tag = "2")]
    pub r#type: i32,
    #[prost(string, tag = "3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub description: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub outcomes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(bool, tag = "7")]
    pub has_triggered: bool,
    #[prost(uint32, tag = "8")]
    pub outcome_index: u32,
    #[prost(message, optional, tag = "9")]
    pub publish_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for DlcEvent {
    const NAME: &'static str = "DLCEvent";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcAttestation {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub event_id: u64,
    #[prost(string, tag = "3")]
    pub outcome: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for DlcAttestation {
    const NAME: &'static str = "DLCAttestation";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DlcOracleStatus {
    OracleStatusPending = 0,
    OracleStatusFailed = 1,
    OracleStatusTimedout = 2,
    OracleStatusEnable = 3,
    OracleStatusDisable = 4,
}
impl DlcOracleStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DlcOracleStatus::OracleStatusPending => "Oracle_Status_Pending",
            DlcOracleStatus::OracleStatusFailed => "Oracle_Status_Failed",
            DlcOracleStatus::OracleStatusTimedout => "Oracle_Status_Timedout",
            DlcOracleStatus::OracleStatusEnable => "Oracle_status_Enable",
            DlcOracleStatus::OracleStatusDisable => "Oracle_status_Disable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Oracle_Status_Pending" => Some(Self::OracleStatusPending),
            "Oracle_Status_Failed" => Some(Self::OracleStatusFailed),
            "Oracle_Status_Timedout" => Some(Self::OracleStatusTimedout),
            "Oracle_status_Enable" => Some(Self::OracleStatusEnable),
            "Oracle_status_Disable" => Some(Self::OracleStatusDisable),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DcmStatus {
    Pending = 0,
    Failed = 1,
    Timedout = 2,
    Enable = 3,
    Disable = 4,
}
impl DcmStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DcmStatus::Pending => "DCM_Status_Pending",
            DcmStatus::Failed => "DCM_Status_Failed",
            DcmStatus::Timedout => "DCM_Status_Timedout",
            DcmStatus::Enable => "DCM_status_Enable",
            DcmStatus::Disable => "DCM_status_Disable",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DCM_Status_Pending" => Some(Self::Pending),
            "DCM_Status_Failed" => Some(Self::Failed),
            "DCM_Status_Timedout" => Some(Self::Timedout),
            "DCM_status_Enable" => Some(Self::Enable),
            "DCM_status_Disable" => Some(Self::Disable),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DlcEventType {
    Unspecified = 0,
    Price = 1,
    Date = 2,
    Lending = 3,
}
impl DlcEventType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DlcEventType::Unspecified => "UNSPECIFIED",
            DlcEventType::Price => "PRICE",
            DlcEventType::Date => "DATE",
            DlcEventType::Lending => "LENDING",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "UNSPECIFIED" => Some(Self::Unspecified),
            "PRICE" => Some(Self::Price),
            "DATE" => Some(Self::Date),
            "LENDING" => Some(Self::Lending),
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
    pub price_event_nonce_queue_size: u32,
    #[prost(message, repeated, tag = "2")]
    pub price_intervals: ::prost::alloc::vec::Vec<PriceInterval>,
    #[prost(uint32, tag = "3")]
    pub date_event_nonce_queue_size: u32,
    #[prost(message, optional, tag = "4")]
    pub date_interval: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    #[prost(uint32, tag = "5")]
    pub lending_event_nonce_queue_size: u32,
    #[prost(message, optional, tag = "6")]
    pub dkg_timeout_period: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
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
    pub events: ::prost::alloc::vec::Vec<DlcEvent>,
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
pub struct QueryAttestationRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryAttestationRequest {
    const NAME: &'static str = "QueryAttestationRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationResponse {
    #[prost(message, optional, tag = "1")]
    pub attestation: ::core::option::Option<DlcAttestation>,
}
impl ::prost::Name for QueryAttestationResponse {
    const NAME: &'static str = "QueryAttestationResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationByEventRequest {
    #[prost(uint64, tag = "1")]
    pub event_id: u64,
}
impl ::prost::Name for QueryAttestationByEventRequest {
    const NAME: &'static str = "QueryAttestationByEventRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationByEventResponse {
    #[prost(message, optional, tag = "1")]
    pub attestation: ::core::option::Option<DlcAttestation>,
}
impl ::prost::Name for QueryAttestationByEventResponse {
    const NAME: &'static str = "QueryAttestationByEventResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryAttestationsRequest {
    const NAME: &'static str = "QueryAttestationsRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAttestationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub attestations: ::prost::alloc::vec::Vec<DlcAttestation>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryAttestationsResponse {
    const NAME: &'static str = "QueryAttestationsResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDcMsRequest {
    #[prost(enumeration = "DcmStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryDcMsRequest {
    const NAME: &'static str = "QueryDCMsRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDcMsResponse {
    #[prost(message, repeated, tag = "1")]
    pub dcms: ::prost::alloc::vec::Vec<Dcm>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryDcMsResponse {
    const NAME: &'static str = "QueryDCMsResponse";
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
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
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
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
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
/// counts should use the same order as recommended oracles in Params
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
pub struct QueryNonceRequest {
    #[prost(uint64, tag = "1")]
    pub oracle_id: u64,
    #[prost(uint64, tag = "2")]
    pub index: u64,
}
impl ::prost::Name for QueryNonceRequest {
    const NAME: &'static str = "QueryNonceRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNonceResponse {
    #[prost(message, optional, tag = "1")]
    pub nonce: ::core::option::Option<DlcNonce>,
}
impl ::prost::Name for QueryNonceResponse {
    const NAME: &'static str = "QueryNonceResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryNoncesRequest {
    #[prost(uint64, tag = "1")]
    pub oracle_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
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
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryNoncesResponse {
    const NAME: &'static str = "QueryNoncesResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryEventRequest is request type for the Query/Event RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEventRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryEventRequest {
    const NAME: &'static str = "QueryEventRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryEventResponse is response type for the Query/Event RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEventResponse {
    #[prost(message, optional, tag = "1")]
    pub event: ::core::option::Option<DlcEvent>,
}
impl ::prost::Name for QueryEventResponse {
    const NAME: &'static str = "QueryEventResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryEventsRequest is request type for the Query/Events RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEventsRequest {
    #[prost(bool, tag = "1")]
    pub triggered: bool,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryEventsRequest {
    const NAME: &'static str = "QueryEventsRequest";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
/// QueryEventsResponse is response type for the Query/Events RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEventsResponse {
    #[prost(message, repeated, tag = "1")]
    pub events: ::prost::alloc::vec::Vec<DlcEvent>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryEventsResponse {
    const NAME: &'static str = "QueryEventsResponse";
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDcmPubKey {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub dcm_id: u64,
    #[prost(string, tag = "4")]
    pub dcm_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitDcmPubKey {
    const NAME: &'static str = "MsgSubmitDCMPubKey";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDcmPubKeyResponse {}
impl ::prost::Name for MsgSubmitDcmPubKeyResponse {
    const NAME: &'static str = "MsgSubmitDCMPubKeyResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitOraclePubKey {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub oracle_id: u64,
    #[prost(string, tag = "4")]
    pub oracle_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitOraclePubKey {
    const NAME: &'static str = "MsgSubmitOraclePubKey";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitOraclePubKeyResponse {}
impl ::prost::Name for MsgSubmitOraclePubKeyResponse {
    const NAME: &'static str = "MsgSubmitOraclePubKeyResponse";
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
    #[prost(enumeration = "DlcEventType", tag = "2")]
    pub event_type: i32,
    #[prost(string, tag = "3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub oracle_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
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
pub struct MsgSubmitAttestation {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub event_id: u64,
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
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateOracle {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "3")]
    pub threshold: u32,
}
impl ::prost::Name for MsgCreateOracle {
    const NAME: &'static str = "MsgCreateOracle";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateOracleResponse {}
impl ::prost::Name for MsgCreateOracleResponse {
    const NAME: &'static str = "MsgCreateOracleResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDcm {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(uint32, tag = "3")]
    pub threshold: u32,
}
impl ::prost::Name for MsgCreateDcm {
    const NAME: &'static str = "MsgCreateDCM";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreateDcmResponse {}
impl ::prost::Name for MsgCreateDcmResponse {
    const NAME: &'static str = "MsgCreateDCMResponse";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
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
    /// params defines the x/dlc parameters to be updated.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.dlc";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.dlc.{}", Self::NAME)
    }
}
include!("side.dlc.serde.rs");
include!("side.dlc.tonic.rs");
// @@protoc_insertion_point(module)
