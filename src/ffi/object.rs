//! A simple allocator for typed JavaScript objects.

use prelude::*;

/// Initialize the typed arena if not already initialized.
fn arena_init() {
    unsafe {
        asm!("\
        if('undefined'===typeof __ObjectPool){\
             __ObjectPool=[];\
             __ObjectPoolFree=[]\
        }");
    }
}

/// Allocate an object into the `__ObjectPool` global array.
fn alloc() -> Int {
    let ret;

    arena_init();
    unsafe {
        asm!("\
        var $0=__ObjectPoolFree.pop()\
        if(!$0){\
            $0=__ObjectPool.len();\
            __ObjectPool.push(null);\
        }" : "=r"(ret));
    }

    ret
}

/// An opaque pointer into some JavaScript-typed object.
pub struct Object {
    /// The index of the `ObjectPool` array this pointer refers to.
    id: Int,
}

impl Object {
    /// Allocate a new object.
    ///
    /// The initial value is `null`.
    pub fn new() -> Object {
        Object {
            id: alloc(),
        }
    }

    /// Get the identifier of this object.
    ///
    /// To access the object in JavaScript code, you do `ObjectPool[id]`.
    pub fn get_id(&self) -> Int {
        self.id
    }

    /// Is this object null?
    pub fn is_null(&self) -> bool {
        let ret;

        unsafe {
            asm!("$0=__ObjectPool[$1]===null"
                 : "=r"(ret)
                 : "r"(self.id));
        }

        ret
    }
}

impl Drop for Object {
    fn drop(&mut self) {
        unsafe {
            asm!("delete __ObjectPool[$0];\
                  __ObjectPoolFree.push($0)" :: "r"(self.id));
        }
    }
}
