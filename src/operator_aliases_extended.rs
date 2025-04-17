//! Even more operator aliases. As of now, they do not work in the [op!] macro.

use super::*;

/// Converts a type into a bit.
pub type AsBit<A> = <A as IntoBit>::Output;

/// Converts a type into an unsigned integer.
pub type AsUnsigned<A> = <A as IntoUnsigned>::Output;

/// Converts a type into an integer.
pub type AsInteger<A> = <A as IntoInteger>::Output;

/// Converts a type into a rational number.
pub type AsRational<A> = <A as IntoRational>::Output;