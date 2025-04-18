//! The [Simplify] trait is used to reduce a rational number to its simplest form,
//! by dividing the numerator and denominator by their greatest common divisor (GCD).

use std::ops::*;

use super::*;

/// Reduces a rational number to its simplest form by dividing the numerator and denominator by their GCD.
/// 
/// Example:
/// ```
/// use crate::extended_typenum::*;
/// use extended_typenum::rational::*;
/// 
/// assert_type_eq!(<R<P6, U8> as Simplify>::Output, R<P3, U4>);
/// assert_type_eq!(<R<N6, U8> as Simplify>::Output, R<N3, U4>);
/// assert_type_eq!(<R<Z0, U8> as Simplify>::Output, R<Z0, U1>);
/// ```
pub trait Simplify {
    /// Type of the simplified rational number.
    type Output;

    /// Returns the simplified rational number.
    fn simplify(self) -> Self::Output;
}

impl<N: Unsigned + NonZero, D: Unsigned + NonZero> Simplify for R<PInt<N>, D> where 
N: Gcd<D>,
N: Div<op!(gcd(N, D))>,
D: Div<op!(gcd(N, D))>,
op!(N / gcd(N, D)): Unsigned + NonZero,
op!(D / gcd(N, D)): Unsigned + NonZero
{
    type Output = R<PInt<op!(N / gcd(N, D))>, op!(D / gcd(N, D))>;

    fn simplify(self) -> Self::Output {
        Self::Output::new()
    }
}

impl<N: Unsigned + NonZero, D: Unsigned + NonZero> Simplify for R<NInt<N>, D> where 
N: Gcd<D>,
N: Div<op!(gcd(N, D))>,
D: Div<op!(gcd(N, D))>,
op!(N / gcd(N, D)): Unsigned + NonZero,
op!(D / gcd(N, D)): Unsigned + NonZero
{
    type Output = R<NInt<op!(N / gcd(N, D))>, op!(D / gcd(N, D))>;

    fn simplify(self) -> Self::Output {
        Self::Output::new()
    }
}

impl<D: Unsigned + NonZero> Simplify for R<Z0, D> {
    type Output = R<Z0, U1>;

    fn simplify(self) -> Self::Output {
        Self::Output::new()
    }
}