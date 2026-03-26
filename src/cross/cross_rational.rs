use std::{
    fmt::Binary,
    ops::{Add, Deref, DerefMut, Div, Mul, Neg, Rem, Sub},
};
use typenum::{Abs, Cmp, Compare, Gcd, Max, Min, Pow};

use crate::{
    FromBit, FromInteger, FromRational, FromUnsigned, GetZero, IntoBit, IntoInteger, IntoRational,
    IntoUnsigned, IsZero, Simplify,
};

/// Same as a [rational](mod@crate::rational) but enables operations with right hand sides being other types 
/// ([uint](crate::uint), [int](crate::int), ...)

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CrossRational<R> {
    r: R,
}

macro_rules! impl_unary {
    ($Trait:ident => $($Output:ident)* {$($content:tt)*}) => {
        impl<R> $Trait for CrossRational<R>
        where R: $Trait
        {
            $(
                type $Output = CrossRational<<R as $Trait>::$Output>;
            )*
            $($content)*
        }
    };
}

macro_rules! impl_binary {
    ($Trait:ident => $($Output:ident)* {$($content:tt)*}) => {
        impl<I1, I2> $Trait<I2> for CrossRational<I1>
        where
            I2: $crate::IntoInteger,
            I1: $Trait<<I2 as IntoInteger>::Output>,
        {
            $(
                type $Output = CrossRational<<I1 as $Trait<<I2 as IntoInteger>::Output>>::$Output>;
            )*

            $($content)*
        }

    };
}

type Into<R> = <R as IntoInteger>::Output;

macro_rules! into {
    ($T:path) => {
        <$T as IntoInteger>::Output::default()
    };
}

impl_unary! {Abs => Output {}}
impl_binary! {Add => Output {
    fn add(self, _rhs: I2) -> Self::Output {
        CrossRational {r: self.r.add(into!(I2))}
    }
}}
impl_unary! {Binary => {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.r.fmt(f)
    }
}}

impl<I1, I2> Cmp<I2> for CrossRational<I1>
where
    I2: IntoInteger,
    I1: Cmp<Into<I2>>,
{
    type Output = Compare<I1, Into<I2>>;

    fn compare<IM: typenum::private::InternalMarker>(&self, _: &I2) -> Self::Output {
        self.r.compare::<IM>(&into!(I2))
    }
}

impl_binary!{Div => Output {
    fn div(self, _rhs: I2) -> Self::Output {
        CrossRational{r: self.r.div(into!(I2))}
    }
}}

impl<B> FromBit for CrossRational<B>
where
    B: IntoBit,
{
    type Output = <B as IntoBit>::Output;

    fn from_bit(&self) -> Self::Output {
        self.r.into_bit()
    }
}

impl<R> FromInteger for CrossRational<R>
where
    R: IntoInteger,
{
    type Output = <R as IntoInteger>::Output;

    fn from_integer(&self) -> Self::Output {
        self.r.into_integer()
    }
}

impl<R> FromRational for CrossRational<R>
where
    R: IntoRational,
{
    type Output = <R as IntoRational>::Output;

    fn from_rational(&self) -> Self::Output {
        self.r.into_rational()
    }
}

impl<U> FromUnsigned for CrossRational<U>
where
    U: IntoUnsigned,
{
    type Output = <U as IntoUnsigned>::Output;

    fn from_unsigned(&self) -> Self::Output {
        self.r.into_unsigned()
    }
}

impl_binary! {Gcd => Output {}}
impl_unary! {GetZero => Output {}}

impl<R> IsZero for CrossRational<R>
where
    R: IsZero,
{
    type Output = <R as IsZero>::Output;
}

impl_binary! {Max => Output {
    fn max(self, _rhs: I2) -> Self::Output {
        CrossRational {r: self.r.max(into!(I2))}
    }
}}
impl_binary! {Min => Output {
    fn min(self, _rhs: I2) -> Self::Output {
        CrossRational {r: self.r.min(into!(I2))}
    }
}}

impl_binary!{Mul => Output {
    fn mul(self, _rhs: I2) -> Self::Output {
        CrossRational{r: self.r.mul(into!(I2))}
    }
}}

impl_unary! {Neg => Output {
    fn neg(self) -> Self::Output {
        CrossRational{r: self.r.neg()}
    }
}}
impl_binary!{Pow => Output {
    fn powi(self, _exp: I2) -> Self::Output {
        CrossRational{r: self.r.powi(into!(I2))}
    }
}}
impl_binary!{Rem => Output {
    fn rem(self, _rhs: I2) -> Self::Output {
        CrossRational{r: self.r.rem(into!(I2))}
    }
}}
impl_binary!{Sub => Output {
    fn sub(self, _rhs: I2) -> Self::Output {
        CrossRational {r: self.r.sub(into!(I2))}
    }
}}

impl_unary!{Simplify => Output {
    fn simplify(self) -> Self::Output {
        CrossRational{r: self.r.simplify()}
    }
}}

impl<R> From<R> for CrossRational<R> {
    fn from(value: R) -> Self {
        CrossRational { r: value }
    }
}

impl<R> AsRef<R> for CrossRational<R> {
    fn as_ref(&self) -> &R {
        &self
    }
}

impl<R> AsMut<R> for CrossRational<R> {
    fn as_mut(&mut self) -> &mut R {
        self.deref_mut()
    }
}

impl<R> Deref for CrossRational<R> {
    type Target = R;

    fn deref(&self) -> &Self::Target {
        &self.r
    }
}

impl<R> DerefMut for CrossRational<R> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.r
    }
}
