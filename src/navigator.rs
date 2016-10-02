/// An user agent identifier.
struct UserAgent {
    /// The textual representation of the user agent.
    string: String,
}

impl UserAgent {
    /// Get the user agent of the browser.
    pub fn get() -> JsString {
        let string = JsString::new("");

        unsafe {
            asm!("__ObjectPool[$0]=navigator.userAgent"
                 :: "r"(string.get_inner_object().get_id()));
        }

        UserAgent {
            string: string.into(),
        }
    }

    /// Get the browser of this user agent.
    pub fn browser(&self) -> Browser {
        if self.string.contains("Chrome") {
            // Where's my customizability?
            Browser::Chrome
        } else if self.string.contains("Safari") {
            // Overpriced shit.
            Browser::Safari
        } else if self.string.contains("Opera") {
            // WTF? Do you actually use Opera?
            Browser::Opera
        } else if self.string.contains("Firefox") {
            // Where did the memory go?
            Browser::Firefox
        } else if self.string.contains("Microsoft Internet Explorer") {
            // You're screwed.
            Browser::Explorer
        } else {
            Browser::Other
        }
    }

    /// Get the inner string (raw representation of the user agent).
    pub fn into_string(self) -> String {
        self.string
    }
}

/// Browser identifier.
pub enum Browser {
    /// Mozilla Firefox.
    Firefox,
    /// Google Chrome.
    Chrome,
    /// Microsoft Internet Explorer.
    Explorer,
    /// Opera Web Browser.
    Opera,
    /// Apple Safari.
    Safari,
    /// Unknown browser.
    Other,
}

/// The user's specified language.
///
/// # Example
///
/// `en-US`
pub fn language() -> JsString {
    let ret = JsString::new("");

    unsafe {
        asm!("__ObjectPool[$0]=navigator.language"
             :: "r"(ret.get_inner_object().get_id()));
    }

    ret
}

/// Is cookies enabled?
pub fn cookies_enabled() -> bool {
    let ret;

    unsafe {
        asm!("$0=navigator.cookiesEnabled"
             :: "r"(ret));
    }

    ret
}
