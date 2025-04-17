use super::*;

/// If `A` implements `FromRational`, then `A::Output` is the [Rational] equivalent to `A`.
pub trait FromRational {
    /// The [Rational] which can be converted into the Self.
    type Output: Rational;

    /// Computes the [Rational] which can be converted into `Self`.
    fn from_rational(&self) -> Self::Output;
}

/// Converts the implementor's type into a [Rational].
pub trait IntoRational {
    /// The output type of the conversion.
    type Output: Rational;

    /// Converts into a [Rational].
    fn into_rational(&self) -> Self::Output;
}

impl<T: FromRational> IntoRational for T {
    type Output = T::Output;

    fn into_rational(&self) -> Self::Output {
        self.from_rational()
    }
}

// Conversions with bits:
impl FromBit for R<Z0, U1> {
    type Output = B0;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromBit for R<P1, U1> {
    type Output = B1;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}
// Conversions with unsigned integers:
impl FromUnsigned for R<Z0, U1> {
    type Output = U0;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned, B: Bit> FromUnsigned for R<PInt<UInt<U, B>>, U1> {
    type Output = UInt<U, B>;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with integers:
impl FromInteger for R<Z0, U1> {
    type Output = Z0;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned + NonZero> FromInteger for R<PInt<U>, U1> {
    type Output = PInt<U>;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl<U: Unsigned + NonZero> FromInteger for R<NInt<U>, U1> {
    type Output = NInt<U>;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with rationals:
impl<N: Integer, D: Unsigned + NonZero> FromRational for R<N, D> {
    type Output = R<N, D>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}