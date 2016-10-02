//! Native JavaScript strings.

use std::{ops, iter, cmp};

use prelude::*;

/// A native JavaScript string.
pub struct JsString {
    /// The internal object.
    obj: Object,
}

impl JsString {
    /// Allocate a new JavaScript string with some content.
    pub fn new(s: &str) -> JsString {
        // Allocate the object.
        let obj = Object::new();

        // Load the string into the object.
        unsafe {
            asm!("__ObjectPool[$0]=Pointer_stringify($1,$2)"
                 :: "r"(self.obj.get_id()), "r"(s.as_ptr()), "r"(len.len()));
        }

        JsString {
            obj: obj,
        }
    }

    /// Push some character to the string.
    pub fn push(&self, c: char) {
        unsafe {
            asm!("__ObjectPool[$0]+=String.fromCharCode($1)"
                 :: "r"(self.obj.get_id()), "r"(c as Int));
        }
    }

    /// Append another JavaScript string.
    pub fn append(&self, other: JsString) {
        unsafe {
            asm!("__ObjectPool[$0]+=__ObjectPool[$1]"
                 :: "r"(self.obj.get_id()), "r"(other.obj.get_id()));
        }
    }

    /// Get the length (codepoints) of this string.
    pub fn len(&self) -> Int {
        let ret;
        unsafe {
            asm!("$0=__ObjectPool[$1].length"
                 : "=r"(ret)
                 : "r"(self.obj.get_id()));
        }
        ret
    }

    /// Get the n'th character of the string.
    ///
    /// This cannot be implemented through the `Index` trait due to the individual characters not
    /// being addressable.
    pub fn nth(&self, n: Int) -> Option<char> {
        if n < self.len() {
            None
        } else {
            let code;

            unsafe {
                asm!("$0=__ObjectPool[$1].charCodeAt($2)"
                     : "=r"(code)
                     : "r"(self.obj.get_id()), "r"(n));
            }

            Some(code as char)
        }
    }

    /// Get an iterator of the characters of this string.
    pub fn chars(&self) -> impl Iterator<Item = char> {
        (0..).filter_map(|x| self.nth(x))
    }

    /// Get the inner object object of this string.
    pub fn get_inner_object(&self) -> &Object {
        &self.obj
    }
}

impl<'a> Into<String> for &'a JsString {
    fn into(self) -> String {
        // TODO: Optimize.

        let mut string = String::new();
        string.extend(self.chars());

        string
    }
}

impl Clone for JsString {
    fn clone(&self) -> JsString {
        // Allocate the object.
        let obj = Object::new();

        // Just assign the string from the old one.
        unsafe {
            asm!("__ObjectPool[$0]=__ObjectPool[$1]"
                 :: "r"(obj.get_id()),
                    "r"(self.obj.get_id()));
        }

        JsString {
            obj: obj,
        }
    }
}

impl cmp::PartialEq for JsString {
    fn eq(&self, other: &JsString) -> bool {
        let ret;
        asm!("$0=$1===$2"
             : "=r"(ret)
             : "r"(self.obj.get_id()), "r"(other.obj.get_id()));
        ret
    }
}

impl cmp::Eq for JsString {}
