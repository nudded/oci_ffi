extern crate libc;
use libc::{c_int, c_ulong, c_void, size_t};

use std::ptr;

// Add standard oracle types for more compliant external FFI description
#[allow(non_camel_case_types)]
type ub4 = c_ulong;
#[allow(non_camel_case_types)]
type sword = c_int;

// Standard way for defining structs that have no fields in the external C library
pub enum OCIEnv {}

// all possible error codes as defined in the documentation
pub enum OCIError {
    OCI_SUCCESS = 0,
    OCI_SUCCESS_WITH_INFO = 1,
    OCI_NO_DATA = 100,
    OCI_ERROR = -1,
    OCI_INVALID_HANDLE = -2,
    OCI_NEED_DATA = 99,
    OCI_STILL_EXECUTING = -3123,
    OCI_CONTINUE = -24200,
    OCI_ROWCBK_DONE = -24201,
}

#[link(name = "clntsh")]
extern "system" {
    fn OCIEnvCreate(envp: *mut *mut OCIEnv,
                    mode: ub4,
                    ctxp: *const c_void,
                    malocfp: *const extern "system" fn(ctxp: *mut c_void, size: size_t) -> *mut c_void,
                    ralocfp: *const extern "system" fn(ctxp: *mut c_void, memptr: *mut c_void, newsize: size_t) -> *mut c_void,
                    mfreep: *const extern "system" fn(ctxp: *mut c_void, memptr: *mut c_void),
                    xtramemsz: size_t,
                    usermempp: *mut *mut c_void) -> sword;
}

pub fn oci_env_create() -> *mut OCIEnv {

    // null pointer because the enum cannot be instantiated!
    // otherwise OCIEnv would have been used!
    let mut oci_env = ptr::null_mut();
    let res = unsafe {
        OCIEnvCreate(&mut oci_env,
                     0 as ub4,
                     ptr::null(),
                     ptr::null(),
                     ptr::null(),
                     ptr::null(),
                     0 as size_t,
                     ptr::null_mut())
    };
    println!("{:?}", res);
    oci_env
}

