use super::*;

/// If `A` implements `FromInteger`, then `A::Output` is the [Integer] equivalent to `A`.
pub trait FromInteger {
    /// The [Integer] which can be converted into the Self.
    type Output: Integer;

    /// Computes the [Integer] which can be converted into `Self`.
    fn from_integer(&self) -> Self::Output;
}

/// Converts the implementor's type into a [Integer].
pub trait IntoInteger {
    /// The output type of the conversion.
    type Output: Integer;

    /// Converts into an [Integer].
    fn into_integer(&self) -> Self::Output;
}

impl<T: FromInteger> IntoInteger for T {
    type Output = T::Output;
    fn into_integer(&self) -> Self::Output {
        self.from_integer()
    }
}

// Conversions with bits:
impl FromBit for Z0 {
    type Output = B0;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromBit for P1 {
    type Output = B1;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with unsigned integers:
impl FromUnsigned for Z0 {
    type Output = U0;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned, B: Bit> FromUnsigned for PInt<UInt<U, B>> {
    type Output = UInt<U, B>;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with integers:
impl FromInteger for Z0 {
    type Output = Z0;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned + NonZero> FromInteger for PInt<U> {
    type Output = PInt<U>;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned + NonZero> FromInteger for NInt<U> {
    type Output = NInt<U>;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with rationals:
impl FromRational for Z0 {
    type Output = R<Z0, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned + NonZero> FromRational for PInt<U> {
    type Output = R<PInt<U>, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned + NonZero> FromRational for NInt<U> {
    type Output = R<NInt<U>, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}