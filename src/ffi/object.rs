//! A simple allocator for typed JavaScript objects.

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

/// An opaque pointer into some JavaScript-typed object.
pub struct Object {
    /// The index of the `jsObjects` array this pointer refers to.
    id: usize,
}

impl Object {
    /// Allocate a new object.
    ///
    /// The initial value is `null`.
    fn new() -> Object {
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

    /// Get the identifier of this object.
    ///
    /// To access the object in JavaScript code, you do `jsObjects[id]`.
    fn get_id(&self) -> usize {
        self.id
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        asm!("delete jsObjects[$0];jsObjectsFree.push($0)" :: "r"(self.id));
    }
}
