//! A simple allocator for typed JavaScript objects.

use ffi::Int;

/// Initialize the typed arena if not already initialized.
fn arena_init() {
    unsafe {
        asm!("\
        if('undefined'===typeof jsObjects){\
             jsObjects=[];\
             jsObjectsFree=[]\
        }");
    }
}

/// Allocate an object into the `jsObjects` global array.
fn alloc() -> Int {
    let ret;

    arena_init();
    unsafe {
        asm!("\
        var $0=jsObjectsFree.pop()\
        if(!$0){\
            $0=jsObjects.len();\
            jsObjects.push(null);\
        }" : "=r"(ret));
    }

    ret
}

/// An opaque pointer into some JavaScript-typed object.
pub struct Object {
    /// The index of the `jsObjects` array this pointer refers to.
    id: Int,
}

impl Object {
    /// Allocate a new object.
    ///
    /// The initial value is `null`.
    fn new() -> Object {
        Object {
            id: alloc(),
        }
    }

    /// Get the identifier of this object.
    ///
    /// To access the object in JavaScript code, you do `jsObjects[id]`.
    fn get_id(&self) -> Int {
        self.id
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        asm!("delete jsObjects[$0];jsObjectsFree.push($0)" :: "r"(self.id));
    }
}
