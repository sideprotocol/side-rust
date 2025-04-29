// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// The minimum number of confirmations required for the deposit transactions
    #[prost(int32, tag = "1")]
    pub deposit_confirmation_depth: i32,
    /// The minimum number of confirmations required for the withdrawal transactions
    #[prost(int32, tag = "2")]
    pub withdraw_confirmation_depth: i32,
    /// The allowed maximum depth for bitcoin block reorganization
    #[prost(int32, tag = "3")]
    pub max_reorg_depth: i32,
    /// Indicates the maximum depth or distance from the latest block up to which transactions are considered for acceptance.
    #[prost(uint64, tag = "4")]
    pub max_acceptable_block_depth: u64,
    /// The denomination of the voucher
    #[prost(string, tag = "5")]
    pub btc_voucher_denom: ::prost::alloc::string::String,
    /// Indicates if deposit is enabled
    #[prost(bool, tag = "6")]
    pub deposit_enabled: bool,
    /// Indicates if withdrawal is enabled
    #[prost(bool, tag = "7")]
    pub withdraw_enabled: bool,
    /// Trusted relayers for non-btc asset deposit
    #[prost(string, repeated, tag = "8")]
    pub trusted_non_btc_relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Trusted fee providers to submit bitcoin fee rate
    #[prost(string, repeated, tag = "9")]
    pub trusted_fee_providers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// Period of validity for the fee rate
    #[prost(int64, tag = "10")]
    pub fee_rate_validity_period: i64,
    /// Asset vaults
    #[prost(message, repeated, tag = "11")]
    pub vaults: ::prost::alloc::vec::Vec<Vault>,
    /// Withdrawal params
    #[prost(message, optional, tag = "12")]
    pub withdraw_params: ::core::option::Option<WithdrawParams>,
    /// Protocol limitations
    #[prost(message, optional, tag = "13")]
    pub protocol_limits: ::core::option::Option<ProtocolLimits>,
    /// Protocol fees
    #[prost(message, optional, tag = "14")]
    pub protocol_fees: ::core::option::Option<ProtocolFees>,
    /// TSS params
    #[prost(message, optional, tag = "15")]
    pub tss_params: ::core::option::Option<TssParams>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Vault defines the asset vault
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Vault {
    /// the vault address for deposit
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    /// public key of the vault
    #[prost(string, tag = "2")]
    pub pub_key: ::prost::alloc::string::String,
    /// the asset type supported by the vault
    #[prost(enumeration = "AssetType", tag = "3")]
    pub asset_type: i32,
    /// version
    #[prost(uint64, tag = "4")]
    pub version: u64,
}
impl ::prost::Name for Vault {
    const NAME: &'static str = "Vault";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawParams {
    /// Maximum number of utxos used to build the signing request; O means unlimited
    #[prost(uint32, tag = "1")]
    pub max_utxo_num: u32,
    /// Period for handling btc withdrawal requests
    #[prost(int64, tag = "2")]
    pub btc_batch_withdraw_period: i64,
    /// Maximum number of btc withdrawal requests to be handled per batch
    #[prost(uint32, tag = "3")]
    pub max_btc_batch_withdraw_num: u32,
}
impl ::prost::Name for WithdrawParams {
    const NAME: &'static str = "WithdrawParams";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// ProtocolLimits defines the params related to the the protocol limitations
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolLimits {
    /// The minimum deposit amount for btc in sat
    #[prost(int64, tag = "1")]
    pub btc_min_deposit: i64,
    /// The minimum withdrawal amount for btc in sat
    #[prost(int64, tag = "2")]
    pub btc_min_withdraw: i64,
    /// The maximum withdrawal amount for btc in sat
    #[prost(int64, tag = "3")]
    pub btc_max_withdraw: i64,
}
impl ::prost::Name for ProtocolLimits {
    const NAME: &'static str = "ProtocolLimits";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// ProtocolFees defines the params related to the protocol fees
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ProtocolFees {
    /// Protocol fee amount for deposit in sat
    #[prost(int64, tag = "1")]
    pub deposit_fee: i64,
    /// Protocol fee amount for withdrawal in sat
    #[prost(int64, tag = "2")]
    pub withdraw_fee: i64,
    /// Protocol fee collector
    #[prost(string, tag = "3")]
    pub collector: ::prost::alloc::string::String,
}
impl ::prost::Name for ProtocolFees {
    const NAME: &'static str = "ProtocolFees";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// TSSParams defines the params related to TSS
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TssParams {
    /// Timeout duration for DKG request
    #[prost(message, optional, tag = "1")]
    pub dkg_timeout_period: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    /// Transition period after which TSS participants update process is completed
    #[prost(message, optional, tag = "2")]
    pub participant_update_transition_period:
        ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
}
impl ::prost::Name for TssParams {
    const NAME: &'static str = "TSSParams";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// AssetType defines the type of asset
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum AssetType {
    /// Unspecified asset type
    Unspecified = 0,
    /// BTC
    Btc = 1,
    /// BRC20: ordi, sats
    Brc20 = 2,
    /// RUNE: dog•go•to•the•moon
    Runes = 3,
}
impl AssetType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            AssetType::Unspecified => "ASSET_TYPE_UNSPECIFIED",
            AssetType::Btc => "ASSET_TYPE_BTC",
            AssetType::Brc20 => "ASSET_TYPE_BRC20",
            AssetType::Runes => "ASSET_TYPE_RUNES",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "ASSET_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "ASSET_TYPE_BTC" => Some(Self::Btc),
            "ASSET_TYPE_BRC20" => Some(Self::Brc20),
            "ASSET_TYPE_RUNES" => Some(Self::Runes),
            _ => None,
        }
    }
}
/// Fee rate
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FeeRate {
    /// fee rate
    #[prost(int64, tag = "1")]
    pub value: i64,
    /// block height at which the fee rate is submitted
    #[prost(int64, tag = "2")]
    pub height: i64,
}
impl ::prost::Name for FeeRate {
    const NAME: &'static str = "FeeRate";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Bitcoin Signing Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SigningRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    #[prost(enumeration = "AssetType", tag = "3")]
    pub r#type: i32,
    #[prost(string, tag = "4")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, tag = "5")]
    pub psbt: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub creation_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "SigningStatus", tag = "7")]
    pub status: i32,
}
impl ::prost::Name for SigningRequest {
    const NAME: &'static str = "SigningRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Compact Signing Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CompactSigningRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub sequence: u64,
    #[prost(enumeration = "AssetType", tag = "3")]
    pub r#type: i32,
    #[prost(string, tag = "4")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub signers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, repeated, tag = "6")]
    pub sig_hashes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(message, optional, tag = "7")]
    pub creation_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "SigningStatus", tag = "8")]
    pub status: i32,
}
impl ::prost::Name for CompactSigningRequest {
    const NAME: &'static str = "CompactSigningRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Withdrawal Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct WithdrawRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint64, tag = "3")]
    pub sequence: u64,
    #[prost(string, tag = "4")]
    pub txid: ::prost::alloc::string::String,
}
impl ::prost::Name for WithdrawRequest {
    const NAME: &'static str = "WithdrawRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Bitcoin UTXO
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Utxo {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub vout: u64,
    #[prost(string, tag = "3")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "4")]
    pub amount: u64,
    #[prost(uint64, tag = "5")]
    pub height: u64,
    #[prost(bytes = "vec", tag = "6")]
    pub pub_key_script: ::prost::alloc::vec::Vec<u8>,
    #[prost(bool, tag = "7")]
    pub is_locked: bool,
    /// rune balances associated with the UTXO
    #[prost(message, repeated, tag = "8")]
    pub runes: ::prost::alloc::vec::Vec<RuneBalance>,
}
impl ::prost::Name for Utxo {
    const NAME: &'static str = "UTXO";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Rune Balance
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneBalance {
    /// serialized rune id
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    /// rune amount
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for RuneBalance {
    const NAME: &'static str = "RuneBalance";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Rune ID
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RuneId {
    /// block height
    #[prost(uint64, tag = "1")]
    pub block: u64,
    /// tx index
    #[prost(uint32, tag = "2")]
    pub tx: u32,
}
impl ::prost::Name for RuneId {
    const NAME: &'static str = "RuneId";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Rune Edict
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Edict {
    #[prost(message, optional, tag = "1")]
    pub id: ::core::option::Option<RuneId>,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(uint32, tag = "3")]
    pub output: u32,
}
impl ::prost::Name for Edict {
    const NAME: &'static str = "Edict";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// BTC UTXO Consolidation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BtcConsolidation {
    /// maximum threshold of the btc value
    #[prost(int64, tag = "1")]
    pub target_threshold: i64,
    /// maximum number of the utxos to be consolidated; 0 means all
    #[prost(uint32, tag = "2")]
    pub max_num: u32,
}
impl ::prost::Name for BtcConsolidation {
    const NAME: &'static str = "BtcConsolidation";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Runes UTXO Consolidation
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RunesConsolidation {
    /// rune id
    #[prost(string, tag = "1")]
    pub rune_id: ::prost::alloc::string::String,
    /// maximum threshold of the corresponding rune balance
    #[prost(string, tag = "2")]
    pub target_threshold: ::prost::alloc::string::String,
    /// maximum number of the utxos to be consolidated; 0 means all
    #[prost(uint32, tag = "3")]
    pub max_num: u32,
}
impl ::prost::Name for RunesConsolidation {
    const NAME: &'static str = "RunesConsolidation";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// DKG Participant
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgParticipant {
    /// the moniker of the corresponding validator
    #[prost(string, tag = "1")]
    pub moniker: ::prost::alloc::string::String,
    /// the operator address of the corresponding validator
    #[prost(string, tag = "2")]
    pub operator_address: ::prost::alloc::string::String,
    /// the consensus public key of the corresponding validator
    #[prost(string, tag = "3")]
    pub consensus_pubkey: ::prost::alloc::string::String,
}
impl ::prost::Name for DkgParticipant {
    const NAME: &'static str = "DKGParticipant";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// DKG Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgRequest {
    /// the unique request id
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// participant set
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<DkgParticipant>,
    /// threshold required to perform DKG
    #[prost(uint32, tag = "3")]
    pub threshold: u32,
    /// asset types of vaults to be generated
    #[prost(enumeration = "AssetType", repeated, tag = "4")]
    pub vault_types: ::prost::alloc::vec::Vec<i32>,
    /// indicates if transferring assets to the newly generated vaults when the DKG request is completed
    #[prost(bool, tag = "5")]
    pub enable_transfer: bool,
    /// target number of the UTXOs to be transferred each time
    #[prost(uint32, tag = "6")]
    pub target_utxo_num: u32,
    /// expiration time
    #[prost(message, optional, tag = "7")]
    pub expiration: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    /// status
    #[prost(enumeration = "DkgRequestStatus", tag = "8")]
    pub status: i32,
}
impl ::prost::Name for DkgRequest {
    const NAME: &'static str = "DKGRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// DKG Completion Request
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DkgCompletionRequest {
    /// request id
    #[prost(uint64, tag = "1")]
    pub id: u64,
    /// sender
    #[prost(string, tag = "2")]
    pub sender: ::prost::alloc::string::String,
    /// new vaults generated by DKG
    #[prost(string, repeated, tag = "3")]
    pub vaults: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// participant consensus pub key
    #[prost(string, tag = "4")]
    pub consensus_pubkey: ::prost::alloc::string::String,
    /// hex encoded participant signature
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for DkgCompletionRequest {
    const NAME: &'static str = "DKGCompletionRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// Bitcoin Signing Status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum SigningStatus {
    /// SIGNING_STATUS_UNSPECIFIED - Default value, should not be used
    Unspecified = 0,
    /// SIGNING_STATUS_PENDING - The signing request is pending
    Pending = 1,
    /// SIGNING_STATUS_BROADCASTED - The signing request is broadcasted
    Broadcasted = 2,
    /// SIGNING_STATUS_CONFIRMED - The signing request is confirmed
    Confirmed = 3,
    /// SIGNING_STATUS_FAILED - The signing request failed to be signed or broadcast due to unexpected exceptions
    Failed = 4,
}
impl SigningStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            SigningStatus::Unspecified => "SIGNING_STATUS_UNSPECIFIED",
            SigningStatus::Pending => "SIGNING_STATUS_PENDING",
            SigningStatus::Broadcasted => "SIGNING_STATUS_BROADCASTED",
            SigningStatus::Confirmed => "SIGNING_STATUS_CONFIRMED",
            SigningStatus::Failed => "SIGNING_STATUS_FAILED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "SIGNING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "SIGNING_STATUS_PENDING" => Some(Self::Pending),
            "SIGNING_STATUS_BROADCASTED" => Some(Self::Broadcasted),
            "SIGNING_STATUS_CONFIRMED" => Some(Self::Confirmed),
            "SIGNING_STATUS_FAILED" => Some(Self::Failed),
            _ => None,
        }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum DkgRequestStatus {
    /// DKG_REQUEST_STATUS_UNSPECIFIED defines the unknown DKG request status
    Unspecified = 0,
    /// DKG_REQUEST_STATUS_PENDING defines the status of the DKG request which is pending
    Pending = 1,
    /// DKG_REQUEST_STATUS_COMPLETED defines the status of the DKG request which is completed
    Completed = 2,
    /// DKG_REQUEST_STATUS_FAILED defines the status of the DKG request which failed
    Failed = 3,
    /// DKG_REQUEST_STATUS_TIMEDOUT defines the status of the DKG request which timed out
    Timedout = 4,
}
impl DkgRequestStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            DkgRequestStatus::Unspecified => "DKG_REQUEST_STATUS_UNSPECIFIED",
            DkgRequestStatus::Pending => "DKG_REQUEST_STATUS_PENDING",
            DkgRequestStatus::Completed => "DKG_REQUEST_STATUS_COMPLETED",
            DkgRequestStatus::Failed => "DKG_REQUEST_STATUS_FAILED",
            DkgRequestStatus::Timedout => "DKG_REQUEST_STATUS_TIMEDOUT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "DKG_REQUEST_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "DKG_REQUEST_STATUS_PENDING" => Some(Self::Pending),
            "DKG_REQUEST_STATUS_COMPLETED" => Some(Self::Completed),
            "DKG_REQUEST_STATUS_FAILED" => Some(Self::Failed),
            "DKG_REQUEST_STATUS_TIMEDOUT" => Some(Self::Timedout),
            _ => None,
        }
    }
}
/// GenesisState defines the btc bridge module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
    #[prost(message, optional, tag = "3")]
    pub dkg_request: ::core::option::Option<DkgRequest>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryWithdrawRequestsByAddressRequest is request type for the Query/WithdrawRequestsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryWithdrawRequestsByAddressRequest {
    const NAME: &'static str = "QueryWithdrawRequestsByAddressRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryWithdrawRequestsByAddressResponse is response type for the Query/WithdrawRequestsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryWithdrawRequestsByAddressResponse {
    const NAME: &'static str = "QueryWithdrawRequestsByAddressResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryWithdrawRequestsByTxHashRequest is request type for the Query/WithdrawRequestsByTxHash RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByTxHashRequest {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryWithdrawRequestsByTxHashRequest {
    const NAME: &'static str = "QueryWithdrawRequestsByTxHashRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryWithdrawRequestsByTxHashResponse is response type for the Query/WithdrawRequestsByTxHash RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawRequestsByTxHashResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
}
impl ::prost::Name for QueryWithdrawRequestsByTxHashResponse {
    const NAME: &'static str = "QueryWithdrawRequestsByTxHashResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryPendingBtcWithdrawRequestsRequest is request type for the Query/PendingBtcWithdrawRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingBtcWithdrawRequestsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryPendingBtcWithdrawRequestsRequest {
    const NAME: &'static str = "QueryPendingBtcWithdrawRequestsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryPendingBtcWithdrawRequestsResponse is response type for the Query/PendingBtcWithdrawRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingBtcWithdrawRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<WithdrawRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryPendingBtcWithdrawRequestsResponse {
    const NAME: &'static str = "QueryPendingBtcWithdrawRequestsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestRequest is request type for the Query/SigningRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestRequest {
    #[prost(uint64, tag = "1")]
    pub sequence: u64,
}
impl ::prost::Name for QuerySigningRequestRequest {
    const NAME: &'static str = "QuerySigningRequestRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestResponse is response type for the Query/SigningRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<SigningRequest>,
}
impl ::prost::Name for QuerySigningRequestResponse {
    const NAME: &'static str = "QuerySigningRequestResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestsRequest is request type for the Query/SigningRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsRequest {
    #[prost(enumeration = "SigningStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QuerySigningRequestsRequest {
    const NAME: &'static str = "QuerySigningRequestsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestsResponse is response type for the Query/SigningRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<SigningRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QuerySigningRequestsResponse {
    const NAME: &'static str = "QuerySigningRequestsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestsByAddressRequest is request type for the Query/SigningRequestsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QuerySigningRequestsByAddressRequest {
    const NAME: &'static str = "QuerySigningRequestsByAddressRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestsByAddressResponse is response type for the Query/SigningRequestsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<SigningRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QuerySigningRequestsByAddressResponse {
    const NAME: &'static str = "QuerySigningRequestsByAddressResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestByTxHashRequest is request type for the Query/SigningRequestByTxHash RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestByTxHashRequest {
    #[prost(string, tag = "1")]
    pub txid: ::prost::alloc::string::String,
}
impl ::prost::Name for QuerySigningRequestByTxHashRequest {
    const NAME: &'static str = "QuerySigningRequestByTxHashRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QuerySigningRequestByTxHashResponse is response type for the Query/SigningRequestByTxHashResponse RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QuerySigningRequestByTxHashResponse {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<SigningRequest>,
}
impl ::prost::Name for QuerySigningRequestByTxHashResponse {
    const NAME: &'static str = "QuerySigningRequestByTxHashResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryPendingSigningRequestsRequest is request type for the Query/PendingSigningRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSigningRequestsRequest {
    #[prost(message, optional, tag = "1")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryPendingSigningRequestsRequest {
    const NAME: &'static str = "QueryPendingSigningRequestsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryPendingSigningRequestsResponse is response type for the Query/PendingSigningRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingSigningRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<CompactSigningRequest>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryPendingSigningRequestsResponse {
    const NAME: &'static str = "QueryPendingSigningRequestsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryFeeRateRequest is request type for the Query/FeeRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeRateRequest {}
impl ::prost::Name for QueryFeeRateRequest {
    const NAME: &'static str = "QueryFeeRateRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryFeeRateResponse is response type for the Query/FeeRate RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryFeeRateResponse {
    #[prost(message, optional, tag = "1")]
    pub fee_rate: ::core::option::Option<FeeRate>,
}
impl ::prost::Name for QueryFeeRateResponse {
    const NAME: &'static str = "QueryFeeRateResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryWithdrawalNetworkFeeRequest is request type for the Query/WithdrawalNetworkFee RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalNetworkFeeRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(int64, tag = "3")]
    pub fee_rate: i64,
}
impl ::prost::Name for QueryWithdrawalNetworkFeeRequest {
    const NAME: &'static str = "QueryWithdrawalNetworkFeeRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryWithdrawalNetworkFeeResponse is response type for the Query/WithdrawalNetworkFee RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryWithdrawalNetworkFeeResponse {
    #[prost(int64, tag = "1")]
    pub fee_rate: i64,
    #[prost(string, tag = "2")]
    pub fee: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryWithdrawalNetworkFeeResponse {
    const NAME: &'static str = "QueryWithdrawalNetworkFeeResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryUTXOsRequest is the request type for the Query/UTXOs RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsRequest {}
impl ::prost::Name for QueryUtxOsRequest {
    const NAME: &'static str = "QueryUTXOsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryUTXOsResponse is the response type for the Query/UTXOs RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsResponse {
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
}
impl ::prost::Name for QueryUtxOsResponse {
    const NAME: &'static str = "QueryUTXOsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryUTXOsByAddressRequest is the request type for the Query/UTXOsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryUtxOsByAddressRequest {
    const NAME: &'static str = "QueryUTXOsByAddressRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryUTXOsByAddressResponse is the response type for the Query/UTXOsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxOsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub utxos: ::prost::alloc::vec::Vec<Utxo>,
}
impl ::prost::Name for QueryUtxOsByAddressResponse {
    const NAME: &'static str = "QueryUTXOsByAddressResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryUTXOCountAndBalancesByAddressRequest is the request type for the Query/UTXOCountAndBalancesByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxoCountAndBalancesByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryUtxoCountAndBalancesByAddressRequest {
    const NAME: &'static str = "QueryUTXOCountAndBalancesByAddressRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryUTXOCountAndBalancesByAddressResponse is the response type for the Query/UTXOCountAndBalancesByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryUtxoCountAndBalancesByAddressResponse {
    #[prost(uint32, tag = "1")]
    pub count: u32,
    #[prost(int64, tag = "2")]
    pub value: i64,
    #[prost(message, repeated, tag = "3")]
    pub rune_balances: ::prost::alloc::vec::Vec<RuneBalance>,
}
impl ::prost::Name for QueryUtxoCountAndBalancesByAddressResponse {
    const NAME: &'static str = "QueryUTXOCountAndBalancesByAddressResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryDKGRequestRequest is the request type for the Query/DKGRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryDkgRequestRequest {
    const NAME: &'static str = "QueryDKGRequestRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryDKGRequestResponse is the response type for the Query/DKGRequest RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestResponse {
    #[prost(message, optional, tag = "1")]
    pub request: ::core::option::Option<DkgRequest>,
}
impl ::prost::Name for QueryDkgRequestResponse {
    const NAME: &'static str = "QueryDKGRequestResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryDKGRequestsRequest is the request type for the Query/DKGRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestsRequest {
    #[prost(enumeration = "DkgRequestStatus", tag = "1")]
    pub status: i32,
}
impl ::prost::Name for QueryDkgRequestsRequest {
    const NAME: &'static str = "QueryDKGRequestsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryDKGRequestsResponse is the response type for the Query/DKGRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DkgRequest>,
}
impl ::prost::Name for QueryDkgRequestsResponse {
    const NAME: &'static str = "QueryDKGRequestsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryAllDKGRequestsRequest is the request type for the Query/AllDKGRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllDkgRequestsRequest {}
impl ::prost::Name for QueryAllDkgRequestsRequest {
    const NAME: &'static str = "QueryAllDKGRequestsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryAllDKGRequestsResponse is the response type for the Query/AllDKGRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryAllDkgRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DkgRequest>,
}
impl ::prost::Name for QueryAllDkgRequestsResponse {
    const NAME: &'static str = "QueryAllDKGRequestsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryDKGCompletionRequestsRequest is the request type for the Query/DKGCompletionRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgCompletionRequestsRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryDkgCompletionRequestsRequest {
    const NAME: &'static str = "QueryDKGCompletionRequestsRequest";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// QueryDKGCompletionRequestsResponse is the response type for the Query/DKGCompletionRequests RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryDkgCompletionRequestsResponse {
    #[prost(message, repeated, tag = "1")]
    pub requests: ::prost::alloc::vec::Vec<DkgCompletionRequest>,
}
impl ::prost::Name for QueryDkgCompletionRequestsResponse {
    const NAME: &'static str = "QueryDKGCompletionRequestsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitDepositTransaction defines the Msg/SubmitDepositTransaction request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDepositTransaction {
    /// this is the relayer address who submits the bitcoin transaction to the side chain
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blockhash: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    /// used for parsing the sender of the transaction
    #[prost(string, tag = "3")]
    pub prev_tx_bytes: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    #[prost(string, tag = "4")]
    pub tx_bytes: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "5")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitDepositTransaction {
    const NAME: &'static str = "MsgSubmitDepositTransaction";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitDepositTransactionResponse defines the Msg/SubmitDepositTransaction response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitDepositTransactionResponse {}
impl ::prost::Name for MsgSubmitDepositTransactionResponse {
    const NAME: &'static str = "MsgSubmitDepositTransactionResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitWithdrawTransaction defines the Msg/SubmitWithdrawTransaction request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawTransaction {
    /// this is the relayer address who submits the bitcoin transaction to the side chain
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub blockhash: ::prost::alloc::string::String,
    /// the tx bytes in base64 format
    #[prost(string, tag = "3")]
    pub tx_bytes: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "4")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitWithdrawTransaction {
    const NAME: &'static str = "MsgSubmitWithdrawTransaction";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitWithdrawTransactionResponse defines the Msg/SubmitWithdrawTransaction response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitWithdrawTransactionResponse {}
impl ::prost::Name for MsgSubmitWithdrawTransactionResponse {
    const NAME: &'static str = "MsgSubmitWithdrawTransactionResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitFeeRate defines the Msg/SubmitFeeRate request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitFeeRate {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(int64, tag = "2")]
    pub fee_rate: i64,
}
impl ::prost::Name for MsgSubmitFeeRate {
    const NAME: &'static str = "MsgSubmitFeeRate";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitFeeRateResponse defines the Msg/SubmitFeeRate response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitFeeRateResponse {}
impl ::prost::Name for MsgSubmitFeeRateResponse {
    const NAME: &'static str = "MsgSubmitFeeRateResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgUpdateTrustedNonBtcRelayers defines the Msg/UpdateTrustedNonBtcRelayers request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedNonBtcRelayers {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub relayers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgUpdateTrustedNonBtcRelayers {
    const NAME: &'static str = "MsgUpdateTrustedNonBtcRelayers";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgUpdateTrustedNonBtcRelayersResponse defines the Msg/UpdateTrustedNonBtcRelayers response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedNonBtcRelayersResponse {}
impl ::prost::Name for MsgUpdateTrustedNonBtcRelayersResponse {
    const NAME: &'static str = "MsgUpdateTrustedNonBtcRelayersResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgUpdateTrustedFeeProviders defines the Msg/UpdateTrustedFeeProviders request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedFeeProviders {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "2")]
    pub fee_providers: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgUpdateTrustedFeeProviders {
    const NAME: &'static str = "MsgUpdateTrustedFeeProviders";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgUpdateTrustedFeeProvidersResponse defines the Msg/UpdateTrustedFeeProviders response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUpdateTrustedFeeProvidersResponse {}
impl ::prost::Name for MsgUpdateTrustedFeeProvidersResponse {
    const NAME: &'static str = "MsgUpdateTrustedFeeProvidersResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgWithdrawToBitcoin defines the Msg/WithdrawToBitcoin request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawToBitcoin {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// withdraw amount in satoshi, etc: 100000000sat = 1btc
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgWithdrawToBitcoin {
    const NAME: &'static str = "MsgWithdrawToBitcoin";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgWithdrawToBitcoinResponse defines the Msg/WithdrawToBitcoin response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgWithdrawToBitcoinResponse {}
impl ::prost::Name for MsgWithdrawToBitcoinResponse {
    const NAME: &'static str = "MsgWithdrawToBitcoinResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitSignatures defines the Msg/SubmitSignatures request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitSignatures {
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub txid: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub signatures: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
impl ::prost::Name for MsgSubmitSignatures {
    const NAME: &'static str = "MsgSubmitSignatures";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgSubmitSignaturesResponse defines the Msg/SubmitSignatures response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgSubmitSignaturesResponse {}
impl ::prost::Name for MsgSubmitSignaturesResponse {
    const NAME: &'static str = "MsgSubmitSignaturesResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgConsolidateVaults is the Msg/ConsolidateVaults request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConsolidateVaults {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// vault version
    #[prost(uint64, tag = "2")]
    pub vault_version: u64,
    /// btc consolidation
    #[prost(message, optional, tag = "3")]
    pub btc_consolidation: ::core::option::Option<BtcConsolidation>,
    /// runes consolidations
    #[prost(message, repeated, tag = "4")]
    pub runes_consolidations: ::prost::alloc::vec::Vec<RunesConsolidation>,
}
impl ::prost::Name for MsgConsolidateVaults {
    const NAME: &'static str = "MsgConsolidateVaults";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgConsolidateVaultsResponse defines the Msg/ConsolidateVaults response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgConsolidateVaultsResponse {}
impl ::prost::Name for MsgConsolidateVaultsResponse {
    const NAME: &'static str = "MsgConsolidateVaultsResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgInitiateDKG is the Msg/InitiateDKG request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInitiateDkg {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// expected participant set
    #[prost(message, repeated, tag = "2")]
    pub participants: ::prost::alloc::vec::Vec<DkgParticipant>,
    /// threshold required to perform DKG
    #[prost(uint32, tag = "3")]
    pub threshold: u32,
    /// asset types of vaults to be generated
    #[prost(enumeration = "AssetType", repeated, tag = "4")]
    pub vault_types: ::prost::alloc::vec::Vec<i32>,
    /// indicates if transferring the current vaults to the newly generated vaults when the DKG request is completed
    #[prost(bool, tag = "5")]
    pub enable_transfer: bool,
    /// target number of the UTXOs to be transferred each time
    #[prost(uint32, tag = "6")]
    pub target_utxo_num: u32,
}
impl ::prost::Name for MsgInitiateDkg {
    const NAME: &'static str = "MsgInitiateDKG";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgInitiateDKGResponse defines the Msg/InitiateDKG response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgInitiateDkgResponse {}
impl ::prost::Name for MsgInitiateDkgResponse {
    const NAME: &'static str = "MsgInitiateDKGResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgCompleteDKG is the Msg/CompleteDKG request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCompleteDkg {
    /// the sender
    #[prost(string, tag = "1")]
    pub sender: ::prost::alloc::string::String,
    /// DKG request id
    #[prost(uint64, tag = "2")]
    pub id: u64,
    /// new vaults generated by DKG
    #[prost(string, repeated, tag = "3")]
    pub vaults: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// participant consensus pub key
    #[prost(string, tag = "4")]
    pub consensus_pubkey: ::prost::alloc::string::String,
    /// hex encoded participant signature
    #[prost(string, tag = "5")]
    pub signature: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgCompleteDkg {
    const NAME: &'static str = "MsgCompleteDKG";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgCompleteDKGResponse defines the Msg/CompleteDKG response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgCompleteDkgResponse {}
impl ::prost::Name for MsgCompleteDkgResponse {
    const NAME: &'static str = "MsgCompleteDKGResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgTransferVault is the Msg/TransferVault request type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferVault {
    /// authority is the address that controls the module (defaults to x/gov unless overwritten).
    #[prost(string, tag = "1")]
    pub authority: ::prost::alloc::string::String,
    /// version of the source vault
    #[prost(uint64, tag = "2")]
    pub source_version: u64,
    /// version of the destination vault
    #[prost(uint64, tag = "3")]
    pub dest_version: u64,
    /// asset type
    #[prost(enumeration = "AssetType", tag = "4")]
    pub asset_type: i32,
    /// a set of optional pre-built PSBTs to perform the asset transfer
    #[prost(string, repeated, tag = "5")]
    pub psbts: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// target number of the UTXOs to be transferred; only take effect when psbt not provided
    #[prost(uint32, tag = "6")]
    pub target_utxo_num: u32,
}
impl ::prost::Name for MsgTransferVault {
    const NAME: &'static str = "MsgTransferVault";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
/// MsgTransferVaultResponse defines the Msg/TransferVault response type.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgTransferVaultResponse {}
impl ::prost::Name for MsgTransferVaultResponse {
    const NAME: &'static str = "MsgTransferVaultResponse";
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.btcbridge";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.btcbridge.{}", Self::NAME)
    }
}
include!("side.btcbridge.serde.rs");
include!("side.btcbridge.tonic.rs");
// @@protoc_insertion_point(module)
