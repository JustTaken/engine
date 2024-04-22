extern "C" {
    pub fn dlopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> *const std::ffi::c_void;
    pub fn dlsym(handle: *const std::ffi::c_void, symbol: *const std::ffi::c_char) -> *const std::ffi::c_void;
}
