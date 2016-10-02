use std::{ops, iter};

use ffi::object::Object;
use ffi::Int;

/// A native JavaScript string.
#[derive(Copy, Clone)]
pub struct JsString {
    /// The internal object.
    inner: Object,
}

impl JsString {
    /// Allocate a new JavaScript string with some content.
    pub fn new(s: &str) -> JsString {
        let obj = Object::new();

        // Load the string into the object.
        unsafe {
            asm!("jsObjects[$0]=Pointer_stringify($1,$2)"
                 :: "id"(self.inner.get_id()), "ptr"(s.as_ptr()), "len"(len.len()));
        }

        JsString {
            inner: Object,
        }
    }

    /// Push some character to the string.
    pub fn push(self, c: char) {
        unsafe {
            asm!("jsObjects[$0]+=String.fromCharCode($1)"
                 :: "r"(self.inner.get_id()), "r"(c as Int));
        }
    }

    /// Append another JavaScript string.
    pub fn append(self, other: JsString) {
        unsafe {
            asm!("jsObjects[$0]+=jsObjects[$1]"
                 :: "r"(self.inner.get_id()), "r"(other.inner.get_id()));
        }
    }

    /// Get the length (codepoints) of this string.
    pub fn len(self) -> Int {
        let ret;
        unsafe {
            asm!("$0=jsObjects[$1].length"
                 : "=r"(ret)
                 : "r"(self.inner.get_id()));
        }
        ret
    }

    /// Get the n'th character of the string.
    ///
    /// This cannot be implemented through the `Index` trait due to the individual characters not
    /// being addressable.
    pub fn nth(self, n: Int) -> Option<char> {
        if n < self.len() {
            None
        } else {
            let code;

            unsafe {
                asm!("$0=jsObjects[$1].charCodeAt($2)"
                     : "=r"(code)
                     : "r"(self.inner.get_id()), "r"(n));
            }

            Some(code as char)
        }
    }

    /// Get an iterator of the characters of this string.
    pub fn chars(self) -> impl Iterator<Item = char> {
        (0..).filter_map(|x| self.nth(x))
    }
}
