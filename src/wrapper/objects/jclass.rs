use objects::JObject;

use sys::{jobject, jclass};

/// Lifetime'd representation of a `jclass`. Just a `JObject` wrapped in a new
/// class.
#[repr(C)]
#[derive(Clone, Copy)]
pub struct JClass<'a>(JObject<'a>);

impl<'a> From<jclass> for JClass<'a> {
    fn from(other: jclass) -> Self {
        JClass(From::from(other as jobject))
    }
}

impl<'a> ::std::ops::Deref for JClass<'a> {
    type Target = JObject<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> AsRef<jobject> for JClass<'a> {
    fn as_ref(&self) -> &jobject {
        self.0.as_ref()
    }
}

impl<'a> From<JClass<'a>> for JObject<'a> {
    fn from(other: JClass) -> JObject {
        other.0
    }
}

impl<'a> From<JObject<'a>> for JClass<'a> {
    fn from(other: JObject) -> JClass {
        (other.into_inner() as jclass).into()
    }
}
