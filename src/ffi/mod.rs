//! JavaScript FFI.

use prelude::*;

/// The integer type of JavaScript.
type Int = u32;

pub mod object;

/// Evaluate arbitrary JavaScript code.
///
/// # Safety
///
/// You can break invariants, and thus it's marked `unsafe`.
pub unsafe fn eval(js: &JsString) {
    asm!("eval(__ObjectPool[$0])"
         :: "r"(js.get_inner_object().get_id()));
}
