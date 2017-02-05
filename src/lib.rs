extern crate libc;

// don't know why it complains about unused imports here, without these it cannot compile
#[allow(unused_imports)]
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
    /* initialize handles */
    fn OCIHandleAlloc(parenth: *const c_void,
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
        OCIHandleAlloc(oci_env as *const _,
                       &mut handle,
                       htype.into(),
                       0,
                       ptr::null_mut())
    };

    check_error!(res, handle)

}


#[link(name = "clntsh")]
extern "system" {
    fn OCIHandleFree(hndlp: *mut c_void, htype: ub4) -> sword;
}

/// Free handles
pub fn oci_handle_free(handle: *mut OCIHandle, handle_type: OCIHandleType) -> OracleResult<()> {

    let res = unsafe { OCIHandleFree(handle as *mut c_void, handle_type.into()) };
    check_error!(res, ())
}

#[link(name = "clntsh")]
extern "system" {
    fn OCILogon2(envhp: *mut OCIEnv,
                 errhp: *mut OCIHandle,
                 svchp: *mut *mut OCISrvCtx,
                 username: *const OraText,
                 uname_len: ub4,
                 password: *const OraText,
                 passwd_len: ub4,
                 dbname: *const OraText,
                 dbname_len: ub4,
                 mode: ub4)
                 -> sword;
}

pub fn oci_logon2(env: *mut OCIEnv,
                  error_handle: *mut OCIHandle,
                  username: &str,
                  password: &str,
                  dbname: &str,
                  mode: ub4)
                  -> OracleResult<*mut OCISrvCtx> {

    let mut srvctx = ptr::null_mut();

    let res = unsafe {
        OCILogon2(env,
                  error_handle,
                  &mut srvctx,
                  username.as_ptr(),
                  username.len() as ub4,
                  password.as_ptr(),
                  password.len() as ub4,
                  dbname.as_ptr(),
                  dbname.len() as ub4,
                  mode)
    };
    check_error!(res, srvctx, error_handle)
}
