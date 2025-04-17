use super::*;

/// If `A` implements `FromBit`, then `A::Output` is the [Bit] equivalent to `A`.
pub trait FromBit {
    /// The [Bit] which can be converted into the Self.
    type Output: Bit;

    /// Computes the [Bit] which can be converted into `Self`.
    fn from_bit(&self) -> Self::Output;
}

/// Converts the implementor's type into a [Bit].
pub trait IntoBit {
    /// The output type of the conversion.
    type Output: Bit;

    /// Converts into a [Bit].
    fn into_bit(&self) -> Self::Output;
}

impl<T: FromBit> IntoBit for T {
    type Output = T::Output;

    fn into_bit(&self) -> Self::Output {
        self.from_bit()
    }
}

// Conversions with bits:
impl FromBit for B0 {
    type Output = B0;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromBit for B1 {
    type Output = B1;

    fn from_bit(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with unsigned integers:
impl FromUnsigned for B0 {
    type Output = U0;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromUnsigned for B1 {
    type Output = U1;

    fn from_unsigned(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with integers:
impl FromInteger for B0 {
    type Output = Z0;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromInteger for B1 {
    type Output = P1;

    fn from_integer(&self) -> Self::Output {
        Self::Output::new()
    }
}

// Conversions with rationals:
impl FromRational for B0 {
    type Output = R<Z0, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}
impl FromRational for B1 {
    type Output = R<P1, U1>;

    fn from_rational(&self) -> Self::Output {
        Self::Output::new()
    }
}