use typenum::{B0, B1, False, Integer, NInt, NonZero, PInt, True, UInt, UTerm, Unsigned, Z0};

use crate::{R, rational};

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

/// Returns the type of zero compatible with implementing type.
pub trait GetZero {
    /// The output type of the operator.
    type Output;
}

impl GetZero for B0 {
    type Output = B0;
}

impl GetZero for B1 {
    type Output = B0;
}

impl GetZero for UTerm {
    type Output = UTerm;
}

impl<U, B> GetZero for UInt<U, B> {
    type Output = UTerm;
}

impl GetZero for Z0 {
    type Output = Z0;
}

impl<U> GetZero for NInt<U>
where
    U: Unsigned + NonZero,
{
    type Output = Z0;
}

impl<U> GetZero for PInt<U>
where
    U: Unsigned + NonZero,
{
    type Output = Z0;
}

impl<N, D> GetZero for R<N, D>
where
    N: Integer,
    D: Unsigned + NonZero,
{
    type Output = rational!(Z0);
}