extern crate libc;

use libc::{c_int, c_ulong, c_ushort, c_void, size_t};

use std::ptr;

mod types;
#[macro_use]
mod errors;
use types::*;

use errors::*;

#[link(name = "clntsh")]
extern "system" {

    /* create the OCIEnv handle! */
    fn OCIEnvCreate(envp: *mut *mut OCIEnv,
                    mode: ub4,
                    ctxp: *const c_void,
                    malocfp: *const extern "system" fn(ctxp: *mut c_void, size: size_t)
                                                       -> *mut c_void,
                    ralocfp: *const extern "system" fn(ctxp: *mut c_void,
                                                       memptr: *mut c_void,
                                                       newsize: size_t)
                                                       -> *mut c_void,
                    mfreep: *const extern "system" fn(ctxp: *mut c_void, memptr: *mut c_void),
                    xtramemsz: size_t,
                    usermempp: *mut *mut c_void)
                    -> sword;
}

pub fn oci_env_create(mode: OCIMode) -> OracleResult<*mut OCIEnv> {

    // null pointer because the enum cannot be instantiated!
    // otherwise OCIEnv would have been used!
    let mut oci_env = ptr::null_mut();
    let res = unsafe {
        OCIEnvCreate(&mut oci_env,
                     mode as ub4,
                     ptr::null(),
                     ptr::null(),
                     ptr::null(),
                     ptr::null(),
                     0 as size_t,
                     ptr::null_mut())
    };

    check_error!(res, oci_env)
}

#[link(name = "clntsh")]
extern "system" {
    /* create the OCIEnv handle with charset options */
    fn OCIEnvNlsCreate(env: *mut *mut OCIEnv,
                       mode: ub4,
                       ctxp: *mut c_void,
                       malocfp: Option<extern "system" fn(ctxp: *mut c_void, size: size_t)
                                                          -> *mut c_void>,
                       ralocfp: Option<extern "system" fn(ctxp: *mut c_void,
                                                          memptr: *mut c_void,
                                                          newsize: size_t)
                                                          -> *mut c_void>,
                       mfreefp: Option<extern "system" fn(ctxp: *mut c_void,
                                                          memptr: *mut c_void)>,
                       xtramemsz: size_t,
                       usrmempp: *mut *mut c_void,
                       charset: ub2,
                       ncharset: ub2)
                       -> sword;
}

pub fn oci_env_nls_create(mode: OCIMode) -> OracleResult<*mut OCIEnv> {
    let mut oci_env = ptr::null_mut();
    let res = unsafe {
        OCIEnvNlsCreate(&mut oci_env,
                        mode as ub4,
                        ptr::null_mut(),
                        None,
                        None,
                        None,
                        0 as size_t,
                        ptr::null_mut(),
                        0,
                        0)
    };
    check_error!(res, oci_env)
}

#[link(name = "clntsh")]
extern "system" {
    /* create the OCIEnv handle with charset options */
    fn OCIHandleAlloc(parenth: *const OCIEnv,
                      hndlpp: *mut *mut OCIHandle,
                      htype: ub4,
                      xtramem_sz: size_t,
                      usrmempp: *mut *mut c_void)
                      -> sword;
}

/// Allocates handles under the given environment
pub fn oci_handle_alloc(oci_env: *mut OCIEnv,
                        htype: OCIHandleType)
                        -> OracleResult<*mut OCIHandle> {

    let mut handle = ptr::null_mut();
    let res = unsafe {
        OCIHandleAlloc(oci_env as *const OCIEnv,
                       &mut handle,
                       htype.into(),
                       0,
                       ptr::null_mut())
    };

    check_error!(res, handle)

}
