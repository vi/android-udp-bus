use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::sys::jlong;
use jni::sys::jstring;
use jni::JNIEnv;
use std::ptr::null_mut;

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_checkConfig(
    env: JNIEnv,
    _class: JClass,
    input: JString,
) -> jstring {
    let input: String = env
        .get_string(input)
        .expect("Couldn't get java string!")
        .into();

    if input.starts_with('[') {
        return null_mut();
    }
    let output = env
        .new_string(format!("Hello, {}!", input))
        .expect("Couldn't create java string!");

    output.into_raw()
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_create(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    1
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_configure(
    _env: JNIEnv,
    _class: JClass,
    instance: jlong,
    config: jstring,
) {
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_getError(
    _env: JNIEnv,
    _class: JClass,
    instance: jlong,
) -> jstring {
    null_mut()
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_start(
    _env: JNIEnv,
    _class: JClass,
    instance: jlong,
) {
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_delete(
    _env: JNIEnv,
    _class: JClass,
    instance: jlong,
) {
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_getStats(
    env: JNIEnv,
    _class: JClass,
    instance: jlong,
    mode: jint,
) -> jstring {
    let output = env
        .new_string(format!("Stats"))
        .expect("Couldn't create java string!");

    output.into_raw()
}
