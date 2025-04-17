use super::*;

/// If `A` implements `FromUnsigned`, then `A::Output` is the [Unsigned] equivalent to `A`.
pub trait FromUnsigned {
    /// The Unsigned which can be converted into the Self.
    type Output: Unsigned;

    /// Computes the [Unsigned] which can be converted into `Self`.
    fn from_unsigned(&self) -> Self::Output;
}

/// Converts the implementor's type into a [Unsigned].
pub trait IntoUnsigned {
    /// The output type of the conversion.
    type Output: Unsigned;

    /// Converts into an [Unsigned].
    fn into_unsigned(&self) -> Self::Output;
}

impl<T: FromUnsigned> IntoUnsigned for T {
    type Output = T::Output;

    fn into_unsigned(&self) -> Self::Output {
        self.from_unsigned()
    }
}

// Conversions with bits:
impl FromBit for U0 {
    type Output = B0;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromBit for U1 {
    type Output = B1;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with unsigned integers:
impl FromUnsigned for U0 {
    type Output = U0;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned, B: Bit> FromUnsigned for UInt<U, B> {
    type Output = UInt<U, B>;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with integers:
impl FromInteger for U0 {
    type Output = Z0;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned, B: Bit> FromInteger for UInt<U, B> {
    type Output = PInt<UInt<U, B>>;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with rationals:
impl FromRational for U0 {
    type Output = R<Z0, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned, B: Bit> FromRational for UInt<U, B> {
    type Output = R<PInt<UInt<U, B>>, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}