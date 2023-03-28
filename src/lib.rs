use jni::objects::*;
use jni::JNIEnv;
use jni::sys::jstring;

#[no_mangle]
pub  extern "system" fn Java_com_example_binance_RustNative_getStringFromRust(env: JNIEnv, _class: JClass)-> jstring {
    let output = env.new_string("hi bro from rust").unwrap();
    output.into_raw()
}

