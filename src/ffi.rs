#[no_mangle]
pub extern "C" fn stardew_emulator_version() -> *const std::ffi::c_char {
    c"0.11.1".as_ptr()
}

#[no_mangle]
pub extern "C" fn stardew_emulator_abi() -> u32 {
    1
}
