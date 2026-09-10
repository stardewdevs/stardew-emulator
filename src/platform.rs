#[cfg(target_os = "android")]
pub fn is_android() -> bool {
    true
}

#[cfg(not(target_os = "android"))]
pub fn is_android() -> bool {
    false
}

#[cfg(target_os = "android")]
pub fn system_fonts_dir() -> &'static str {
    "/system/fonts"
}

#[cfg(not(target_os = "android"))]
pub fn system_fonts_dir() -> &'static str {
    "/usr/share/fonts"
}
