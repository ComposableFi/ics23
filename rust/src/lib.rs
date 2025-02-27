#![cfg_attr(not(feature = "std"), no_std)]
#![allow(clippy::large_enum_variant)]

extern crate alloc;
extern crate core;

mod api;
mod compress;
mod helpers;
mod host_functions;
mod ics23;
mod ops;
mod verify;

pub use crate::ics23::*;
pub use api::{
    iavl_spec, smt_spec, tendermint_spec, verify_batch_membership, verify_batch_non_membership,
    verify_membership, verify_non_membership,
};
pub use compress::{compress, decompress, is_compressed};
pub use helpers::{Hash, Result};
pub use host_functions::HostFunctionsProvider;
pub use verify::calculate_existence_root;

#[cfg(feature = "std")]
pub use host_functions::host_functions_impl::HostFunctionsManager;
