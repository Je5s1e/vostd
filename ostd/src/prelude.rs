// SPDX-License-Identifier: MPL-2.0
//! The prelude.
/// A specialized [`Result`] type for this crate.
///
/// [`Result`]: core::result::Result
pub type Result<T> = core::result::Result<T, crate::error::Error>;

pub(crate) use alloc::{boxed::Box, sync::Arc, vec::Vec};

#[cfg(ktest)]
pub use ostd_macros::ktest;

pub use crate::mm::{Paddr, /*UntypedMem,*/ Vaddr};
// early_print, early_println: not yet defined in Verus mode
/*pub use crate::{
early_print as print, early_println as println,
panic::abort,
};*/
