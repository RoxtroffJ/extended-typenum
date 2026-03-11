//! Even more type operator traits.

use typenum::{False, Integer, NInt, NonZero, PInt, True, UInt, UTerm, Unsigned, B0, B1, Z0};

use crate::R;

/// If then else operator.
///
/// More convinient syntax with the associated [If](crate::operator_aliases_extended::If) type operator alias.
pub trait TypeIf<Then, Else> {
    /// The output type of the operator.
    type Output;
}

impl<Then, Else> TypeIf<Then, Else> for True {
    type Output = Then;
}

impl<Then, Else> TypeIf<Then, Else> for False {
    type Output = Else;
}

/// Indicates if the implementing type is zero.
pub trait IsZero {
    /// The output type of the operator.
    ///
    /// Should be either [True] or [False].
    type Output;
}

impl IsZero for B0 {
    type Output = True;
}

impl IsZero for B1 {
    type Output = False;
}

impl IsZero for UTerm {
    type Output = True;
}

impl<U, B> IsZero for UInt<U, B> {
    type Output = False;
}

impl IsZero for Z0 {
    type Output = True;
}

impl<U> IsZero for NInt<U>
where
    U: Unsigned + NonZero,
{
    type Output = False;
}

impl<U> IsZero for PInt<U>
where
    U: Unsigned + NonZero,
{
    type Output = False;
}

impl<N, D> IsZero for R<N, D>
where
    N: IsZero + Integer,
    D: Unsigned + NonZero,
{
    type Output = <N as IsZero>::Output;
}
