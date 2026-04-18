use typenum::{Integer, NInt, NonZero, PInt, UInt, UTerm, Unsigned, B0, B1, Z0};

use crate::{CrossInt, CrossRational, R};

/// Same as [`Display`](std::fmt::Display), but formats a type and not a value.
pub trait TypeDisplay {
    /// Formats the type using the given formatter.
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result;
}

impl TypeDisplay for B0 {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B0")
    }
}

impl TypeDisplay for B1 {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "B1")
    }
}

impl TypeDisplay for UTerm {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0")
    }
}

impl<U, B> TypeDisplay for UInt<U, B>
where
    UInt<U, B>: Unsigned,
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", <UInt<U, B> as Unsigned>::USIZE)
    }
}

impl TypeDisplay for Z0 {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "0")
    }
}

impl<U: TypeDisplay + Unsigned + NonZero> TypeDisplay for PInt<U> {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        U::fmt(f)
    }
}

impl<U: TypeDisplay + Unsigned + NonZero> TypeDisplay for NInt<U> {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-")?;
        U::fmt(f)
    }
}

impl<N: TypeDisplay + Integer, D: TypeDisplay + Unsigned + NonZero> TypeDisplay for R<N, D> {
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        N::fmt(f)?;
        write!(f, "/")?;
        D::fmt(f)
    }
}

impl<I: TypeDisplay> TypeDisplay for CrossInt<I>
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        I::fmt(f)
    }
}

impl<R: TypeDisplay> TypeDisplay for CrossRational<R>
{
    fn fmt(f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        R::fmt(f)
    }
}