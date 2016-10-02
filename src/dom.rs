use prelude::*;

/// A DOM element.
pub struct Element {
    /// The inner object.
    obj: Object,
}

impl Element {
    /// Get a DOM element from its ID.
    ///
    /// The behavior is equivalent to JavaScript's `document.getElementFromId()`.
    ///
    /// In case no element with matching ID exists, `None` is returned.
    pub fn from_id(id: &JsString) -> Option<Element> {
        let obj = Object::new();

        unsafe {
            asm!("__ObjectPool[$0]=document.getElementFromId(__ObjectPool[$1])"
                 : "r"(obj.get_id()),
                   "r"(id.get_inner_object().get_id()))
        }

        Element {
            obj: obj,
        }
    }

    /// Set or unset the "hidden" property.
    ///
    /// `true` will make the element invisible and `false` vice versa.
    pub fn set_hidden(&self, opt: bool) {
        unsafe {
            if opt {
                asm!("__ObjectPool[$0].hidden=true"
                     :: "r"(self.obj.get_id()));
            } else {
                asm!("__ObjectPool[$0].hidden=false"
                     :: "r"(self.obj.get_id()));
            }
        }
    }

    /// Set a CSS style propety.
    pub fn set_style_property(&self, key: &JsString, val: &JsString) {
        unsafe {
            asm!("__ObjectPool[$0].style[__ObjectPool[$1]]=__ObjectPool[$2]"
                 :: "r"(self.obj.get_id()),
                    "r"(key.get_inner_object().get_id(),
                    "r"(val.get_inner_object().get_id())));
        }
    }

    /// Set the contained text of the element.
    pub fn set_text(&self, txt: &JsString) {
        unsafe {
            asm!("__ObjectPool[$0].textContent=__ObjectPool[$1]"
                 :: "r"(self.obj.get_id()),
                    "r"(txt.get_inner_object().get_id()));
        }
    }

    /// Set the inner HTML of the element.
    pub fn set_html(&self, html: &JsString) {
        unsafe {
            asm!("__ObjectPool[$0].innerHTML=__ObjectPool[$1]"
                 :: "r"(self.obj.get_id()),
                    "r"(html.get_inner_object().get_id()));
        }
    }
}
