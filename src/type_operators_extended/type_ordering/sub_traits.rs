use typenum::{Equal, False, Greater, Less, True};

use crate::TypeCmp;

/// Similar to [typenum::IsEqual], but for any types, not just numbers.
/// 
/// To implement this trait, use the [impl_Cmp] macro.
pub trait TypeEqual<Rhs> {
    /// The output type of the equality test.
    type Output: typenum::Bit;
}

/// Similar to [typenum::IsGreater], but for any types, not just numbers.
/// 
/// To implement this trait, use the [impl_Cmp] macro.
pub trait TypeGreater<Rhs> {
    /// The output type of the greater test.
    type Output: typenum::Bit;
}

/// Similar to [typenum::IsLess], but for any types, not just numbers.
/// 
/// To implement this trait, use the [impl_Cmp] macro.
pub trait TypeLess<Rhs> {
    /// The output type of the less test.
    type Output: typenum::Bit;
}

/// Helper trait to tell if an [Ord] is equal. 
pub trait OrdIsEqual {
    /// The output type of the equality test.
    type Output: typenum::Bit;
}

impl OrdIsEqual for Less {
    type Output = False;
}

impl OrdIsEqual for Equal {
    type Output = True;
}

impl OrdIsEqual for Greater {
    type Output = False;
}

/// Helper trait to tell if an [Ord] is greater.
pub trait OrdIsGreater {
    /// The output type of the greater test.
    type Output: typenum::Bit;
}

impl OrdIsGreater for Less {
    type Output = False;
}

impl OrdIsGreater for Equal {
    type Output = False;
}

impl OrdIsGreater for Greater {
    type Output = True;
}

/// Helper trait to tell if an [Ord] is less.
pub trait OrdIsLess {
    /// The output type of the less test.
    type Output: typenum::Bit;
}

impl OrdIsLess for Less {
    type Output = True;
}

impl OrdIsLess for Equal {
    type Output = False;
}

impl OrdIsLess for Greater {
    type Output = False;
}

impl<L, R> TypeEqual<R> for L
where 
    L: TypeCmp<R>,
    <L as TypeCmp<R>>::Output: OrdIsEqual,
{
    type Output = <<L as TypeCmp<R>>::Output as OrdIsEqual>::Output;
}

impl<L, R> TypeGreater<R> for L
where 
    L: TypeCmp<R>,
    <L as TypeCmp<R>>::Output: OrdIsGreater,
{
    type Output = <<L as TypeCmp<R>>::Output as OrdIsGreater>::Output;
}

impl<L, R> TypeLess<R> for L
where 
    L: TypeCmp<R>,
    <L as TypeCmp<R>>::Output: OrdIsLess,
{
    type Output = <<L as TypeCmp<R>>::Output as OrdIsLess>::Output;
}
