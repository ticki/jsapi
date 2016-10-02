//! Bindings to JavaScript's `localStorage` API.

use prelude::*;

/// Get the value of some key in the local storage (`localStorage`).
///
/// It consumes the key and uses the allocation of the key to store the return value.
pub fn get(key: &JsString) -> JsString {
    let ret = JsString::new("");

    unsafe {
        asm!("__ObjectPool[$0]=localStorage[__ObjectPool[$1]]"
             :: "r"(ret.get_inner_object().get_id()),
                "r"(key.get_inner_object().get_id()))
    }

    ret
}

/// Set the value of some key in the local storage (`localStorage`).
///
/// Return the old value.
pub fn set(key: &JsString, val: &JsString) -> JsString {
    let ret = JsString::new("");

    unsafe {
        asm!("__ObjectPool[$0]=localStorage[__ObjectPool[$1]]=__ObjectPool[$2]"
             :: "r"(ret.get_inner_object().get_id()),
                "r"(key.get_inner_object().get_id()),
                "r"(val.get_inner_object().get_id()))
    }

    ret
}
