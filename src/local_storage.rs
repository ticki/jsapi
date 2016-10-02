//! Bindings to JavaScript's `localStorage` API.

/// Get the value of some key in the local storage (`localStorage`).
///
/// It consumes the key and uses the allocation of the key to store the return value.
fn get(key: JsString) -> JsString {
    unsafe {
        asm!("jsObjects[$0]=localStorage[jsObjects[$0]]"
             :: "r"(key.get_inner_object().get_id()))
    }

    key
}

/// Set the value of some key in the local storage (`localStorage`).
///
/// Return the old value.
fn set(key: JsString, val: JsString) -> JsString {
    unsafe {
        asm!("jsObjects[$0]=localStorage[jsObjects[$0]]=jsObjects[$1]"
             :: "r"(key.get_inner_object().get_id()),
                "r"(val.get_inner_object().get_id()))
    }

    key
}
