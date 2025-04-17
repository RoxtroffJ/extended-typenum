use std::ops::Mul;

use super::*;

// Abs
impl<N: Integer, D: Unsigned + NonZero> Abs for R<N, D> where 
N: Abs,
op!(abs(N)): Integer
{
    type Output = R<op!(abs(N)), D>;
}

// Cmp
impl<Nl: Integer, Dl: Unsigned + NonZero, Nr: Integer, Dr: Unsigned + NonZero> Cmp<R<Nr, Dr>> for R<Nl, Dl> where
    Dl: IntoInteger,
    Dr: IntoInteger,
    Nl: Mul<AsInteger<Dr>>,
    Nr: Mul<AsInteger<Dl>>,
    Prod<Nl,AsInteger<Dr>>: Cmp<Prod<Nr, AsInteger<Dl>>>
{
    type Output = Compare<Prod<Nl,AsInteger<Dr>>, Prod<Nr, AsInteger<Dl>>>;
    
    fn compare<IM: private::InternalMarker>(&self, rhs: &R<Nr, Dr>) -> Self::Output {
        let lhs = self.num * rhs.den.into_integer();
        let rhs = rhs.num * self.den.into_integer();

        lhs.compare::<IM>(&rhs)
    }
}

// Max
#[doc(hidden)]
pub trait PrivateMax<Rhs, Cmp> {
    type Output;

    fn private_max(self, rhs: Rhs) -> Self::Output;
}
impl<T, Rhs> PrivateMax<Rhs, Less> for T {
    type Output = Rhs;

    fn private_max(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}
impl<T, Rhs> PrivateMax<Rhs, Greater> for T {
    type Output = T;

    fn private_max(self, _rhs: Rhs) -> Self::Output {
        self
    }
}
impl<T, Rhs> PrivateMax<Rhs, Equal> for T {
    type Output = T;

    fn private_max(self, _rhs: Rhs) -> Self::Output {
        self
    }
}

impl<Nl: Integer, Dl: Unsigned + NonZero, Nr: Integer, Dr: Unsigned + NonZero> Max<R<Nr, Dr>> for R<Nl, Dl> where
    R<Nl, Dl>: Cmp<R<Nr, Dr>>,
    R<Nl, Dl>: PrivateMax<R<Nr, Dr>, Compare<R<Nl, Dl>, R<Nr, Dr>>>
{
    type Output = <R<Nl, Dl> as PrivateMax<R<Nr, Dr>, Compare<R<Nl, Dl>, R<Nr, Dr>>>>::Output;
    
    fn max(self, rhs: R<Nr, Dr>) -> Self::Output {
        self.private_max(rhs)
    }
}

// Min
#[doc(hidden)]
pub trait PrivateMin<Rhs, Cmp> {
    type Output;

    fn private_min(self, rhs: Rhs) -> Self::Output;
}
impl<T, Rhs> PrivateMin<Rhs, Less> for T {
    type Output = T;

    fn private_min(self, _rhs: Rhs) -> Self::Output {
        self
    }
}
impl<T, Rhs> PrivateMin<Rhs, Greater> for T {
    type Output = Rhs;

    fn private_min(self, rhs: Rhs) -> Self::Output {
        rhs
    }
}
impl<T, Rhs> PrivateMin<Rhs, Equal> for T {
    type Output = T;

    fn private_min(self, _rhs: Rhs) -> Self::Output {
        self
    }
}

impl<Nl: Integer, Dl: Unsigned + NonZero, Nr: Integer, Dr: Unsigned + NonZero> Min<R<Nr, Dr>> for R<Nl, Dl> where
    R<Nl, Dl>: Cmp<R<Nr, Dr>>,
    R<Nl, Dl>: PrivateMin<R<Nr, Dr>, Compare<R<Nl, Dl>, R<Nr, Dr>>>
{
    type Output = <R<Nl, Dl> as PrivateMin<R<Nr, Dr>, Compare<R<Nl, Dl>, R<Nr, Dr>>>>::Output;
    
    fn min(self, rhs: R<Nr, Dr>) -> Self::Output {
        self.private_min(rhs)
    }
}

