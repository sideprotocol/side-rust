#![doc = include_str!("../README.md")]
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/cosmos/cosmos-rust/main/.images/cosmos.png"
)]
#![cfg_attr(docsrs, feature(doc_auto_cfg))]
#![allow(
    rustdoc::bare_urls,
    rustdoc::broken_intra_doc_links,
    clippy::derive_partial_eq_without_eq
)]
#![forbid(unsafe_code)]
#![warn(trivial_casts, trivial_numeric_casts, unused_import_braces)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod traits;

pub use prost;
pub use tendermint_proto as tendermint;
pub use tendermint_proto::google::protobuf::{Any, Timestamp};
pub use cosmos_sdk_proto::cosmos;

/// The version (commit hash) of the Cosmos SDK used when generating this library.
pub const VERSION: &str = include_str!("prost/side/GIT_COMMIT");

pub mod side {
    pub mod btcbridge {
        include!("prost/side/side.btcbridge.rs");
    }
    pub mod liquidation {
        include!("prost/side/side.liquidation.rs");
    }
    pub mod dlc {
        include!("prost/side/side.dlc.rs");
    }
    pub mod lending {
        include!("prost/side/side.lending.rs");
    }
    pub mod oracle {
        include!("prost/side/side.oracle.rs");
    }
    pub mod tss {
        include!("prost/side/side.tss.rs");
    }
}
