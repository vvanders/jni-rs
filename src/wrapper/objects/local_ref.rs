use JNIEnv;
use wrapper::objects::JObject;
use jni_sys::jobject;

pub struct LocalRef<'a,T>
        where T: AsRef<jobject> + Clone {
    inner: T,
    env: &'a JNIEnv<'a>
}

impl<'a,T> AsRef<T> for LocalRef<'a,T>
        where T: AsRef<jobject> + Clone {
    fn as_ref(&self) -> &T {
        &self.inner
    }
}

impl<'a,T> Drop for LocalRef<'a,T>
        where T: AsRef<jobject> + Clone {
    fn drop(&mut self) {
        self.env.delete_local_ref(JObject::from(self.inner.as_ref().clone())).unwrap_or(());
    }
}

impl<'a,T> LocalRef<'a,T>
        where T: AsRef<jobject> + Clone {
    pub fn from_env(env: &'a JNIEnv<'a>, other: T) -> LocalRef<'a,T> {
        LocalRef {
            inner: other,
            env: env
        }
    }
}