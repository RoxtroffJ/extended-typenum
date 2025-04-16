use super::*;

/// If `A` implements `FromInteger`, then `A::Output` is the [Integer] equivalent to `A`.
pub trait FromInteger {
    /// The Integer which can be converted into the Self.
    type Output: Integer;
}

/// Converts the implementor's type into a [Integer].
pub trait IntoInteger {
    /// The output type of the conversion.
    type Output: Integer;
}

impl<T: FromInteger> IntoInteger for T {
    type Output = T::Output;
}

// Conversions with bits:
impl FromBit for Z0 {
    type Output = B0;
}
impl FromBit for P1 {
    type Output = B1;
}

// Conversions with unsigned integers:
impl FromUnsigned for Z0 {
    type Output = U0;
}
impl<U: Unsigned, B: Bit> FromUnsigned for PInt<UInt<U, B>> {
    type Output = UInt<U, B>;
}

// Conversions with integers:
impl FromInteger for Z0 {
    type Output = Z0;
}
impl<U: Unsigned + NonZero> FromInteger for PInt<U> {
    type Output = PInt<U>;
}
impl<U: Unsigned + NonZero> FromInteger for NInt<U> {
    type Output = NInt<U>;
}

// Conversions with rationals:
impl FromRational for Z0 {
    type Output = R<Z0, U1>;
}
impl<U: Unsigned + NonZero> FromRational for PInt<U> {
    type Output = R<PInt<U>, U1>;
}
impl<U: Unsigned + NonZero> FromRational for NInt<U> {
    type Output = R<NInt<U>, U1>;
}