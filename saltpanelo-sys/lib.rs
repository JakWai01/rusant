use std::{ffi::CString, ptr::null};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod saltpanelo {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

unsafe extern "C" fn open_url(url: *mut ::std::os::raw::c_char, rv: *mut *mut ::std::os::raw::c_char) {
    *rv = CString::new("Feli").unwrap().into_raw();
    println!("We did it! We did it!");
}

pub fn tti() {
    unsafe {
        let ptr = saltpanelo::SaltpaneloNewAdapter(
            None,
            None,
            None,
            Some(open_url),
            CString::new("ws://localhost:1338").unwrap().into_raw(),
            CString::new("127.0.0.1").unwrap().into_raw(),
            0,
            10000,
            CString::new("https://pojntfx.eu.auth0.com/").unwrap().into_raw(),
            CString::new("An94hvwzqxMmFcL8iEpTVrd88zFdhVdl").unwrap().into_raw(),
            CString::new("https://localhost:11337").unwrap().into_raw(),
        );

        println!("{:#?}", ptr);

        let res = saltpanelo::SaltpaneloAdapterLogin(ptr);

        let c_str = unsafe { std::ffi::CStr::from_ptr(res) };

        println!("{:?}", c_str.to_str().unwrap());
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
