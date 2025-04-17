//! Type level rational numbers.
//! 
//! Rational numbers are represented by a pair of integers, the numerator and the denominator.
//! If there are two different representations of the same rational number, the compiler will not consider them equal.
//! Therefore, it is required to **always** [simplify](Simplify) the representations. This is why it is not recommended to
//! use the types defined here directly. Instead, always use the [rational!] macro.

use super::*;

/// A type level rational number.
#[derive(Clone, Copy, Hash, Debug, Default)]
pub struct R<N: Integer, D: Unsigned + NonZero> {
    pub(crate) num: N,
    pub(crate) den: D
}
impl<N: Integer, D: Unsigned + NonZero> R<N, D> {
    /// Creates a new rational number.
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
/// type R1 = rational!(P6, U8);
/// type R2 = rational!(U3, P4);
/// // type R3 = rational!(P6, U0); This will fail, we can't divide by zero.
/// 
/// assert_type_eq!(R1, R2);
/// 
/// type R4 = rational!(B1);
/// assert_type_eq!(R4, R<P1, U1>);
/// 
/// ```
#[macro_export]
macro_rules! rational{
    ($n: ty) => {
        <$n as IntoRational>::Output
    };

    ($n: ty, $d: ty) => {
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
