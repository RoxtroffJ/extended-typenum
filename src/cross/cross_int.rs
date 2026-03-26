use std::{
    fmt::Binary,
    ops::{Add, Deref, DerefMut, Div, Mul, Neg, Rem, Sub},
};
use typenum::{Abs, Cmp, Compare, Gcd, Integer, Max, Min, NInt, NonZero, PInt, Pow, ToInt, UInt, Unsigned};

use crate::{
    FromBit, FromInteger, FromRational, FromUnsigned, GetZero, IntoBit, IntoInteger, IntoRational,
    IntoUnsigned, IsZero, R,
};

/// Same as a [int](crate::int) but enables operations with right hand sides being other types ([uint](crate::uint), [rational](crate::rational), ...)
///
/// Note: although most traits are implemented automatically using [IntoInteger], some are not, and need to be implemented by hand.
/// Here is the list of these traits:
/// - [Div]
/// - [Mul]
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrossInt<I> {
    i: I,
}

macro_rules! impl_unary {
    ($Trait:ident => $($Output:ident)* {$($content:tt)*}) => {
        impl<I> $Trait for CrossInt<I>
        where I: $Trait
        {
            $(
                type $Output = CrossInt<<I as $Trait>::$Output>;
            )*
            $($content)*
        }
    };
}

macro_rules! impl_binary {
    ($Trait:ident => $($Output:ident)* {$($content:tt)*}) => {
        impl<I1, I2> $Trait<I2> for CrossInt<I1>
        where
            I2: $crate::IntoInteger,
            I1: $Trait<<I2 as IntoInteger>::Output>,
        {
            $(
                type $Output = CrossInt<<I1 as $Trait<<I2 as IntoInteger>::Output>>::$Output>;
            )*

            $($content)*
        }

    };
}

type Into<I> = <I as IntoInteger>::Output;

macro_rules! into {
    ($T:path) => {
        <$T as IntoInteger>::Output::default()
    };
}

impl_unary! {Abs => Output {}}
impl_binary! {Add => Output {
    fn add(self, _rhs: I2) -> Self::Output {
        CrossInt {i: self.i.add(into!(I2))}
    }
}}
impl_unary! {Binary => {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.i.fmt(f)
    }
}}

impl<I1, I2> Cmp<I2> for CrossInt<I1>
where
    I2: IntoInteger,
    I1: Cmp<Into<I2>>,
{
    type Output = Compare<I1, Into<I2>>;

    fn compare<IM: typenum::private::InternalMarker>(&self, _: &I2) -> Self::Output {
        self.i.compare::<IM>(&into!(I2))
    }
}

/// I/(N/D) = (I*D)/N
impl<I, N: Integer, D: Unsigned + NonZero> Div<R<N, D>> for CrossInt<I>
where
    D: IntoInteger,
    N: IntoInteger,
    I: Mul<Into<D>>,
    <I as Mul<Into<D>>>::Output: Div<Into<N>>,
{
    type Output = CrossInt<<<I as Mul<Into<D>>>::Output as Div<Into<N>>>::Output>;

    fn div(self, _rhs: R<N, D>) -> Self::Output {
        CrossInt {
            i: self.i.mul(into!(D)).div(into!(N)),
        }
    }
}

impl<I, U: Unsigned + NonZero> Div<PInt<U>> for CrossInt<I>
where
    I: Div<PInt<U>>,
{
    type Output = CrossInt<<I as Div<PInt<U>>>::Output>;

    fn div(self, rhs: PInt<U>) -> Self::Output {
        CrossInt { i: self.i.div(rhs) }
    }
}

impl<I, U: Unsigned + NonZero> Div<NInt<U>> for CrossInt<I>
where
    I: Div<NInt<U>>,
{
    type Output = CrossInt<<I as Div<NInt<U>>>::Output>;

    fn div(self, rhs: NInt<U>) -> Self::Output {
        CrossInt { i: self.i.div(rhs) }
    }
}

impl<I, U, B> Div<UInt<U, B>> for CrossInt<I>
where
    UInt<U, B>: IntoInteger,
    I: Div<Into<UInt<U, B>>>,
{
    type Output = CrossInt<<I as Div<Into<UInt<U, B>>>>::Output>;

    fn div(self, _rhs: UInt<U, B>) -> Self::Output {
        CrossInt {
            i: self.i.div(into!(UInt<U, B>)),
        }
    }
}

impl<B> FromBit for CrossInt<B>
where
    B: IntoBit,
{
    type Output = <B as IntoBit>::Output;

    fn from_bit(&self) -> Self::Output {
        self.i.into_bit()
    }
}

impl<I> FromInteger for CrossInt<I>
where
    I: IntoInteger,
{
    type Output = <I as IntoInteger>::Output;

    fn from_integer(&self) -> Self::Output {
        self.i.into_integer()
    }
}

impl<R> FromRational for CrossInt<R>
where
    R: IntoRational,
{
    type Output = <R as IntoRational>::Output;

    fn from_rational(&self) -> Self::Output {
        self.i.into_rational()
    }
}

impl<U> FromUnsigned for CrossInt<U>
where
    U: IntoUnsigned,
{
    type Output = <U as IntoUnsigned>::Output;

    fn from_unsigned(&self) -> Self::Output {
        self.i.into_unsigned()
    }
}

impl_binary! {Gcd => Output {}}
impl_unary! {GetZero => Output {}}

impl<I> IsZero for CrossInt<I>
where
    I: IsZero,
{
    type Output = <I as IsZero>::Output;
}

impl_binary! {Max => Output {
    fn max(self, _rhs: I2) -> Self::Output {
        CrossInt {i: self.i.max(into!(I2))}
    }
}}
impl_binary! {Min => Output {
    fn min(self, _rhs: I2) -> Self::Output {
        CrossInt {i: self.i.min(into!(I2))}
    }
}}

// Mul implementations mirror Div but perform multiplication semantics.
// I*(N/D) = (I*N)/D
impl<I, N: Integer, D: Unsigned + NonZero> Mul<R<N, D>> for CrossInt<I>
where
    N: IntoInteger,
    D: IntoInteger,
    I: Mul<Into<N>>,
    <I as Mul<Into<N>>>::Output: Div<Into<D>>,
{
    type Output = CrossInt<<<I as Mul<Into<N>>>::Output as Div<Into<D>>>::Output>;

    fn mul(self, _rhs: R<N, D>) -> Self::Output {
        CrossInt {
            i: self.i.mul(into!(N)).div(into!(D)),
        }
    }
}

impl<I, U: Unsigned + NonZero> Mul<PInt<U>> for CrossInt<I>
where
    I: Mul<PInt<U>>,
{
    type Output = CrossInt<<I as Mul<PInt<U>>>::Output>;

    fn mul(self, rhs: PInt<U>) -> Self::Output {
        CrossInt { i: self.i.mul(rhs) }
    }
}

impl<I, U: Unsigned + NonZero> Mul<NInt<U>> for CrossInt<I>
where
    I: Mul<NInt<U>>,
{
    type Output = CrossInt<<I as Mul<NInt<U>>>::Output>;

    fn mul(self, rhs: NInt<U>) -> Self::Output {
        CrossInt { i: self.i.mul(rhs) }
    }
}

impl<I, U, B> Mul<UInt<U, B>> for CrossInt<I>
where
    UInt<U, B>: IntoInteger,
    I: Mul<Into<UInt<U, B>>>,
{
    type Output = CrossInt<<I as Mul<Into<UInt<U, B>>>>::Output>;

    fn mul(self, _rhs: UInt<U, B>) -> Self::Output {
        CrossInt {
            i: self.i.mul(into!(UInt<U, B>)),
        }
    }
}

impl_unary! {Neg => Output {
    fn neg(self) -> Self::Output {
        CrossInt{i: self.i.neg()}
    }
}}
impl_binary!{Pow => Output {
    fn powi(self, _exp: I2) -> Self::Output {
        CrossInt{i: self.i.powi(into!(I2))}
    }
}}
impl_binary!{Rem => Output {
    fn rem(self, _rhs: I2) -> Self::Output {
        CrossInt{i: self.i.rem(into!(I2))}
    }
}}
impl_binary!{Sub => Output {
    fn sub(self, _rhs: I2) -> Self::Output {
        CrossInt {i: self.i.sub(into!(I2))}
    }
}}

impl<T, I> ToInt<T> for CrossInt<I>
where I: ToInt<T>
{
    fn to_int() -> T {
        I::to_int()
    }

    const INT: T = I::INT;
}

impl<I> From<I> for CrossInt<I> {
    fn from(value: I) -> Self {
        CrossInt { i: value }
    }
}

impl<I> AsRef<I> for CrossInt<I> {
    fn as_ref(&self) -> &I {
        &self
    }
}

impl<I> AsMut<I> for CrossInt<I> {
    fn as_mut(&mut self) -> &mut I {
        self.deref_mut()
    }
}

impl<I> Deref for CrossInt<I> {
    type Target = I;

    fn deref(&self) -> &Self::Target {
        &self.i
    }
}

impl<I> DerefMut for CrossInt<I> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.i
    }
}
