use super::*;

/// If `A` implements `FromUnsigned`, then `A::Output` is the [Unsigned] equivalent to `A`.
pub trait FromUnsigned {
    /// The Unsigned which can be converted into the Self.
    type Output: Unsigned;
}

/// Converts the implementor's type into a [Unsigned].
pub trait IntoUnsigned {
    /// The output type of the conversion.
    type Output: Unsigned;
}

impl<T: FromUnsigned> IntoUnsigned for T {
    type Output = T::Output;
}

// Conversions with bits:
impl FromBit for U0 {
    type Output = B0;
}
impl FromBit for U1 {
    type Output = B1;
}

// Conversions with unsigned integers:
impl FromUnsigned for U0 {
    type Output = U0;
}
impl<U: Unsigned, B: Bit> FromUnsigned for UInt<U, B> {
    type Output = UInt<U, B>;
}

// Conversions with integers:
impl FromInteger for U0 {
    type Output = Z0;
}
impl<U: Unsigned, B: Bit> FromInteger for UInt<U, B> {
    type Output = PInt<UInt<U, B>>;
}

// Conversions with rationals:
impl FromRational for U0 {
    type Output = R<Z0, U1>;
}
impl<U: Unsigned, B: Bit> FromRational for UInt<U, B> {
    type Output = R<PInt<UInt<U, B>>, U1>;
}