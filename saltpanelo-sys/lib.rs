use std::{ffi::CString, ptr::null};

use saltpanelo::SaltpaneloOnRequestCallResponse;

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
mod saltpanelo {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

unsafe extern "C" fn open_url(
    url: *mut ::std::os::raw::c_char,
    rv: *mut *mut ::std::os::raw::c_char,
) {
    *rv = CString::new("Feli").unwrap().into_raw();
    println!("We did it! We did it!");
}

unsafe extern "C" fn on_request_call(
    src_id: *mut ::std::os::raw::c_char,
    src_email: *mut ::std::os::raw::c_char,
    route_id: *mut ::std::os::raw::c_char,
    channel_id: *mut ::std::os::raw::c_char,
    rv: *mut SaltpaneloOnRequestCallResponse,
) {
    // This cannot be tested locally since we need a second party
    println!("Requested call");
}

unsafe extern "C" fn on_call_disconnected(
    route_id: *mut ::std::os::raw::c_char,
    rv: *mut *mut ::std::os::raw::c_char,
) {
    let c_str = std::ffi::CStr::from_ptr(route_id);
    println!("Call with route ID {} was ended", c_str.to_str().unwrap());
}

unsafe extern "C" fn on_handle_call(
    route_id: *mut ::std::os::raw::c_char,
    raddr: *mut ::std::os::raw::c_char,
    rv: *mut *mut ::std::os::raw::c_char,
) {
    let route_id_c_str = std::ffi::CStr::from_ptr(route_id);
    let raddr_c_str = std::ffi::CStr::from_ptr(raddr);

    println!("Call with route ID {:?} and remote address {:?} started", route_id_c_str, raddr_c_str);
}

pub fn tti() {
    unsafe {
        // This can happen in the main.rs

        let ptr = saltpanelo::SaltpaneloNewAdapter(
            Some(on_request_call),
            Some(on_call_disconnected),
            Some(on_handle_call),
            Some(open_url),
            CString::new("ws://localhost:1338").unwrap().into_raw(),
            CString::new("127.0.0.1").unwrap().into_raw(),
            0,
            10000,
            CString::new("https://pojntfx.eu.auth0.com/")
                .unwrap()
                .into_raw(),
            CString::new("An94hvwzqxMmFcL8iEpTVrd88zFdhVdl")
                .unwrap()
                .into_raw(),
            CString::new("https://localhost:11337").unwrap().into_raw(),
        );

        println!("{:#?}", ptr);

        let res = saltpanelo::SaltpaneloAdapterLogin(ptr);

        let c_str = std::ffi::CStr::from_ptr(res);

        println!("{:?}", c_str.to_str().unwrap());

        // TODO: Adapter linker?

        // This needs to happen when a call is being started
        // How does this work?
        let rv = saltpanelo::SaltpaneloAdapterRequestCall(
            ptr,
            CString::new("jane@example.org").unwrap().into_raw(),
            CString::new("12345").unwrap().into_raw(),
        );

        if std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap().eq("") {
            println!(
                "Error in SalpaneloAdapterRequestCall: {}",
                std::ffi::CStr::from_ptr(rv.r1).to_str().unwrap()
            );
        }

        if rv.r0 == 1 {
            println!("Callee accepted the call");
        } else {
            println!("Callee denied the call");
        }
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
