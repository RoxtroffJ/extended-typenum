//! Even more type operator traits.

use typenum::{False, True};

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