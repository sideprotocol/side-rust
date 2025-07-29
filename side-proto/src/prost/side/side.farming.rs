// @generated
/// Epoch defines the epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Epoch {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(message, optional, tag = "2")]
    pub start_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(message, optional, tag = "3")]
    pub end_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(message, repeated, tag = "4")]
    pub total_stakings: ::prost::alloc::vec::Vec<TotalStaking>,
    #[prost(enumeration = "EpochStatus", tag = "5")]
    pub status: i32,
}
impl ::prost::Name for Epoch {
    const NAME: &'static str = "Epoch";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// Staking defines the staking
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Staking {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(string, tag = "2")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "4")]
    pub lock_duration: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    #[prost(string, tag = "5")]
    pub lock_multiplier: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "6")]
    pub effective_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "7")]
    pub pending_rewards: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "8")]
    pub total_rewards: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "9")]
    pub start_time: ::core::option::Option<::tendermint_proto::google::protobuf::Timestamp>,
    #[prost(enumeration = "StakingStatus", tag = "10")]
    pub status: i32,
}
impl ::prost::Name for Staking {
    const NAME: &'static str = "Staking";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// TotalStaking defines total staking per denom
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TotalStaking {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "3")]
    pub effective_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for TotalStaking {
    const NAME: &'static str = "TotalStaking";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// AccountRewardPerEpoch defines the account reward per epoch
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountRewardPerEpoch {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(message, repeated, tag = "2")]
    pub stakings: ::prost::alloc::vec::Vec<TotalStaking>,
    #[prost(string, repeated, tag = "3")]
    pub shares: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(string, tag = "4")]
    pub total_share: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "5")]
    pub reward: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for AccountRewardPerEpoch {
    const NAME: &'static str = "AccountRewardPerEpoch";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// Epoch status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum EpochStatus {
    Pending = 0,
    Started = 1,
    Ended = 2,
}
impl EpochStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            EpochStatus::Pending => "EPOCH_STATUS_PENDING",
            EpochStatus::Started => "EPOCH_STATUS_STARTED",
            EpochStatus::Ended => "EPOCH_STATUS_ENDED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "EPOCH_STATUS_PENDING" => Some(Self::Pending),
            "EPOCH_STATUS_STARTED" => Some(Self::Started),
            "EPOCH_STATUS_ENDED" => Some(Self::Ended),
            _ => None,
        }
    }
}
/// Staking status
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum StakingStatus {
    Unspecified = 0,
    Staked = 1,
    Unlocked = 2,
    Unstaked = 3,
}
impl StakingStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            StakingStatus::Unspecified => "STAKING_STATUS_UNSPECIFIED",
            StakingStatus::Staked => "STAKING_STATUS_STAKED",
            StakingStatus::Unlocked => "STAKING_STATUS_UNLOCKED",
            StakingStatus::Unstaked => "STAKING_STATUS_UNSTAKED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "STAKING_STATUS_UNSPECIFIED" => Some(Self::Unspecified),
            "STAKING_STATUS_STAKED" => Some(Self::Staked),
            "STAKING_STATUS_UNLOCKED" => Some(Self::Unlocked),
            "STAKING_STATUS_UNSTAKED" => Some(Self::Unstaked),
            _ => None,
        }
    }
}
/// Asset defines the farming asset
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    /// Asset denom
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
    /// Ratio of the reward relative to the total rewards
    #[prost(string, tag = "2")]
    pub reward_ratio: ::prost::alloc::string::String,
}
impl ::prost::Name for Asset {
    const NAME: &'static str = "Asset";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    #[prost(message, optional, tag = "2")]
    pub epoch_duration: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
    #[prost(message, optional, tag = "3")]
    pub reward_per_epoch: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, repeated, tag = "4")]
    pub lock_durations: ::prost::alloc::vec::Vec<::tendermint_proto::google::protobuf::Duration>,
    #[prost(message, repeated, tag = "5")]
    pub eligible_assets: ::prost::alloc::vec::Vec<Asset>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// GenesisState defines the farming module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
    #[prost(message, repeated, tag = "2")]
    pub stakings: ::prost::alloc::vec::Vec<Staking>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryStakingRequest is request type for the Query/Staking RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStakingRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryStakingRequest {
    const NAME: &'static str = "QueryStakingRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryStakingResponse is response type for the Query/Staking RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStakingResponse {
    #[prost(message, optional, tag = "1")]
    pub staking: ::core::option::Option<Staking>,
}
impl ::prost::Name for QueryStakingResponse {
    const NAME: &'static str = "QueryStakingResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryStakingsRequest is request type for the Query/Stakings RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStakingsRequest {
    #[prost(enumeration = "StakingStatus", tag = "1")]
    pub status: i32,
    #[prost(message, optional, tag = "2")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryStakingsRequest {
    const NAME: &'static str = "QueryStakingsRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryStakingsResponse is response type for the Query/Stakings RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStakingsResponse {
    #[prost(message, repeated, tag = "1")]
    pub stakings: ::prost::alloc::vec::Vec<Staking>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryStakingsResponse {
    const NAME: &'static str = "QueryStakingsResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryStakingsByAddressRequest is request type for the Query/StakingsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStakingsByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(enumeration = "StakingStatus", tag = "2")]
    pub status: i32,
    #[prost(message, optional, tag = "3")]
    pub pagination: ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageRequest>,
}
impl ::prost::Name for QueryStakingsByAddressRequest {
    const NAME: &'static str = "QueryStakingsByAddressRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryStakingsByAddressResponse is response type for the Query/StakingsByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryStakingsByAddressResponse {
    #[prost(message, repeated, tag = "1")]
    pub stakings: ::prost::alloc::vec::Vec<Staking>,
    #[prost(message, optional, tag = "2")]
    pub pagination:
        ::core::option::Option<super::super::cosmos::base::query::v1beta1::PageResponse>,
}
impl ::prost::Name for QueryStakingsByAddressResponse {
    const NAME: &'static str = "QueryStakingsByAddressResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryTotalStakingRequest is request type for the Query/TotalStaking RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalStakingRequest {
    #[prost(string, tag = "1")]
    pub denom: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryTotalStakingRequest {
    const NAME: &'static str = "QueryTotalStakingRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryTotalStakingResponse is response type for the Query/TotalStaking RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryTotalStakingResponse {
    #[prost(message, optional, tag = "1")]
    pub total_staking: ::core::option::Option<TotalStaking>,
}
impl ::prost::Name for QueryTotalStakingResponse {
    const NAME: &'static str = "QueryTotalStakingResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryCurrentEpochRequest is request type for the Query/CurrentEpoch RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochRequest {}
impl ::prost::Name for QueryCurrentEpochRequest {
    const NAME: &'static str = "QueryCurrentEpochRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryCurrentEpochResponse is response type for the Query/CurrentEpoch RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryCurrentEpochResponse {
    #[prost(message, optional, tag = "1")]
    pub current_epoch: ::core::option::Option<Epoch>,
}
impl ::prost::Name for QueryCurrentEpochResponse {
    const NAME: &'static str = "QueryCurrentEpochResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryRewardsRequest is request type for the Query/Rewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryRewardsRequest {
    const NAME: &'static str = "QueryRewardsRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryRewardsResponse is response type for the Query/Rewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsResponse {
    #[prost(string, tag = "1")]
    pub pending_rewards: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub total_rewards: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryRewardsResponse {
    const NAME: &'static str = "QueryRewardsResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryPendingRewardRequest is request type for the Query/PendingReward RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingRewardRequest {
    #[prost(uint64, tag = "1")]
    pub id: u64,
}
impl ::prost::Name for QueryPendingRewardRequest {
    const NAME: &'static str = "QueryPendingRewardRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryPendingRewardResponse is response type for the Query/PendingReward RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingRewardResponse {
    #[prost(string, tag = "1")]
    pub pending_reward: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPendingRewardResponse {
    const NAME: &'static str = "QueryPendingRewardResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryPendingRewardByAddressRequest is request type for the Query/PendingRewardByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingRewardByAddressRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
}
impl ::prost::Name for QueryPendingRewardByAddressRequest {
    const NAME: &'static str = "QueryPendingRewardByAddressRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryPendingRewardByAddressResponse is response type for the Query/PendingRewardByAddress RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryPendingRewardByAddressResponse {
    #[prost(message, optional, tag = "1")]
    pub pending_reward: ::core::option::Option<AccountRewardPerEpoch>,
}
impl ::prost::Name for QueryPendingRewardByAddressResponse {
    const NAME: &'static str = "QueryPendingRewardByAddressResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryEstimatedRewardRequest is request type for the Query/EstimatedReward RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimatedRewardRequest {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub amount: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "3")]
    pub lock_duration: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
}
impl ::prost::Name for QueryEstimatedRewardRequest {
    const NAME: &'static str = "QueryEstimatedRewardRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryEstimatedRewardResponse is response type for the Query/EstimatedReward RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryEstimatedRewardResponse {
    #[prost(message, optional, tag = "1")]
    pub reward: ::core::option::Option<AccountRewardPerEpoch>,
}
impl ::prost::Name for QueryEstimatedRewardResponse {
    const NAME: &'static str = "QueryEstimatedRewardResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStake {
    /// Staker address
    #[prost(string, tag = "1")]
    pub staker: ::prost::alloc::string::String,
    /// Staked amount
    #[prost(message, optional, tag = "2")]
    pub amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Lock duration
    #[prost(message, optional, tag = "3")]
    pub lock_duration: ::core::option::Option<::tendermint_proto::google::protobuf::Duration>,
}
impl ::prost::Name for MsgStake {
    const NAME: &'static str = "MsgStake";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgStakeResponse {}
impl ::prost::Name for MsgStakeResponse {
    const NAME: &'static str = "MsgStakeResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnstake {
    /// Staker address
    #[prost(string, tag = "1")]
    pub staker: ::prost::alloc::string::String,
    /// Staking id
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
impl ::prost::Name for MsgUnstake {
    const NAME: &'static str = "MsgUnstake";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgUnstakeResponse {}
impl ::prost::Name for MsgUnstakeResponse {
    const NAME: &'static str = "MsgUnstakeResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaim {
    /// Staker address
    #[prost(string, tag = "1")]
    pub staker: ::prost::alloc::string::String,
    /// Staking id
    #[prost(uint64, tag = "2")]
    pub id: u64,
}
impl ::prost::Name for MsgClaim {
    const NAME: &'static str = "MsgClaim";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimResponse {}
impl ::prost::Name for MsgClaimResponse {
    const NAME: &'static str = "MsgClaimResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimAll {
    /// Staker address
    #[prost(string, tag = "1")]
    pub staker: ::prost::alloc::string::String,
}
impl ::prost::Name for MsgClaimAll {
    const NAME: &'static str = "MsgClaimAll";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MsgClaimAllResponse {}
impl ::prost::Name for MsgClaimAllResponse {
    const NAME: &'static str = "MsgClaimAllResponse";
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.farming";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.farming.{}", Self::NAME)
    }
}
include!("side.farming.serde.rs");
include!("side.farming.tonic.rs");
// @@protoc_insertion_point(module)
