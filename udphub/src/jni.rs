use jni::objects::{JClass, JString};
use jni::sys::jint;
use jni::sys::jlong;
use jni::sys::jstring;
use jni::JNIEnv;
use std::ptr::null_mut;

use crate::app::{App, GetStatsMode};
use crate::config::Config;

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

    match serde_json::from_slice::<Config>(input.as_bytes()) {
        Ok(_) => null_mut(),
        Err(e) => {
            let output = env
                .new_string(format!("{}", e))
                .expect("Couldn't create java string!");
            output.into_raw()
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_create(
    _env: JNIEnv,
    _class: JClass,
) -> jlong {
    Box::into_raw(Box::new(App::new())) as usize as jlong
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_start(
    env: JNIEnv,
    _class: JClass,
    instance: jlong,
    config: JString,
) {
    let config: String = env
        .get_string(config)
        .expect("Couldn't get java string!")
        .into();

    let mut app = unsafe { Box::from_raw(instance as usize as *mut App) };
    app.start(&config);
    let _ = Box::into_raw(app);

}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_getError(
    env: JNIEnv,
    _class: JClass,
    instance: jlong,
) -> jstring {

    let app = unsafe { Box::from_raw(instance as usize as *mut App) };
    let ret = match &app.error {
        Some(x) => {
            let output = env
                .new_string(x)
                .expect("Couldn't create java string!");

            output.into_raw()
        }
        None => null_mut()
    };
    let _ = Box::into_raw(app);

    ret
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_delete(
    _env: JNIEnv,
    _class: JClass,
    instance: jlong,
) {
    let app = unsafe { Box::from_raw(instance as usize as *mut App) };
    drop(app);
}

#[no_mangle]
pub extern "system" fn Java_org_vi_1server_androidudpbus_Native_getStats(
    env: JNIEnv,
    _class: JClass,
    instance: jlong,
    mode: jint,
) -> jstring {
    let mode = if mode == 1 {
        GetStatsMode::Long
    } else {
        GetStatsMode::Short
    };
    let app = unsafe { Box::from_raw(instance as usize as *mut App) };
    let ret = match app.get_stats(mode) {
        Some(x) => {
            let output = env
                .new_string(x)
                .expect("Couldn't create java string!");

            output.into_raw()
        }
        None => null_mut()
    };
    let _ = Box::into_raw(app);

    ret
}
