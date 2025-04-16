//! Module with traits that allows for converting between different types.
//! 
//! The conversions happen between categories ([Unsigned], [Integer], ...) of numeric types.
//! 
//! Like rust's [From] and [Into] traits, it is only required to implement the `From...` traits.
//! 
//! Examples:
//! ```
//! use crate::extended_typenum::*;
//! 
//! assert_type_eq!(<U42 as IntoInteger>::Output, P42);
//! assert_type_eq!(<B1 as IntoRational>::Output, rational!(P1, U1));
//! 
//! // assert_type_eq!(<N3 as IntoUnsigned>::Output, U3); This fails, N3 is signed.
//! ```

use super::*;

mod conversion_bit;
mod conversion_unsigned;
mod conversion_integer;
mod conversion_rational;

pub use conversion_bit::*;
pub use conversion_unsigned::*;
pub use conversion_integer::*;
pub use conversion_rational::*;