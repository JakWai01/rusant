use std::{
    ffi::{CString, c_void},
    ptr::{null, null_mut}, thread, fmt::Pointer,
};

use saltpanelo::SaltpaneloOnRequestCallResponse;

use crate::saltpanelo::{SaltpaneloAdapterLink, SaltpaneloAdapterHangupCall};

#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
pub mod saltpanelo {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}