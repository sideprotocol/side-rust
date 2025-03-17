// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// origination fee collector address
    #[prost(string, tag = "1")]
    pub origination_fee_collector: ::prost::alloc::string::String,
    /// protocol fee collector address
    #[prost(string, tag = "2")]
    pub protocol_fee_collector: ::prost::alloc::string::String,
    /// final timeout duration for each loan
    #[prost(message, optional, tag = "3")]
    pub final_timeout_duration:
        ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Lending pool config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolConfig {
    /// supply rate permille
    #[prost(uint32, tag = "1")]
    pub supply_rate: u32,
    /// borrow rate permille
    #[prost(uint32, tag = "2")]
    pub borrow_rate: u32,
    /// reserve factor permille
    #[prost(uint32, tag = "3")]
    pub reserve_factor: u32,
    /// supply cap
    #[prost(string, tag = "4")]
    pub supply_cap: ::prost::alloc::string::String,
    /// borrow cap
    #[prost(string, tag = "5")]
    pub borrow_cap: ::prost::alloc::string::String,
    /// debt ceiling
    #[prost(string, tag = "6")]
    pub debt_ceiling: ::prost::alloc::string::String,
    /// origination fee
    #[prost(string, tag = "7")]
    pub origination_fee: ::prost::alloc::string::String,
    /// maximum loan to value permille
    #[prost(uint32, tag = "8")]
    pub ltv: u32,
    /// liquidation ltv permille
    #[prost(uint32, tag = "9")]
    pub liquidation_threshold: u32,
    /// liquidation penalty permille
    #[prost(uint32, tag = "10")]
    pub liquidation_penalty: u32,
}
impl ::prost::Name for PoolConfig {
    const NAME: &'static str = "PoolConfig";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LendingPool {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub supply: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "3")]
    pub available_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub borrowed_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub total_shares: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub config: ::core::option::Option<PoolConfig>,
    #[prost(enumeration = "PoolStatus", tag = "7")]
    pub status: i32,
}
impl ::prost::Name for LendingPool {
    const NAME: &'static str = "LendingPool";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Loan {
    /// id
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub borrower_pub_key: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub agency: ::prost::alloc::string::String,
    #[prost(int64, tag = "5")]
    pub maturity_time: i64,
    #[prost(int64, tag = "6")]
    pub final_timeout: i64,
    #[prost(message, optional, tag = "7")]
    pub borrow_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "8")]
    pub origination_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub interest: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub protocol_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub term: ::prost::alloc::string::String,
    #[prost(uint64, tag = "12")]
    pub event_id: u64,
    #[prost(uint64, tag = "13")]
    pub attestation_id: u64,
    #[prost(string, repeated, tag = "14")]
    pub deposit_txs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "15")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "16")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "17")]
    pub auction_id: u64,
    #[prost(message, optional, tag = "18")]
    pub create_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "LoanStatus", tag = "19")]
    pub status: i32,
}
impl ::prost::Name for Loan {
    const NAME: &'static str = "Loan";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcMeta {
    #[prost(string, tag = "1")]
    pub liquidation_cet: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub signed_liquidation_cet_hex: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub liquidation_adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub liquidation_adapted_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub liquidation_agency_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub vault_utxo: ::core::option::Option<super::btcbridge::Utxo>,
    #[prost(string, tag = "7")]
    pub internal_key: ::prost::alloc::string::String,
    #[prost(string, tag = "8")]
    pub liquidation_cet_script: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub repayment_script: ::prost::alloc::string::String,
    #[prost(string, tag = "10")]
    pub forced_repayment_script: ::prost::alloc::string::String,
    #[prost(string, tag = "11")]
    pub timeout_refund_script: ::prost::alloc::string::String,
}
impl ::prost::Name for DlcMeta {
    const NAME: &'static str = "DLCMeta";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DepositLog {
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "1")]
    pub vault_address: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub deposit_tx: ::prost::alloc::string::String,
}
impl ::prost::Name for DepositLog {
    const NAME: &'static str = "DepositLog";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Repayment {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub adaptor_point: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub dca_adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "6")]
    pub dca_adapted_signature: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub create_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for Repayment {
    const NAME: &'static str = "Repayment";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Cancellation {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "5")]
    pub dca_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "6")]
    pub create_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for Cancellation {
    const NAME: &'static str = "Cancellation";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Status options for a lending pool
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum PoolStatus {
    Inactive = 0,
    Active = 1,
}
impl PoolStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            PoolStatus::Inactive => "INACTIVE",
            PoolStatus::Active => "ACTIVE",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INACTIVE" => Some(Self::Inactive),
            "ACTIVE" => Some(Self::Active),
            _ => None,
        }
    }
}
/// Loan Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum LoanStatus {
    /// Unspecified
    Unspecified = 0,
    /// Loan Requested
    Requested = 1,
    /// Loan Open
    Open = 2,
    /// Loan Rejected
    Rejected = 3,
    /// Loan Cancelled
    Cancelled = 4,
    /// Loan Repaid
    Repaid = 5,
    /// Loan Defaulted
    Defaulted = 6,
    /// Loan Liquidated
    Liquidated = 7,
    /// Loan Closed
    Closed = 8,
}
impl LoanStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            LoanStatus::Unspecified => "Unspecified",
            LoanStatus::Requested => "Requested",
            LoanStatus::Open => "Open",
            LoanStatus::Rejected => "Rejected",
            LoanStatus::Cancelled => "Cancelled",
            LoanStatus::Repaid => "Repaid",
            LoanStatus::Defaulted => "Defaulted",
            LoanStatus::Liquidated => "Liquidated",
            LoanStatus::Closed => "Closed",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "Unspecified" => Some(Self::Unspecified),
            "Requested" => Some(Self::Requested),
            "Open" => Some(Self::Open),
            "Rejected" => Some(Self::Rejected),
            "Cancelled" => Some(Self::Cancelled),
            "Repaid" => Some(Self::Repaid),
            "Defaulted" => Some(Self::Defaulted),
            "Liquidated" => Some(Self::Liquidated),
            "Closed" => Some(Self::Closed),
            _ => None,
        }
    }
}
/// GenesisState defines the lending module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub pools: ::prost::alloc::vec::Vec<LendingPool>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryPoolRequest is request type for the Query/Pool RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolRequest {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPoolRequest {
    const NAME: &'static str = "QueryPoolRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryPoolResponse is response type for the Query/Pool RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolResponse {
    #[prost(message, optional, tag = "1")]
    pub pool: ::core::option::Option<LendingPool>,
}
impl ::prost::Name for QueryPoolResponse {
    const NAME: &'static str = "QueryPoolResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryPoolsRequest is request type for the Query/Pools RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryPoolsRequest {
    const NAME: &'static str = "QueryPoolsRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryPoolsResponse is response type for the Query/Pools RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolsResponse {
    #[prost(message, repeated, tag = "1")]
    pub pools: ::prost::alloc::vec::Vec<LendingPool>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryPoolsResponse {
    const NAME: &'static str = "QueryPoolsResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationEventRequest {
    #[prost(string, tag = "1")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub borrow_amount: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLiquidationEventRequest {
    const NAME: &'static str = "QueryLiquidationEventRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationEventResponse {
    #[prost(uint64, tag = "1")]
    pub event_id: u64,
    #[prost(string, tag = "2")]
    pub oracle_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub nonce: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub signature_point: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLiquidationEventResponse {
    const NAME: &'static str = "QueryLiquidationEventResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationCetRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub borrower_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub agency_pubkey: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLiquidationCetRequest {
    const NAME: &'static str = "QueryLiquidationCetRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationCetResponse {
    #[prost(string, tag = "1")]
    pub script: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub sig_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for QueryLiquidationCetResponse {
    const NAME: &'static str = "QueryLiquidationCetResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollateralAddressRequest {
    #[prost(string, tag = "1")]
    pub borrower_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub agency_pubkey: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub maturity_time: u64,
}
impl ::prost::Name for QueryCollateralAddressRequest {
    const NAME: &'static str = "QueryCollateralAddressRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCollateralAddressResponse {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryCollateralAddressResponse {
    const NAME: &'static str = "QueryCollateralAddressResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanRequest is request type for the Query/Loan RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLoanRequest {
    const NAME: &'static str = "QueryLoanRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanResponse is response type for the Query/Loan RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanResponse {
    #[prost(message, optional, tag = "1")]
    pub loan: ::core::option::Option<Loan>,
}
impl ::prost::Name for QueryLoanResponse {
    const NAME: &'static str = "QueryLoanResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoansRequest is request type for the Query/Loans RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoansRequest {
    #[prost(enumeration = "LoanStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryLoansRequest {
    const NAME: &'static str = "QueryLoansRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoansResponse is response type for the Query/Loans RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoansResponse {
    #[prost(message, repeated, tag = "1")]
    pub loans: ::prost::alloc::vec::Vec<Loan>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryLoansResponse {
    const NAME: &'static str = "QueryLoansResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoansByAddressRequest is request type for the Query/LoansByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoansByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "LoanStatus", tag = "2")]
    pub status: i32,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryLoansByAddressRequest {
    const NAME: &'static str = "QueryLoansByAddressRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoansByAddressResponse is response type for the Query/LoansByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoansByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub loans: ::prost::alloc::vec::Vec<Loan>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryLoansByAddressResponse {
    const NAME: &'static str = "QueryLoansByAddressResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanDlcMetaRequest is request type for the Query/LoanDlcMeta RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanDlcMetaRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLoanDlcMetaRequest {
    const NAME: &'static str = "QueryLoanDlcMetaRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanDlcMetaResponse is response type for the Query/LoanDlcMeta RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanDlcMetaResponse {
    #[prost(message, optional, tag = "1")]
    pub dlc_meta: ::core::option::Option<DlcMeta>,
}
impl ::prost::Name for QueryLoanDlcMetaResponse {
    const NAME: &'static str = "QueryLoanDlcMetaResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanCancellationRequest is request type for the Query/LoanCancellation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanCancellationRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLoanCancellationRequest {
    const NAME: &'static str = "QueryLoanCancellationRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanCancellationResponse is response type for the Query/LoanCancellation RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanCancellationResponse {
    #[prost(message, optional, tag = "1")]
    pub cancellation: ::core::option::Option<Cancellation>,
}
impl ::prost::Name for QueryLoanCancellationResponse {
    const NAME: &'static str = "QueryLoanCancellationResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRepaymentRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryRepaymentRequest {
    const NAME: &'static str = "QueryRepaymentRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRepaymentResponse {
    #[prost(message, optional, tag = "1")]
    pub repayment: ::core::option::Option<Repayment>,
}
impl ::prost::Name for QueryRepaymentResponse {
    const NAME: &'static str = "QueryRepaymentResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePool {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// pool id
    #[prost(string, tag = "2")]
    pub id: ::prost::alloc::string::String,
    /// lending asset
    #[prost(string, tag = "3")]
    pub lending_asset: ::prost::alloc::string::String,
    /// pool config
    #[prost(message, optional, tag = "4")]
    pub config: ::core::option::Option<PoolConfig>,
}
impl ::prost::Name for MsgCreatePool {
    const NAME: &'static str = "MsgCreatePool";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCreatePoolResponse {}
impl ::prost::Name for MsgCreatePoolResponse {
    const NAME: &'static str = "MsgCreatePoolResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdatePoolConfig {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// pool id
    #[prost(string, tag = "2")]
    pub pool_id: ::prost::alloc::string::String,
    /// pool config
    #[prost(message, optional, tag = "3")]
    pub config: ::core::option::Option<PoolConfig>,
}
impl ::prost::Name for MsgUpdatePoolConfig {
    const NAME: &'static str = "MsgUpdatePoolConfig";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdatePoolConfigResponse {}
impl ::prost::Name for MsgUpdatePoolConfigResponse {
    const NAME: &'static str = "MsgUpdatePoolConfigResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRepay {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub adaptor_point: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgRepay {
    const NAME: &'static str = "MsgRepay";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRepayResponse {}
impl ::prost::Name for MsgRepayResponse {
    const NAME: &'static str = "MsgRepayResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddLiquidity {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgAddLiquidity {
    const NAME: &'static str = "MsgAddLiquidity";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgAddLiquidityResponse {}
impl ::prost::Name for MsgAddLiquidityResponse {
    const NAME: &'static str = "MsgAddLiquidityResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveLiquidity {
    #[prost(string, tag = "1")]
    pub lender: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub shares: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for MsgRemoveLiquidity {
    const NAME: &'static str = "MsgRemoveLiquidity";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRemoveLiquidityResponse {}
impl ::prost::Name for MsgRemoveLiquidityResponse {
    const NAME: &'static str = "MsgRemoveLiquidityResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgApply {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub borrower_pubkey: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub maturity_time: i64,
    #[prost(string, tag = "4")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub borrow_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(uint64, tag = "6")]
    pub agency_id: u64,
}
impl ::prost::Name for MsgApply {
    const NAME: &'static str = "MsgApply";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgApplyResponse {}
impl ::prost::Name for MsgApplyResponse {
    const NAME: &'static str = "MsgApplyResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitLiquidationCet {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub event_id: u64,
    #[prost(string, tag = "4")]
    pub deposit_tx: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub liquidation_cet: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "6")]
    pub liquidation_adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitLiquidationCet {
    const NAME: &'static str = "MsgSubmitLiquidationCet";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitLiquidationCetResponse {}
impl ::prost::Name for MsgSubmitLiquidationCetResponse {
    const NAME: &'static str = "MsgSubmitLiquidationCetResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgApprove {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub deposit_tx_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgApprove {
    const NAME: &'static str = "MsgApprove";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgApproveResponse {}
impl ::prost::Name for MsgApproveResponse {
    const NAME: &'static str = "MsgApproveResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancel {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgCancel {
    const NAME: &'static str = "MsgCancel";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCancelResponse {}
impl ::prost::Name for MsgCancelResponse {
    const NAME: &'static str = "MsgCancelResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitCancellationSignatures {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitCancellationSignatures {
    const NAME: &'static str = "MsgSubmitCancellationSignatures";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitCancellationSignaturesResponse {}
impl ::prost::Name for MsgSubmitCancellationSignaturesResponse {
    const NAME: &'static str = "MsgSubmitCancellationSignaturesResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitRepaymentAdaptorSignatures {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitRepaymentAdaptorSignatures {
    const NAME: &'static str = "MsgSubmitRepaymentAdaptorSignatures";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitRepaymentAdaptorSignaturesResponse {}
impl ::prost::Name for MsgSubmitRepaymentAdaptorSignaturesResponse {
    const NAME: &'static str = "MsgSubmitRepaymentAdaptorSignaturesResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitLiquidationCetSignatures {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitLiquidationCetSignatures {
    const NAME: &'static str = "MsgSubmitLiquidationCetSignatures";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitLiquidationCetSignaturesResponse {}
impl ::prost::Name for MsgSubmitLiquidationCetSignaturesResponse {
    const NAME: &'static str = "MsgSubmitLiquidationCetSignaturesResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClose {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgClose {
    const NAME: &'static str = "MsgClose";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCloseResponse {}
impl ::prost::Name for MsgCloseResponse {
    const NAME: &'static str = "MsgCloseResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitPrice {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub price: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgSubmitPrice {
    const NAME: &'static str = "MsgSubmitPrice";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitPriceResponse {}
impl ::prost::Name for MsgSubmitPriceResponse {
    const NAME: &'static str = "MsgSubmitPriceResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
include!("side.lending.serde.rs");
include!("side.lending.tonic.rs");
// @@protoc_insertion_point(module)
