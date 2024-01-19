//! Factorio lib
//!
//! This library provides an interface to Factorio data types, mod prototypes and Web API.
//! Check the individual module documentation for details
//!
//! Modules are gated behind features with corresponding names, which are all enabled by default.

// TODO: transcode lua data to json

#[cfg(feature = "data_structs")]
pub use factorio_data_structs;
#[cfg(feature = "prototypes")]
pub use factorio_prototypes;
#[cfg(feature = "webapi")]
pub use factorio_web_util;
