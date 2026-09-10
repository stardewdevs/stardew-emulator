use crate::StardewEmulator;
use jni::objects::{JByteArray, JClass};
use jni::sys::{jint, jlong, jstring};
use jni::JNIEnv;
use std::sync::{Arc, Mutex};

lazy_static::lazy_static! {
    static ref EMULATOR: Arc<Mutex<Option<StardewEmulator>>> = Arc::new(Mutex::new(None));
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_create(
    mut env: JNIEnv,
    _class: JClass,
    cols: jint,
    rows: jint,
) -> jlong {
    match StardewEmulator::new(cols as usize, rows as usize) {
        Ok(emulator) => {
            let mut guard = EMULATOR.lock().unwrap();
            *guard = Some(emulator);
            1
        }
        Err(e) => {
            let _ = env.throw_new("java/lang/RuntimeException", e.to_string());
            0
        }
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_write(
    env: JNIEnv,
    _class: JClass,
    _handle: jlong,
    data: JByteArray,
) {
    let bytes = match env.convert_byte_array(data) {
        Ok(b) => b,
        Err(_) => return,
    };

    let mut guard = EMULATOR.lock().unwrap();
    if let Some(emulator) = guard.as_mut() {
        let _ = emulator.write(&bytes);
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_update(
    _env: JNIEnv,
    _class: JClass,
    _handle: jlong,
) {
    let mut guard = EMULATOR.lock().unwrap();
    if let Some(emulator) = guard.as_mut() {
        let _ = emulator.update();
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_snapshot(
    mut env: JNIEnv,
    _class: JClass,
    _handle: jlong,
) -> jstring {
    let guard = EMULATOR.lock().unwrap();
    if let Some(emulator) = guard.as_ref() {
        match serde_json::to_string(emulator.snapshot()) {
            Ok(json) => match env.new_string(json) {
                Ok(s) => s.into_raw(),
                Err(_) => std::ptr::null_mut(),
            },
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_resize(
    _env: JNIEnv,
    _class: JClass,
    _handle: jlong,
    cols: jint,
    rows: jint,
) {
    let mut guard = EMULATOR.lock().unwrap();
    if let Some(emulator) = guard.as_mut() {
        let _ = emulator.resize(cols as usize, rows as usize);
    }
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_destroy(
    _env: JNIEnv,
    _class: JClass,
    _handle: jlong,
) {
    let mut guard = EMULATOR.lock().unwrap();
    *guard = None;
}

#[no_mangle]
pub extern "system" fn Java_io_stardew_emulator_EmulatorNative_title(
    mut env: JNIEnv,
    _class: JClass,
    _handle: jlong,
) -> jstring {
    let guard = EMULATOR.lock().unwrap();
    if let Some(emulator) = guard.as_ref() {
        let engine = emulator.engine();
        let engine_guard = engine.lock().unwrap();
        let title = engine_guard.title().to_string();
        drop(engine_guard);

        match env.new_string(title) {
            Ok(s) => s.into_raw(),
            Err(_) => std::ptr::null_mut(),
        }
    } else {
        std::ptr::null_mut()
    }
}
