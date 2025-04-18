//! Even more operator aliases. As of now, they do not work in the [op!] macro.

use super::*;

/// Converts a type into a [Bit].
pub type AsBit<A> = <A as IntoBit>::Output;

/// Converts a type into an [Unsigned] integer.
pub type AsUnsigned<A> = <A as IntoUnsigned>::Output;

/// Converts a type into an [Integer].
pub type AsInteger<A> = <A as IntoInteger>::Output;

/// Converts a type into a [Rational] number.
pub type AsRational<A> = <A as IntoRational>::Output;

/// Converts an [Integer] and [Unsigned] into a rational number.
pub type ToRational<N, D> = <R<N, D> as Simplify>::Output;

/// Simplifies a [Rational] number.
pub type Simplified<R> = <R as Simplify>::Output;