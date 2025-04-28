// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Liquidation {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub debtor: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub dcm: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub collateral_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub actual_collateral_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub debt_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "8")]
    pub liquidated_price: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub liquidated_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(message, optional, tag = "10")]
    pub liquidated_collateral_amount:
        ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "11")]
    pub liquidated_debt_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "12")]
    pub liquidation_bonus_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "13")]
    pub protocol_liquidation_fee: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "14")]
    pub unliquidated_collateral_amount:
        ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "15")]
    pub liquidation_cet: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub settlement_tx: ::prost::alloc::string::String,
    #[prost(string, tag = "17")]
    pub settlement_tx_id: ::prost::alloc::string::String,
    #[prost(enumeration = "LiquidationStatus", tag = "18")]
    pub status: i32,
}
impl ::prost::Name for Liquidation {
    const NAME: &'static str = "Liquidation";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidationRecord {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub liquidation_id: u64,
    #[prost(string, tag = "3")]
    pub liquidator: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub debt_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub collateral_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub bonus_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for LiquidationRecord {
    const NAME: &'static str = "LiquidationRecord";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
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
pub enum LiquidationStatus {
    Unspecified = 0,
    Liquidating = 1,
    Liquidated = 2,
    Settling = 3,
    Settled = 4,
}
impl LiquidationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LiquidationStatus::Unspecified => "LIQUIDATION_STATUS_UNSPECIFIED",
            LiquidationStatus::Liquidating => "LIQUIDATION_STATUS_LIQUIDATING",
            LiquidationStatus::Liquidated => "LIQUIDATION_STATUS_LIQUIDATED",
            LiquidationStatus::Settling => "LIQUIDATION_STATUS_SETTLING",
            LiquidationStatus::Settled => "LIQUIDATION_STATUS_SETTLED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIQUIDATION_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "LIQUIDATION_STATUS_LIQUIDATING" => Some(Self::Liquidating),
            "LIQUIDATION_STATUS_LIQUIDATED" => Some(Self::Liquidated),
            "LIQUIDATION_STATUS_SETTLING" => Some(Self::Settling),
            "LIQUIDATION_STATUS_SETTLED" => Some(Self::Settled),
            _ => None,
        }
    }
}
/// Signing intent
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SigningIntent {
    Default = 0,
}
impl SigningIntent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SigningIntent::Default => "SIGNING_INTENT_DEFAULT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNING_INTENT_DEFAULT" => Some(Self::Default),
            _ => None,
        }
    }
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// minimum liquidation factor permille
    #[prost(uint32, tag = "1")]
    pub min_liquidation_factor: u32,
    /// liquidation bonus factor permille
    #[prost(uint32, tag = "2")]
    pub liquidation_bonus_factor: u32,
    /// protocol liquidation fee factor permille
    #[prost(uint32, tag = "3")]
    pub protocol_liquidation_fee_factor: u32,
    /// protocol liquidation fee collector
    #[prost(string, tag = "4")]
    pub protocol_liquidation_fee_collector: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// GenesisState defines the liquidation module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub liquidations: ::prost::alloc::vec::Vec<Liquidation>,
    #[prost(message, repeated, tag = "3")]
    pub liquidation_records: ::prost::alloc::vec::Vec<LiquidationRecord>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationRequest is request type for the Query/Liquidation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryLiquidationRequest {
    const NAME: &'static str = "QueryLiquidationRequest";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationResponse is response type for the Query/Liquidation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationResponse {
    #[prost(message, optional, tag = "1")]
    pub liquidation: ::core::option::Option<Liquidation>,
}
impl ::prost::Name for QueryLiquidationResponse {
    const NAME: &'static str = "QueryLiquidationResponse";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationsRequest is request type for the Query/Liquidations RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationsRequest {
    #[prost(enumeration = "LiquidationStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryLiquidationsRequest {
    const NAME: &'static str = "QueryLiquidationsRequest";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationsResponse is response type for the Query/Liquidations RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationsResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidations: ::prost::alloc::vec::Vec<Liquidation>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryLiquidationsResponse {
    const NAME: &'static str = "QueryLiquidationsResponse";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationRecordRequest is request type for the Query/LiquidationRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationRecordRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryLiquidationRecordRequest {
    const NAME: &'static str = "QueryLiquidationRecordRequest";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationRecordResponse is response type for the Query/LiquidationRecord RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationRecordResponse {
    #[prost(message, optional, tag = "1")]
    pub liquidation_record: ::core::option::Option<LiquidationRecord>,
}
impl ::prost::Name for QueryLiquidationRecordResponse {
    const NAME: &'static str = "QueryLiquidationRecordResponse";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationRecordsRequest is request type for the Query/LiquidationRecords RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationRecordsRequest {
    #[prost(uint64, tag = "1")]
    pub liquidation_id: u64,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryLiquidationRecordsRequest {
    const NAME: &'static str = "QueryLiquidationRecordsRequest";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// QueryLiquidationRecordsResponse is response type for the Query/LiquidationRecords RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationRecordsResponse {
    #[prost(message, repeated, tag = "1")]
    pub liquidation_records: ::prost::alloc::vec::Vec<LiquidationRecord>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryLiquidationRecordsResponse {
    const NAME: &'static str = "QueryLiquidationRecordsResponse";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// MsgLiquidate defines the Msg/Liquidate request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidate {
    #[prost(string, tag = "1")]
    pub liquidator: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub liquidation_id: u64,
    #[prost(message, optional, tag = "3")]
    pub debt_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgLiquidate {
    const NAME: &'static str = "MsgLiquidate";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
/// MsgLiquidateResponse defines the Msg/Liquidate response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgLiquidateResponse {}
impl ::prost::Name for MsgLiquidateResponse {
    const NAME: &'static str = "MsgLiquidateResponse";
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.liquidation";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.liquidation.{}", Self::NAME)
    }
}
include!("side.liquidation.serde.rs");
include!("side.liquidation.tonic.rs");
// @@protoc_insertion_point(module)
