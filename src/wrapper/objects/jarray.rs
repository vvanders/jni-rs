use sys::{ jarray, jint, jshort, jfloat, jdouble, jbyte, jchar, jboolean, jlong, jsize, JNI_ABORT, JNI_COMMIT };
use wrapper::signature::*;
use wrapper::objects::*;
use wrapper::jnienv::JNIEnv;
use errors::*;

pub struct JArray<'a> {
    internal: jarray,
    env: &'a JNIEnv<'a>
}

pub struct JArrayPrimitiveData<'a,T> {
    array: &'a JArray<'a>,
    ty: Primitive,
    data: *mut T,
    len: usize,
    dirty: bool
}

impl<'a,T> JArrayPrimitiveData<'a,T> {
    pub fn get(&self) -> &'a [T] {
        use std::slice;
        unsafe { slice::from_raw_parts(self.data, self.len) }
    }

    pub fn get_mut(&mut self) -> &'a mut [T] {
        use std::slice;
        self.dirty = true;
        unsafe { slice::from_raw_parts_mut(self.data, self.len) }
    }

    pub fn commit(&self) -> Result<()> {
        self.internal_release(true)
    }

    fn internal_release(&self, just_commit: bool) -> Result<()> {
        use std::mem;

        unsafe {
            let release_flag = if just_commit {
                JNI_COMMIT
            } else {
                if self.dirty {
                0 //Copy back the data since we may have touched it
                } else {
                    JNI_ABORT //Release the data without copying
                }
            };

            match self.ty {
                Primitive::Byte => jni_unchecked!(self.array.env.inner(), ReleaseByteArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Boolean => jni_unchecked!(self.array.env.inner(), ReleaseBooleanArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Char => jni_unchecked!(self.array.env.inner(), ReleaseCharArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Float => jni_unchecked!(self.array.env.inner(), ReleaseFloatArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Double => jni_unchecked!(self.array.env.inner(), ReleaseDoubleArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Int => jni_unchecked!(self.array.env.inner(), ReleaseIntArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Long => jni_unchecked!(self.array.env.inner(), ReleaseLongArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Short => jni_unchecked!(self.array.env.inner(), ReleaseShortArrayElements, self.array.internal, mem::transmute(self.data), release_flag),
                Primitive::Void => return Ok(())
            }
        }

        Ok(())
    }
}

impl<'a, T> Drop for JArrayPrimitiveData<'a,T> {
    fn drop(&mut self) {
        self.internal_release(false).unwrap_or(())
    }
}

impl<'a> JArray<'a> {
    pub fn from_env(env: &'a JNIEnv<'a>, array: JObject<'a>) -> Result<JArray<'a>> {
        Ok(JArray {
            internal: array.into_inner(),
            env: env
        })
    }

    pub fn into_inner(self) -> jarray {
        self.internal
    }

    pub fn new_char(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewCharArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_byte(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewByteArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_short(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewShortArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_int(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewIntArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_boolean(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewBooleanArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_float(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewFloatArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_double(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewDoubleArray, size);
        JArray::from_env(env, obj)
    }

    pub fn new_object(env: &'a JNIEnv<'a>, class: JClass<'a>, size: jsize, initial: JObject<'a>) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewObjectArray, size, class.into_inner(), initial.into_inner());
        JArray::from_env(env, obj)
    }
 
    pub fn new_long(env: &'a JNIEnv<'a>, size: jsize) -> Result<JArray<'a>> {
        let obj: JObject<'a> = jni_call!(env.inner(), NewLongArray, size);
        JArray::from_env(env, obj)
    }

    pub fn get_data_byte(&'a self) -> Result<JArrayPrimitiveData<'a, jbyte>> {
        self.primitive_data(Primitive::Byte)
    }

    pub fn get_data_char(&'a self) -> Result<JArrayPrimitiveData<'a, jshort>> {
        self.primitive_data(Primitive::Char)
    }

    pub fn get_data_float(&'a self) -> Result<JArrayPrimitiveData<'a, jfloat>> {
        self.primitive_data(Primitive::Float)
    }

    pub fn get_data_double(&'a self) -> Result<JArrayPrimitiveData<'a, jdouble>> {
        self.primitive_data(Primitive::Double)
    }

    pub fn get_data_boolean(&'a self) -> Result<JArrayPrimitiveData<'a, jboolean>> {
        self.primitive_data(Primitive::Boolean)
    }

    pub fn get_data_int(&'a self) -> Result<JArrayPrimitiveData<'a, jint>> {
        self.primitive_data(Primitive::Int)
    }

    pub fn get_data_long(&'a self) -> Result<JArrayPrimitiveData<'a, jlong>> {
        self.primitive_data(Primitive::Long)
    }

    pub fn get_data_short(&'a self) -> Result<JArrayPrimitiveData<'a, jshort>> {
        self.primitive_data(Primitive::Short)
    }

    pub fn get_object_element(&'a self, idx: usize) -> Result<JObject<'a>> {
        Ok(jni_call!(self.env.inner(), GetObjectArrayElement, self.internal, idx as jint))
    }

    pub fn set_object_element(&'a self, idx: usize, value: JObject<'a>) -> Result<()> {
        unsafe {
            jni_unchecked!(self.env.inner(), SetObjectArrayElement, self.internal, idx as jint, value.into_inner());
            check_exception!(self.env.inner());
        }

        Ok(())
    }

    fn primitive_data<T>(&'a self, ty: Primitive) -> Result<JArrayPrimitiveData<'a, T>> {
        use std::ptr;
        use std::mem;

        unsafe {
            let data: *mut T = match ty {
                Primitive::Byte => mem::transmute(jni_unchecked!(self.env.inner(), GetByteArrayElements, self.internal, ptr::null_mut())),
                Primitive::Boolean => mem::transmute(jni_unchecked!(self.env.inner(), GetBooleanArrayElements, self.internal, ptr::null_mut())),
                Primitive::Char => mem::transmute(jni_unchecked!(self.env.inner(), GetCharArrayElements, self.internal, ptr::null_mut())),
                Primitive::Float => mem::transmute(jni_unchecked!(self.env.inner(), GetFloatArrayElements, self.internal, ptr::null_mut())),
                Primitive::Double => mem::transmute(jni_unchecked!(self.env.inner(), GetDoubleArrayElements, self.internal, ptr::null_mut())),
                Primitive::Int => mem::transmute(jni_unchecked!(self.env.inner(), GetIntArrayElements, self.internal, ptr::null_mut())),
                Primitive::Long => mem::transmute(jni_unchecked!(self.env.inner(), GetLongArrayElements, self.internal, ptr::null_mut())),
                Primitive::Short => mem::transmute(jni_unchecked!(self.env.inner(), GetShortArrayElements, self.internal, ptr::null_mut())),
                Primitive::Void => return Err(ErrorKind::WrongJValueType("T", "Void").into())
            };

            let len = jni_unchecked!(self.env.inner(), GetArrayLength, self.internal);

            let res = JArrayPrimitiveData::<'a, T> {
                array: self,
                ty: ty,
                data: data,
                len: len as usize,
                dirty: false
            };

            Ok(res)
        }
    }
}