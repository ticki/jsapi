//! Logging to the JavaScript consoles and other integrated targets.

/// Issue an error log message.
pub fn error(msg: &JsString) {
    unsafe {
        asm!("console.error(__ObjectPool[$0])"
             :: "r"(msg.get_inner_object().get_id()));
    }
}

/// Issue an log message of unspecified kind.
pub fn log(msg: &JsString) {
    unsafe {
        asm!("console.log(__ObjectPool[$0])"
             :: "r"(msg.get_inner_object().get_id()));
    }
}

/// Issue an warning.
pub fn warn(msg: &JsString) {
    unsafe {
        asm!("console.warn(__ObjectPool[$0])"
             :: "r"(msg.get_inner_object().get_id()));
    }
}

/// Issue an info log message.
pub fn info(msg: &JsString) {
    unsafe {
        asm!("console.info(__ObjectPool[$0])"
             :: "r"(msg.get_inner_object().get_id()));
    }
}

/// Make a popup box with some message.
///
/// This is not recommended for anything other than debugging.
pub fn alert(msg: &JsString) {
    unsafe {
        asm!("alert(__ObjectPool[$0])"
             :: "r"(msg.get_inner_object().get_id()));
    }
}
