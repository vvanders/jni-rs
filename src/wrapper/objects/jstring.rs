use objects::JObject;

use sys::{jobject, jstring};

/// Lifetime'd representation of a `jstring`. Just a `JObject` wrapped in a new
/// class.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JString<'a>(JObject<'a>);

impl<'a> From<jstring> for JString<'a> {
    fn from(other: jstring) -> Self {
        JString(From::from(other as jobject))
    }
}

impl<'a> ::std::ops::Deref for JString<'a> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> AsRef<jobject> for JString<'a> {
    fn as_ref(&self) -> &jobject {
        self.0.as_ref()
    }
}

impl<'a> From<JString<'a>> for JObject<'a> {
    fn from(other: JString) -> JObject {
        other.0
    }
}

impl<'a> From<JObject<'a>> for JString<'a> {
    fn from(other: JObject) -> JString {
        (other.into_inner() as jstring).into()
    }
}
