#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use std::env;

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

// Need to set DYLD_LIBRARY_PATH for runtime linking on MacOS
// DYLD_LIBRARY_PATH=./vendor cargo test
#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CStr;

    #[test]
    fn test_config() {
        unsafe {
            config.setMap();
            let arr = config.getMap();
            let foo = CStr::from_ptr(arr.as_ptr() as *const i8);
            assert_eq!(foo.to_str().unwrap(), "bar",);
        };
    }
}
