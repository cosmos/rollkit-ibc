#![cfg_attr(not(test), deny(clippy::unwrap_used))]
#![deny(
    warnings,
    trivial_casts,
    trivial_numeric_casts,
    unused_import_braces,
    unused_qualifications,
    rust_2018_idioms
)]
#![forbid(unsafe_code)]

extern crate alloc;

pub mod context;
pub mod contract;
mod error;
pub mod handlers;
pub mod helpers;
pub mod msg;
pub mod response;
pub mod types;

pub use crate::error::ContractError;
pub use crate::helpers::*;
pub use crate::msg::*;
pub use crate::response::GenesisMetadata;
pub use crate::response::*;
