use std::ops::*;
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

// Pow
impl<N: Integer, D: Unsigned + NonZero> Pow<R<N,D>> for f32 {
    type Output = f32;

    fn powi(self, exp: R<N,D>) -> Self::Output {
        self.powf(exp.to_f32())
    }
}
impl<N: Integer, D: Unsigned + NonZero> Pow<R<N,D>> for f64 {
    type Output = f64;

    fn powi(self, exp: R<N,D>) -> Self::Output {
        self.powf(exp.to_f64())
    }
}

impl<N: Integer, D: Unsigned + NonZero> Pow<rational!(Z0)> for R<N,D>
{
    type Output = rational!(P1);
    
    fn powi(self, _: rational!(Z0)) -> Self::Output {
        Self::Output::new()
    }
}
impl<N: Integer, D: Unsigned + NonZero, U: Unsigned + NonZero> Pow<R<PInt<U>, U1>> for R<N,D> where
N: Pow<PInt<U>>,
D: Pow<U>,
Exp<N, PInt<U>>: Integer,
Exp<D, U>: Unsigned + NonZero
{
    type Output = R<Exp<N, PInt<U>>, Exp<D, U>>;

    fn powi(self, _: R<PInt<U>, U1>) -> Self::Output {
        Self::Output::new()
    }
}
impl<N: Unsigned + NonZero, D: Unsigned + NonZero, U: Unsigned + NonZero> Pow<R<NInt<U>, U1>> for R<PInt<N>,D> where
R<PInt<D>, N>: Pow<R<PInt<U>, U1>>
{
    type Output = Exp<R<PInt<D>, N>, R<PInt<U>, U1>>;

    fn powi(self, _: R<NInt<U>, U1>) -> Self::Output {
        R::<PInt<D>, N>::new().powi(R::<PInt<U>, U1>::new())
    }
}
impl<N: Unsigned + NonZero, D: Unsigned + NonZero, U: Unsigned + NonZero> Pow<R<NInt<U>, U1>> for R<NInt<N>,D> where
R<NInt<D>, N>: Pow<R<PInt<U>, U1>>
{
    type Output = Exp<R<NInt<D>, N>, R<PInt<U>, U1>>;

    fn powi(self, _: R<NInt<U>, U1>) -> Self::Output {
        R::<NInt<D>, N>::new().powi(R::<PInt<U>, U1>::new())
    }
}

// Add
impl<Nl: Integer, Dl: Unsigned + NonZero, Nr: Integer, Dr: Unsigned + NonZero> Add<R<Nr, Dr>> for R<Nl, Dl> where
    Dl: IntoInteger,
    Dr: IntoInteger,
    Nl: Mul<AsInteger<Dr>>,
    Nr: Mul<AsInteger<Dl>>,
    Dl: Mul<Dr>,
    Prod<Nl, AsInteger<Dr>>: Add<Prod<Nr, AsInteger<Dl>>>,
    Sum<Prod<Nl, AsInteger<Dr>>, Prod<Nr, AsInteger<Dl>>>: Integer,
    Prod<Dl, Dr>: Unsigned + NonZero,
    R<Sum<Prod<Nl, AsInteger<Dr>>, Prod<Nr, AsInteger<Dl>>>, Prod<Dl, Dr>>: Simplify
{
    type Output = rational!(Sum<Prod<Nl, AsInteger<Dr>>, Prod<Nr, AsInteger<Dl>>>, Prod<Dl, Dr>);
    
    fn add(self, rhs: R<Nr, Dr>) -> Self::Output {
        let a = self.num * rhs.den.into_integer();
        let b = rhs.num * self.den.into_integer();

        let num = a + b;
        let den = self.den * rhs.den;

        R::from_parts(num, den).simplify()
    }
}

// Mul
impl<Nl: Integer, Dl: Unsigned + NonZero, Nr: Integer, Dr: Unsigned + NonZero> Mul<R<Nr, Dr>> for R<Nl, Dl> where
Nl: Mul<Nr>,
Dl: Mul<Dr>,
Prod<Nl, Nr>: Integer,
Prod<Dl, Dr>: Unsigned + NonZero,
R<Prod<Nl, Nr>, Prod<Dl, Dr>>: Simplify
{
    type Output = rational!(Prod<Nl, Nr>, Prod<Dl, Dr>);
    
    fn mul(self, rhs: R<Nr, Dr>) -> Self::Output {
        let num = self.num * rhs.num;
        let den = self.den * rhs.den;

        R::from_parts(num, den).simplify()
    }
}

// Div
impl<Nl: Integer, Dl: Unsigned + NonZero, Rhs> Div<Rhs> for R<Nl, Dl> where
Rhs: Pow<R<N1, U1>>,
R<Nl, Dl>: Mul<Exp<Rhs, rational!(N1)>>,
{
    type Output = Prod<R<Nl, Dl>, Exp<Rhs, rational!(N1)>>;
    
    fn div(self, rhs: Rhs) -> Self::Output {
        self * rhs.powi(R::<N1, U1>::new())
    }
}

// Neg
impl<N: Integer, D: Unsigned + NonZero> Neg for R<N, D> where
    N: Neg,
    Negate<N>: Integer
{
    type Output = R<Negate<N>, D>;

    fn neg(self) -> Self::Output {
        Self::Output::new()
    }
}

// Sub
impl<Nl: Integer, Dl: Unsigned + NonZero, Rhs: Neg> Sub<Rhs> for R<Nl, Dl> where
R<Nl, Dl>: Add<Negate<Rhs>>,
{
    type Output = <R<Nl, Dl> as Add<Negate<Rhs>>>::Output;
    
    fn sub(self, rhs: Rhs) -> Self::Output {
        self.add(rhs.neg())
    }
}