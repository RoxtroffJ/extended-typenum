use super::*;

/// If `A` implements `FromBit`, then `A::Output` is the [Bit] equivalent to `A`.
pub trait FromBit {
    /// The bit which can be converted into the Self.
    type Output: Bit;
}

/// Converts the implementor's type into a [Bit].
pub trait IntoBit {
    /// The output type of the conversion.
    type Output: Bit;
}

impl<T: FromBit> IntoBit for T {
    type Output = T::Output;
}

// Conversions with bits:
impl FromBit for B0 {
    type Output = B0;
}
impl FromBit for B1 {
    type Output = B1;
}

// Conversions with unsigned integers:
impl FromUnsigned for B0 {
    type Output = U0;
}
impl FromUnsigned for B1 {
    type Output = U1;
}

// Conversions with integers:
impl FromInteger for B0 {
    type Output = Z0;
}
impl FromInteger for B1 {
    type Output = P1;
}

// Conversions with rationals:
impl FromRational for B0 {
    type Output = R<Z0, U1>;
}
impl FromRational for B1 {
    type Output = R<P1, U1>;
}