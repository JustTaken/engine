extern "C" {
    pub fn dlopen(filename: *const std::ffi::c_char, flags: std::ffi::c_int) -> *const std::ffi::c_void;
    pub fn dlsym(handle: *const std::ffi::c_void, symbol: *const std::ffi::c_char) -> *const std::ffi::c_void;
}

pub fn load_library(library_name: &str) -> Result<*const std::ffi::c_void, ()>  {
    let vk_name = std::ffi::CString::new(library_name).unwrap();
    let library = unsafe { dlopen(vk_name.as_ptr(), 1) };

    if library.is_null() {
        Err(())
    } else {
        Ok(library)
    }
}

