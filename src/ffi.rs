#[no_mangle]
pub extern "C" fn stardew_emulator_version() -> *const u8 {
    b"0.11.1\0".as_ptr()
}

#[no_mangle]
pub extern "C" fn stardew_emulator_abi() -> u32 {
    1
}
