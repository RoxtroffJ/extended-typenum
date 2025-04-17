//! This library reuploads and extends the typenum crate.
#![warn(missing_docs)]

pub use typenum::*;

pub mod rational;
pub use rational::*;

pub mod conversions;
pub use conversions::*;

pub mod operator_aliases_extended;
pub use operator_aliases_extended::*;