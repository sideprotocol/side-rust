// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// final timeout duration for loan
    #[prost(message, optional, tag = "1")]
    pub final_timeout_duration:
        ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    /// request fee collector address
    #[prost(string, tag = "2")]
    pub request_fee_collector: ::prost::alloc::string::String,
    /// origination fee collector address
    #[prost(string, tag = "3")]
    pub origination_fee_collector: ::prost::alloc::string::String,
    /// protocol fee collector address
    #[prost(string, tag = "4")]
    pub protocol_fee_collector: ::prost::alloc::string::String,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Asset metadata
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AssetMetadata {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub decimals: i32,
    #[prost(string, tag = "4")]
    pub price_symbol: ::prost::alloc::string::String,
    #[prost(bool, tag = "5")]
    pub is_base_price_asset: bool,
}
impl ::prost::Name for AssetMetadata {
    const NAME: &'static str = "AssetMetadata";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Pool tranche config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolTrancheConfig {
    /// maturity duration in seconds
    #[prost(int64, tag = "1")]
    pub maturity: i64,
    /// borrow apr permille
    #[prost(uint32, tag = "2")]
    pub borrow_apr: u32,
    /// minimum maturity factor permille
    #[prost(uint32, tag = "3")]
    pub min_maturity_factor: u32,
}
impl ::prost::Name for PoolTrancheConfig {
    const NAME: &'static str = "PoolTrancheConfig";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Pool config
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolConfig {
    /// collateral asset metadata
    #[prost(message, optional, tag = "1")]
    pub collateral_asset: ::core::option::Option<AssetMetadata>,
    /// lending asset metadata
    #[prost(message, optional, tag = "2")]
    pub lending_asset: ::core::option::Option<AssetMetadata>,
    /// supply cap
    #[prost(string, tag = "3")]
    pub supply_cap: ::prost::alloc::string::String,
    /// borrow cap
    #[prost(string, tag = "4")]
    pub borrow_cap: ::prost::alloc::string::String,
    /// minimum amount to be borrowed
    #[prost(string, tag = "5")]
    pub min_borrow_amount: ::prost::alloc::string::String,
    /// maximum amount to be borrowed
    #[prost(string, tag = "6")]
    pub max_borrow_amount: ::prost::alloc::string::String,
    /// tranches
    #[prost(message, repeated, tag = "7")]
    pub tranches: ::prost::alloc::vec::Vec<PoolTrancheConfig>,
    /// request fee
    #[prost(message, optional, tag = "8")]
    pub request_fee: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// origination fee
    #[prost(string, tag = "9")]
    pub origination_fee: ::prost::alloc::string::String,
    /// reserve factor permille
    #[prost(uint32, tag = "10")]
    pub reserve_factor: u32,
    /// referral fee factor permille
    #[prost(uint32, tag = "11")]
    pub referral_fee_factor: u32,
    /// maximum ltv percent
    #[prost(uint32, tag = "12")]
    pub max_ltv: u32,
    /// liquidation ltv percent
    #[prost(uint32, tag = "13")]
    pub liquidation_threshold: u32,
    /// indicates if the pool is paused
    #[prost(bool, tag = "14")]
    pub paused: bool,
}
impl ::prost::Name for PoolConfig {
    const NAME: &'static str = "PoolConfig";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Pool tranche
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PoolTranche {
    /// maturity duration
    #[prost(int64, tag = "1")]
    pub maturity: i64,
    /// borrow index
    #[prost(string, tag = "2")]
    pub borrow_index: ::prost::alloc::string::String,
    /// total borrowed
    #[prost(string, tag = "3")]
    pub total_borrowed: ::prost::alloc::string::String,
    /// total reserve
    #[prost(string, tag = "4")]
    pub total_reserve: ::prost::alloc::string::String,
}
impl ::prost::Name for PoolTranche {
    const NAME: &'static str = "PoolTranche";
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
    pub total_borrowed: ::prost::alloc::string::String,
    #[prost(string, tag = "6")]
    pub reserve_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "7")]
    pub total_reserve: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "8")]
    pub total_stokens: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "9")]
    pub tranches: ::prost::alloc::vec::Vec<PoolTranche>,
    #[prost(message, optional, tag = "10")]
    pub config: ::core::option::Option<PoolConfig>,
    #[prost(enumeration = "PoolStatus", tag = "11")]
    pub status: i32,
}
impl ::prost::Name for LendingPool {
    const NAME: &'static str = "LendingPool";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// Authorization with deposit txs for cets
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Authorization {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, repeated, tag = "2")]
    pub deposit_txs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(enumeration = "AuthorizationStatus", tag = "3")]
    pub status: i32,
}
impl ::prost::Name for Authorization {
    const NAME: &'static str = "Authorization";
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
    pub borrower_auth_pub_key: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub dcm: ::prost::alloc::string::String,
    #[prost(int64, tag = "6")]
    pub maturity_time: i64,
    #[prost(int64, tag = "7")]
    pub final_timeout: i64,
    #[prost(string, tag = "8")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "9")]
    pub borrow_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "10")]
    pub request_fee: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(string, tag = "11")]
    pub origination_fee: ::prost::alloc::string::String,
    #[prost(string, tag = "12")]
    pub interest: ::prost::alloc::string::String,
    #[prost(string, tag = "13")]
    pub protocol_fee: ::prost::alloc::string::String,
    #[prost(int64, tag = "14")]
    pub maturity: i64,
    #[prost(uint32, tag = "15")]
    pub borrow_apr: u32,
    #[prost(int64, tag = "16")]
    pub min_maturity: i64,
    #[prost(string, tag = "17")]
    pub start_borrow_index: ::prost::alloc::string::String,
    #[prost(string, tag = "18")]
    pub liquidation_price: ::prost::alloc::string::String,
    #[prost(uint64, tag = "19")]
    pub dlc_event_id: u64,
    #[prost(message, repeated, tag = "20")]
    pub authorizations: ::prost::alloc::vec::Vec<Authorization>,
    #[prost(string, tag = "21")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "22")]
    pub liquidation_id: u64,
    #[prost(string, tag = "23")]
    pub referrer: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "24")]
    pub create_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(message, optional, tag = "25")]
    pub disburse_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "LoanStatus", tag = "26")]
    pub status: i32,
}
impl ::prost::Name for Loan {
    const NAME: &'static str = "Loan";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// LeafScript defines the tap leaf script
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LeafScript {
    #[prost(string, tag = "1")]
    pub script: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub control_block: ::prost::alloc::string::String,
}
impl ::prost::Name for LeafScript {
    const NAME: &'static str = "LeafScript";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CetInfo {
    #[prost(uint64, tag = "1")]
    pub event_id: u64,
    #[prost(uint32, tag = "2")]
    pub outcome_index: u32,
    #[prost(string, tag = "3")]
    pub signature_point: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "4")]
    pub script: ::core::option::Option<LeafScript>,
}
impl ::prost::Name for CetInfo {
    const NAME: &'static str = "CetInfo";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct LiquidationCet {
    #[prost(string, tag = "1")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub borrower_adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub borrower_adapted_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub dcm_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub signed_tx_hex: ::prost::alloc::string::String,
}
impl ::prost::Name for LiquidationCet {
    const NAME: &'static str = "LiquidationCet";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RepaymentCet {
    #[prost(string, tag = "1")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub dcm_adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "3")]
    pub dcm_adapted_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "4")]
    pub borrower_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "5")]
    pub signed_tx_hex: ::prost::alloc::string::String,
}
impl ::prost::Name for RepaymentCet {
    const NAME: &'static str = "RepaymentCet";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DlcMeta {
    #[prost(message, optional, tag = "1")]
    pub liquidation_cet: ::core::option::Option<LiquidationCet>,
    #[prost(message, optional, tag = "2")]
    pub default_liquidation_cet: ::core::option::Option<LiquidationCet>,
    #[prost(message, optional, tag = "3")]
    pub repayment_cet: ::core::option::Option<RepaymentCet>,
    #[prost(string, tag = "4")]
    pub timeout_refund_tx: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "5")]
    pub vault_utxos: ::prost::alloc::vec::Vec<super::btcbridge::Utxo>,
    #[prost(string, tag = "6")]
    pub internal_key: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "7")]
    pub liquidation_script: ::core::option::Option<LeafScript>,
    #[prost(message, optional, tag = "8")]
    pub repayment_script: ::core::option::Option<LeafScript>,
    #[prost(message, optional, tag = "9")]
    pub timeout_refund_script: ::core::option::Option<LeafScript>,
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
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vault_address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub authorization_id: u64,
    #[prost(string, tag = "4")]
    pub deposit_tx: ::prost::alloc::string::String,
    #[prost(enumeration = "DepositStatus", tag = "5")]
    pub status: i32,
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
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
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
pub struct Redemption {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub dcm_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub create_at: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
}
impl ::prost::Name for Redemption {
    const NAME: &'static str = "Redemption";
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
    Paused = 2,
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
            PoolStatus::Paused => "PAUSED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INACTIVE" => Some(Self::Inactive),
            "ACTIVE" => Some(Self::Active),
            "PAUSED" => Some(Self::Paused),
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
    /// Loan Authorized
    Authorized = 2,
    /// Loan Rejected
    Rejected = 3,
    /// Loan Open
    Open = 4,
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
            LoanStatus::Authorized => "Authorized",
            LoanStatus::Rejected => "Rejected",
            LoanStatus::Open => "Open",
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
            "Authorized" => Some(Self::Authorized),
            "Rejected" => Some(Self::Rejected),
            "Open" => Some(Self::Open),
            "Repaid" => Some(Self::Repaid),
            "Defaulted" => Some(Self::Defaulted),
            "Liquidated" => Some(Self::Liquidated),
            "Closed" => Some(Self::Closed),
            _ => None,
        }
    }
}
/// Authorization Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AuthorizationStatus {
    /// Pending
    Pending = 0,
    /// Authorized
    Authorized = 1,
    /// Rejected
    Rejected = 2,
}
impl AuthorizationStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AuthorizationStatus::Pending => "AUTHORIZATION_STATUS_PENDING",
            AuthorizationStatus::Authorized => "AUTHORIZATION_STATUS_AUTHORIZED",
            AuthorizationStatus::Rejected => "AUTHORIZATION_STATUS_REJECTED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "AUTHORIZATION_STATUS_PENDING" => Some(Self::Pending),
            "AUTHORIZATION_STATUS_AUTHORIZED" => Some(Self::Authorized),
            "AUTHORIZATION_STATUS_REJECTED" => Some(Self::Rejected),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum CetType {
    Liquidation = 0,
    DefaultLiquidation = 1,
    Repayment = 2,
}
impl CetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            CetType::Liquidation => "LIQUIDATION",
            CetType::DefaultLiquidation => "DEFAULT_LIQUIDATION",
            CetType::Repayment => "REPAYMENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "LIQUIDATION" => Some(Self::Liquidation),
            "DEFAULT_LIQUIDATION" => Some(Self::DefaultLiquidation),
            "REPAYMENT" => Some(Self::Repayment),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DepositStatus {
    Pending = 0,
    Verified = 1,
    Redeeming = 2,
    Redeemed = 3,
}
impl DepositStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DepositStatus::Pending => "DEPOSIT_STATUS_PENDING",
            DepositStatus::Verified => "DEPOSIT_STATUS_VERIFIED",
            DepositStatus::Redeeming => "DEPOSIT_STATUS_REDEEMING",
            DepositStatus::Redeemed => "DEPOSIT_STATUS_REDEEMED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DEPOSIT_STATUS_PENDING" => Some(Self::Pending),
            "DEPOSIT_STATUS_VERIFIED" => Some(Self::Verified),
            "DEPOSIT_STATUS_REDEEMING" => Some(Self::Redeeming),
            "DEPOSIT_STATUS_REDEEMED" => Some(Self::Redeemed),
            _ => None,
        }
    }
}
/// Signing intent
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SigningIntent {
    Repayment = 0,
    Liquidation = 1,
    DefaultLiquidation = 2,
    Redemption = 3,
}
impl SigningIntent {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SigningIntent::Repayment => "SIGNING_INTENT_REPAYMENT",
            SigningIntent::Liquidation => "SIGNING_INTENT_LIQUIDATION",
            SigningIntent::DefaultLiquidation => "SIGNING_INTENT_DEFAULT_LIQUIDATION",
            SigningIntent::Redemption => "SIGNING_INTENT_REDEMPTION",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNING_INTENT_REPAYMENT" => Some(Self::Repayment),
            "SIGNING_INTENT_LIQUIDATION" => Some(Self::Liquidation),
            "SIGNING_INTENT_DEFAULT_LIQUIDATION" => Some(Self::DefaultLiquidation),
            "SIGNING_INTENT_REDEMPTION" => Some(Self::Redemption),
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
/// QueryPoolExchangeRateRequest is request type for the Query/PoolExchangeRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolExchangeRateRequest {
    #[prost(string, tag = "1")]
    pub pool_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPoolExchangeRateRequest {
    const NAME: &'static str = "QueryPoolExchangeRateRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryPoolExchangeRateResponse is response type for the Query/PoolExchangeRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPoolExchangeRateResponse {
    #[prost(string, tag = "1")]
    pub exchange_rate: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPoolExchangeRateResponse {
    const NAME: &'static str = "QueryPoolExchangeRateResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationPriceRequest {
    #[prost(string, tag = "1")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub collateral_amount: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub borrow_amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "4")]
    pub maturity: i64,
}
impl ::prost::Name for QueryLiquidationPriceRequest {
    const NAME: &'static str = "QueryLiquidationPriceRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLiquidationPriceResponse {
    #[prost(string, tag = "1")]
    pub price: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub pair: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLiquidationPriceResponse {
    const NAME: &'static str = "QueryLiquidationPriceResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDlcEventCountRequest {}
impl ::prost::Name for QueryDlcEventCountRequest {
    const NAME: &'static str = "QueryDlcEventCountRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDlcEventCountResponse {
    #[prost(uint64, tag = "1")]
    pub count: u64,
}
impl ::prost::Name for QueryDlcEventCountResponse {
    const NAME: &'static str = "QueryDlcEventCountResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanCetInfosRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLoanCetInfosRequest {
    const NAME: &'static str = "QueryLoanCetInfosRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanCetInfosResponse {
    #[prost(message, optional, tag = "1")]
    pub liquidation_cet_info: ::core::option::Option<CetInfo>,
    #[prost(message, optional, tag = "2")]
    pub default_liquidation_cet_info: ::core::option::Option<CetInfo>,
    #[prost(message, optional, tag = "3")]
    pub repayment_cet_info: ::core::option::Option<CetInfo>,
}
impl ::prost::Name for QueryLoanCetInfosResponse {
    const NAME: &'static str = "QueryLoanCetInfosResponse";
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
    pub borrower_auth_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub dcm_pubkey: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
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
    pub id: ::prost::alloc::string::String,
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
/// QueryLoanAuthorizationRequest is request type for the Query/LoanAuthorization RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanAuthorizationRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
impl ::prost::Name for QueryLoanAuthorizationRequest {
    const NAME: &'static str = "QueryLoanAuthorizationRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryLoanAuthorizationResponse is response type for the Query/LoanAuthorization RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanAuthorizationResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposits: ::prost::alloc::vec::Vec<DepositLog>,
    #[prost(enumeration = "AuthorizationStatus", tag = "2")]
    pub status: i32,
}
impl ::prost::Name for QueryLoanAuthorizationResponse {
    const NAME: &'static str = "QueryLoanAuthorizationResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanDepositsRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryLoanDepositsRequest {
    const NAME: &'static str = "QueryLoanDepositsRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryLoanDepositsResponse {
    #[prost(message, repeated, tag = "1")]
    pub deposits: ::prost::alloc::vec::Vec<DepositLog>,
}
impl ::prost::Name for QueryLoanDepositsResponse {
    const NAME: &'static str = "QueryLoanDepositsResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryRedemptionRequest is request type for the Query/Redemption RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedemptionRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryRedemptionRequest {
    const NAME: &'static str = "QueryRedemptionRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
/// QueryRedemptionResponse is response type for the Query/Redemption RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRedemptionResponse {
    #[prost(message, optional, tag = "1")]
    pub redemption: ::core::option::Option<Redemption>,
}
impl ::prost::Name for QueryRedemptionResponse {
    const NAME: &'static str = "QueryRedemptionResponse";
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
pub struct QueryCurrentInterestRequest {
    #[prost(string, tag = "1")]
    pub loan_id: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryCurrentInterestRequest {
    const NAME: &'static str = "QueryCurrentInterestRequest";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentInterestResponse {
    #[prost(message, optional, tag = "1")]
    pub interest: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for QueryCurrentInterestResponse {
    const NAME: &'static str = "QueryCurrentInterestResponse";
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
    /// pool config
    #[prost(message, optional, tag = "3")]
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
    pub stokens: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
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
    #[prost(string, tag = "3")]
    pub borrower_auth_pubkey: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub pool_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub borrow_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(int64, tag = "6")]
    pub maturity: i64,
    #[prost(uint64, tag = "7")]
    pub dcm_id: u64,
    #[prost(string, tag = "8")]
    pub referrer: ::prost::alloc::string::String,
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
pub struct MsgSubmitCets {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub deposit_txs: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub liquidation_cet: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub liquidation_adaptor_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub default_liquidation_adaptor_signatures:
        ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "7")]
    pub repayment_cet: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "8")]
    pub repayment_signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitCets {
    const NAME: &'static str = "MsgSubmitCets";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitCetsResponse {}
impl ::prost::Name for MsgSubmitCetsResponse {
    const NAME: &'static str = "MsgSubmitCetsResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDepositTransaction {
    #[prost(string, tag = "1")]
    pub relayer: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub vault: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub deposit_tx: ::prost::alloc::string::String,
    #[prost(string, tag = "4")]
    pub block_hash: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitDepositTransaction {
    const NAME: &'static str = "MsgSubmitDepositTransaction";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDepositTransactionResponse {}
impl ::prost::Name for MsgSubmitDepositTransactionResponse {
    const NAME: &'static str = "MsgSubmitDepositTransactionResponse";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRedeem {
    #[prost(string, tag = "1")]
    pub borrower: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub loan_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub tx: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgRedeem {
    const NAME: &'static str = "MsgRedeem";
    const PACKAGE: &'static str = "side.lending";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.lending.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgRedeemResponse {}
impl ::prost::Name for MsgRedeemResponse {
    const NAME: &'static str = "MsgRedeemResponse";
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
