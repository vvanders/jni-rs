use std::convert::From;

use errors::*;

use sys::{jobject, JNIEnv};
use wrapper::objects::JObject;
use wrapper;

/// A global JVM reference. These are "pinned" by the garbage collector and are
/// guaranteed to not get collected until released. Thus, this is allowed to
/// outlive the `JNIEnv` that it came from. Still can't cross thread boundaries
/// since it requires a pointer to the `JNIEnv` to do anything useful with it.
pub struct GlobalRef {
    obj: jobject,
    env: *mut JNIEnv,
}

impl GlobalRef {
    /// Create a new global reference object. This assumes that
    /// `CreateGlobalRef` has already been called.
    pub unsafe fn new(env: *mut JNIEnv, obj: jobject) -> Self {
        GlobalRef {
            obj: obj,
            env: env,
        }
    }

    /// Creates a new global ref from an object
    pub unsafe fn from(env: &wrapper::JNIEnv, object: &JObject) -> Result<GlobalRef> {
        let obj: JObject = jni_call!(env.inner(), NewGlobalRef, object.into_inner());

        Ok(GlobalRef::new(env.inner(), obj.into_inner()))
    }

    /// accessor for global ref
    pub fn inner(&mut self) -> JObject {
        JObject::from(self.obj)
    }

    fn drop_ref(&mut self) -> Result<()> {
        unsafe {
            jni_unchecked!(self.env, DeleteGlobalRef, self.obj);
            check_exception!(self.env);
        }
        Ok(())
    }
}

impl Drop for GlobalRef {
    fn drop(&mut self) {
        let res = self.drop_ref();
        match res {
            Ok(()) => {}
            Err(e) => debug!("error dropping global ref: {:#?}", e),
        }
    }
}
