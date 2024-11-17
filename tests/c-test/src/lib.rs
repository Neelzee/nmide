use libloading::{Library, Symbol};
use nmide_plugin_manager::Nmlugin;
use nmide_rust_ffi::{CKeyPair, CMap};
use once_cell::sync::Lazy;
use std::{
    ffi::{c_char, CStr},
    path::PathBuf,
};
use tokio::sync::Mutex;

static PLUGIN: Lazy<Mutex<Nmlugin>> = Lazy::new(|| {
    Mutex::new(
        Nmlugin::new(
            PathBuf::new()
                .join("../../nmide-plugin/nmide-plugin-c/release/libnmide-plugin-c.so")
                .canonicalize()
                .expect("couldnt canonicalize"),
        )
        .expect("Couldnt make plugin"),
    )
});

#[tokio::test]
async fn name() {
    extern "C" {
        fn greetings() -> *const c_char;
    }
    let c_chars = unsafe { greetings() };
    let c_str = unsafe { CStr::from_ptr(c_chars) };
    println!("{}", c_str.to_string_lossy().to_string());
    let lib = unsafe {
        Library::new(
            PathBuf::new().join("../../nmide-plugin/nmide-plugin-c/release/libnmide-plugin-c.so"),
        )
    }
    .unwrap();
    let func: Symbol<unsafe extern "C" fn() -> CMap> = unsafe { lib.get(b"cinit") }.unwrap();
    let res = unsafe { func() };
    //let vec = unsafe { Vec::from_raw_parts(res.values, res.len, std::mem::size_of::<CKeyPair>()) };
}
