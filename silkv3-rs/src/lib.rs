#[allow(dead_code)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(non_upper_case_globals)]
pub mod bindings;

pub fn silk_decoder(in_file: &str, out_file: &str) -> i32 {
    let in_file = std::ffi::CString::new(in_file).unwrap();
    let out_file = std::ffi::CString::new(out_file).unwrap();
    unsafe { silk_v3_decoder(in_file.as_ptr(), out_file.as_ptr()) }
}

extern "C" {
    fn silk_v3_decoder(in_file: *const i8, out_file: *const i8) -> i32;
}

#[cfg(test)]
mod test {
    use crate::bindings;
    #[test]
    fn test_skp_silk_sdk_get_version() {
        unsafe {
            let result = bindings::SKP_Silk_SDK_get_version();
            let c_str = std::ffi::CStr::from_ptr(result);
            let str_slice = c_str.to_str().unwrap();
            println!("Result: {}", str_slice);
        }
    }
}
