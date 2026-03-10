//! The [TypeCmp] and [TypeEqual] trait. Comparaison between any types, not just numbers.

use typenum::{Equal, Ord};

/// Similar to [typenum::Cmp], but for any types, not just numbers. 
/// 
/// The output is a type that implements [Ord], and should be one of [typenum::Less], [typenum::Equal] or [typenum::Greater].
/// 
/// To implement this trait, use the [impl_Cmp] macro.
pub trait TypeCmp<Rhs> {
    /// The output type of the comparaison. 
    /// 
    /// It should be one of [typenum::Less], [typenum::Equal] or [typenum::Greater].
    type Output: Ord;
}

mod sub_traits;
pub use sub_traits::*;

impl<T> TypeCmp<T> for T {
    type Output = Equal;
}

/// Implements the [TypeCmp] trait for provided types.
/// The ordering is from left to right, ascending order.
/// 
/// If you provide only one list of types, the [TypeCmp] trait will be implemented between all these types.
/// 
/// If you provide two lists of types, separated by `=>`: the first contains existing types that already implement [TypeCmp] between them,
/// and the second contains new types for which you want to implement [TypeCmp].
/// 
/// If you provide two lists of types, separated by `|`: both lists contain types that already implement [TypeCmp] between them, 
/// and you want to implement [TypeCmp] between all the types of the first list and all the types of the second list.
/// 
/// Note: When providing a list of already [TypeCmp] types, the ordering is irrelevant.
/// 
/// ## Example:
/// ```
/// use extended_typenum::{impl_TypeCmp, TypeCmp};
/// use extended_typenum::{Less, Equal, Greater, True, False};
/// use extended_typenum::assert_type_eq;
/// 
/// // Let's implement comapraison between three types: A, B and C, with A < B < C.
/// 
/// struct A;
/// struct B;
/// struct C;
/// 
/// impl_TypeCmp!{ A, B, C }
/// 
/// // Now we can compare these types:
/// assert_type_eq!(<A as TypeCmp<B>>::Output, Less);
/// assert_type_eq!(<B as TypeCmp<A>>::Output, Greater);
/// assert_type_eq!(<A as TypeCmp<C>>::Output, Less);
/// 
/// // We can also use operator aliases:
/// use extended_typenum::TCmp;
/// use extended_typenum::TEq;
/// use extended_typenum::TLess;
/// use extended_typenum::TGreater;
/// 
/// assert_type_eq!(TCmp<C, A>, Greater);
/// assert_type_eq!(TEq<B, B>, True);
/// assert_type_eq!(TLess<A, C>, True);
/// assert_type_eq!(TGreater<C, B>, True);
/// 
/// // Now let's add more types to this ordering:
/// struct D;
/// struct E;
/// struct F;
/// 
/// impl_TypeCmp!{ A, B, C => D, E, F } // Now A, B, C are all less than D, E, or F, and D < E < F.
/// 
/// assert_type_eq!(TCmp<D, B>, Greater);
/// assert_type_eq!(TCmp<E, C>, Greater);
/// assert_type_eq!(TCmp<A, F>, Less);
/// 
/// assert_type_eq!(TCmp<D, E>, Less);
/// assert_type_eq!(TCmp<E, F>, Less);
/// 
/// // Now let's suppose we already have an ordering of even more types:
/// struct G;
/// struct H;
/// struct I;
/// impl_TypeCmp!{ G, H, I }
/// 
/// // We can merge two orderings with the `|` separator:
/// impl_TypeCmp!{ A, B, C | G, H, I }
/// 
/// assert_type_eq!(TCmp<G, A>, Greater);
/// assert_type_eq!(TCmp<H, B>, Greater);
/// assert_type_eq!(TCmp<I, C>, Greater);
/// 
/// // But wait, we forgot to merge D, E, F with G, H, I! No problem:
/// impl_TypeCmp!{ G, H, I | D, E, F }
/// 
/// // Now we have the following ordering: A < B < C < G < H < I < D < E < F.
/// 
/// assert_type_eq!(TCmp<G, A>, Greater);
/// assert_type_eq!(TCmp<H, B>, Greater);
/// assert_type_eq!(TCmp<I, D>, Less);
/// 
/// // Finally, let's try to define a bad ordering:
/// struct BadA;
/// struct BadB;
/// struct BadC;
/// 
/// impl_TypeCmp!{BadA, BadB}
/// impl_TypeCmp!{BadB, BadC}
/// impl_TypeCmp!{BadC, BadA}
/// 
/// // For now this works, but the order is incomplete, the types were not compared in all possible ways.
/// ```

#[macro_export]
macro_rules! impl_TypeCmp {
    // Single list => transform to two list call with empty existing list.
    { $($New:ty),* $(,)? } => {
        impl_TypeCmp!( => $($New),* );
    };

    // Base case: no new type to implement, just return.
    {$($Existing:ty),* $(,)? => } => {};

    // Recursive case: implement [TypeCmp] between all existing types and the new type, then call recursively with the new type added to the existing types.
    {$($Existing:ty),* $(,)? => $New:ty $(, $Rest:ty)* $(,)? } => {
        $(
            impl extended_typenum::TypeCmp<$New> for $Existing {
                type Output = extended_typenum::Less;
            }

            impl extended_typenum::TypeCmp<$Existing> for $New {
                type Output = extended_typenum::Greater;
            }

        )*

        impl_TypeCmp!{ $New, $($Existing),* => $($Rest),* }
    };

    {$($Existing:ty),* $(,)? | } => {};

    {$($Existing:ty),* $(,)? | $New:ty $(, $Rest:ty)* $(,)? } => {
        $(
            impl extended_typenum::TypeCmp<$New> for $Existing {
                type Output = extended_typenum::Less;
            }

            impl extended_typenum::TypeCmp<$Existing> for $New {
                type Output = extended_typenum::Greater;
            }

        )*

        impl_TypeCmp!{ $($Existing),* | $($Rest),* }
    };
}