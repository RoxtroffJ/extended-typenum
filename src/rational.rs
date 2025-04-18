//! Type level rational numbers.
//! 
//! Rational numbers are represented by a pair of integers, the numerator and the denominator.
//! If there are two different representations of the same rational number, the compiler will not consider them equal.
//! Therefore, it is required to **always** [simplify](Simplify) the representations. This is why it is not recommended to
//! use the types defined here directly. Instead, always use the [rational!] macro.

use super::*;

/// A type level rational number.
#[derive(Clone, Copy, Hash, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
pub struct R<N: Integer, D: Unsigned + NonZero> {
    pub(crate) num: N,
    pub(crate) den: D
}
impl<N: Integer, D: Unsigned + NonZero> R<N, D> {
    /// Creates a new rational number.
    /// 
    /// **Warning:** Don't forget to simplify it!
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns the numerator of the rational number.
    pub fn numerator(&self) -> N {
        self.num
    }

    /// Returns the denominator of the rational number.
    pub fn denominator(&self) -> D {
        self.den
    }

    /// Builds a rational number from a numerator and denominator.
    /// 
    /// This number is the same as the one created by the [new] method, but rust can guess which type of rational you want to create with the arguments.
    /// 
    /// **Warning:** Don't forget to simplify the result!
    pub fn from_parts(num: N, den: D) -> Self {
        Self { num, den }
    }
}

mod simplify;
pub use simplify::*;


/// Macro to create a rational number type.
/// 
/// It takes one or two arguments.
/// If one is given, the argument is converted into a rational with the [IntoRational] trait.
/// If two are given, the first is the numerator and must be [IntoInteger] and the second is the denominator and must be [IntoUnsigned].
/// 
/// Example:
/// ```
/// use crate::extended_typenum::*;
/// use extended_typenum::rational::*;
/// 
/// type R1 = rational!(P6, U8); // Expands to <R<P6, U8> as Simplify>::Output.
/// type R2 = rational!(U3; P4); // If conversions are needed for the above to work, use a ; instead of a ,.
/// // type R3 = rational!(P6, U0); This will fail, we can't divide by zero.
/// 
/// assert_type_eq!(R1, R2);
/// 
/// type R4 = rational!(P1); // Expands to R<P1, U1>.
/// type R5 = rational!(B1;); // If conversions are needed, use a ;. Expands to <B1 as IntoRational>::Output.
/// 
/// assert_type_eq!(R4, R5);
/// assert_type_eq!(R4, R<P1, U1>);
/// 
/// ```
/// 
/// Note: Operator [Pow] is not implemented for rational exponents due to skill issue.
#[macro_export]
macro_rules! rational{
    ($n: ty) => {
        R<$n, U1>
    };
    ($n: ty;) => {
        <$n as IntoRational>::Output
    };
    ($n: ty, $d: ty) => {
        <R<$n, $d> as Simplify>::Output
    };
    ($n: ty; $d: ty) => {
        <R<<$n as IntoInteger>::Output, <$d as IntoUnsigned>::Output> as Simplify>::Output
    };
}

/// Marker trait for rational numbers.
pub trait Rational {
    /// Returns a `f32` representation of the rational number.
    fn to_f32(&self) -> f32;

    /// Returns a `f64` representation of the rational number.
    fn to_f64(&self) -> f64;
}

impl<N: Integer, D: Unsigned + NonZero> Rational for R<N,D> {
    fn to_f32(&self) -> f32 {
        (N::to_i16() as f32) / (D::to_u16() as f32)
    }

    fn to_f64(&self) -> f64 {
        (N::to_i32() as f64) / (D::to_u32() as f64)
    }
}

mod operations;
