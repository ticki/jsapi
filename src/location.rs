//! Bindings to the location API.

use prelude::*;

/// Goto some specified destination.
pub fn goto(href: &JsString) {
    asm!("location.href=__ObjectPool[$0]"
         :: "r"(href.get_inner_object().get_id()));
}

/// Reload the page.
pub fn reload() {
    asm!("location.reload()");
}

/// Get the full hyperreference (URL) to the current page.
pub fn get_href() -> JsString {
    let ret = JsString::new("");

    unsafe {
        asm!("__ObjectPool[$0]=location.href"
             :: "r"(ret.get_inner_object().get_id()));
    }

    ret
}
