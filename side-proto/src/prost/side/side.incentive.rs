// @generated
/// Params defines the parameters for the module.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Params {
    /// Indicates if the incentive mechanism is enabled
    #[prost(bool, tag = "1")]
    pub enabled: bool,
    /// Reward per deposit tx via btc bridge
    #[prost(message, optional, tag = "2")]
    pub reward_per_deposit: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    /// Reward per withdrawal tx via btc bridge
    #[prost(message, optional, tag = "3")]
    pub reward_per_withdraw: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for Params {
    const NAME: &'static str = "Params";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// GenesisState defines the incentive module's genesis state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GenesisState {
    #[prost(message, optional, tag = "1")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for GenesisState {
    const NAME: &'static str = "GenesisState";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// Rewards
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rewards {
    #[prost(string, tag = "1")]
    pub address: ::prost::alloc::string::String,
    #[prost(uint64, tag = "2")]
    pub deposit_count: u64,
    #[prost(uint64, tag = "3")]
    pub withdraw_count: u64,
    #[prost(message, optional, tag = "4")]
    pub deposit_reward: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "5")]
    pub withdraw_reward: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
    #[prost(message, optional, tag = "6")]
    pub total_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for Rewards {
    const NAME: &'static str = "Rewards";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// Reward Statistics
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RewardStats {
    #[prost(uint64, tag = "1")]
    pub address_count: u64,
    #[prost(uint64, tag = "2")]
    pub tx_count: u64,
    #[prost(message, optional, tag = "3")]
    pub total_reward_amount: ::core::option::Option<super::super::cosmos::base::v1beta1::Coin>,
}
impl ::prost::Name for RewardStats {
    const NAME: &'static str = "RewardStats";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// QueryRewardsResponse is response type for the Query/Rewards RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardsResponse {
    #[prost(message, optional, tag = "1")]
    pub rewards: ::core::option::Option<Rewards>,
}
impl ::prost::Name for QueryRewardsResponse {
    const NAME: &'static str = "QueryRewardsResponse";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// QueryRewardStatsRequest is request type for the Query/RewardStats RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardStatsRequest {}
impl ::prost::Name for QueryRewardStatsRequest {
    const NAME: &'static str = "QueryRewardStatsRequest";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// QueryRewardStatsResponse is response type for the Query/RewardStats RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryRewardStatsResponse {
    #[prost(message, optional, tag = "1")]
    pub reward_stats: ::core::option::Option<RewardStats>,
}
impl ::prost::Name for QueryRewardStatsResponse {
    const NAME: &'static str = "QueryRewardStatsResponse";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
/// QueryParamsRequest is request type for the Query/Params RPC method.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct QueryParamsRequest {}
impl ::prost::Name for QueryParamsRequest {
    const NAME: &'static str = "QueryParamsRequest";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
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
    /// params defines the x/incentive parameters to be updated.
    ///
    /// NOTE: All parameters must be supplied.
    #[prost(message, optional, tag = "2")]
    pub params: ::core::option::Option<Params>,
}
impl ::prost::Name for MsgUpdateParams {
    const NAME: &'static str = "MsgUpdateParams";
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
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
    const PACKAGE: &'static str = "side.incentive";
    fn full_name() -> ::prost::alloc::string::String {
        ::prost::alloc::format!("side.incentive.{}", Self::NAME)
    }
}
include!("side.incentive.serde.rs");
include!("side.incentive.tonic.rs");
// @@protoc_insertion_point(module)
