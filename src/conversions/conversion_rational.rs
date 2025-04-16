use super::*;

/// If `A` implements `FromRational`, then `A::Output` is the `Rational` equivalent to `A`.
pub trait FromRational {
    /// The Rational which can be converted into the Self.
    type Output: Rational;
}

/// Reciprocal of [FromRational].
/// 
/// Converts the implementor's type into a Rational.
pub trait IntoRational {
    /// The output type of the conversion.
    type Output: Rational;
}

impl<T: FromRational> IntoRational for T {
    type Output = T::Output;
}

// Conversions with bits:
impl FromBit for R<Z0, U1> {
    type Output = B0;
}
impl FromBit for R<P1, U1> {
    type Output = B1;
}
// Conversions with unsigned integers:
impl FromUnsigned for R<Z0, U1> {
    type Output = U0;
}
impl<U: Unsigned + NonZero> FromUnsigned for R<PInt<U>, U1> {
    type Output = U;
}

// Conversions with integers:
impl FromInteger for R<Z0, U1> {
    type Output = Z0;
}
impl<U: Unsigned + NonZero> FromInteger for R<PInt<U>, U1> {
    type Output = PInt<U>;
}
impl<U: Unsigned + NonZero> FromInteger for R<NInt<U>, U1> {
    type Output = NInt<U>;
}

// Conversions with rationals:
impl<N: Integer, D: Unsigned + NonZero> FromRational for R<N, D> {
    type Output = R<N, D>;
}