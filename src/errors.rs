use types::*;

use std::ptr;
use std::ffi::CString;

/// Oracle error holder
#[derive(Debug)]
pub struct OracleError {
    /// Error code coming from Oracle
    error_code: sword,
    /// Message coming from Oracle
    message: String,
    /// Information about the caller, where the error occurred
    error_location: String,
}

impl OracleError {
    pub fn new(code: sword, message: &str, location: &str) -> OracleError {
        OracleError {
            error_code: code,
            message: message.to_string(),
            error_location: location.to_string(),
        }
    }
}

// I'm just having fun with macros here, could have also been 2 functions
#[macro_export]
macro_rules! check_error {
    ($code:expr, $result:expr) => {check_error($code, None, &format!("{}:{}", file!(), line!()), $result)};
    ($code:expr, $result:expr, $handle:expr) => {check_error($code, Some($handle), &format!("{}:{}", file!(), line!()), $result)};
}

pub fn check_error<T>(result_code: sword,
                      handle: Option<*mut OCIHandle>,
                      location: &str,
                      result: T)
                      -> OracleResult<T> {
    match check_error_code(result_code, handle, location) {
        None => Ok(result),
        Some(error) => Err(error),
    }
}

/// Result of an oracle function call
pub type OracleResult<T> = Result<T, OracleError>;

/// convenience function for converting errors and getting more information
pub fn check_error_code(result_code: sword,
                        handle: Option<*mut OCIHandle>,
                        location: &str)
                        -> Option<OracleError> {
    match result_code.into_oci_error() {
        OCIError::OCI_SUCCESS => None,
        OCIError::OCI_SUCCESS_WITH_INFO => {
            Some(check_error_with_handle(handle, result_code, "Success with info", location))
        }
        OCIError::OCI_NO_DATA => Some(OracleError::new(result_code, "No data", location)),
        OCIError::OCI_ERROR => {
            Some(check_error_with_handle(handle, result_code, "Error without details", location))
        }
        OCIError::OCI_INVALID_HANDLE => {
            Some(OracleError::new(result_code, "Invalid handle", location))
        }
        OCIError::OCI_NEED_DATA => Some(OracleError::new(result_code, "Need data", location)),
        OCIError::OCI_STILL_EXECUTING => {
            Some(OracleError::new(result_code, "Still executing", location))
        }
        /* this is to be investigated, it seems these cannot be returned from oracle funcations */
        OCIError::OCI_CONTINUE => None,
        OCIError::OCI_ROWCBK_DONE => None,
    }
}

fn check_error_with_handle(handle: Option<*mut OCIHandle>,
                           result_code: sword,
                           default_msg: &str,
                           location: &str)
                           -> OracleError {
    if let Some(handle) = handle {
        oci_error_get(handle, location)
    } else {
        OracleError::new(result_code, default_msg, location)
    }
}

#[link(name = "clntsh")]
extern "system" {

    // get the error information out of the DB
    fn OCIErrorGet(hndlp: *mut OCIHandle,
                   recordno: ub4,
                   sqlstate: *mut OraText,
                   errcodep: *mut sb4,
                   bufp: OraText,
                   bufsize: ub4,
                   error_type: ub4)
                   -> sword;

}

const ERROR_BUF_SIZE: usize = 512;

fn oci_error_get(handle: *mut OCIHandle, location: &str) -> OracleError {
    let errcodep: *mut sword = &mut 0;
    let mut bufp = Vec::with_capacity(ERROR_BUF_SIZE);
    let error_code: sword;
    let mut message = String::from("Error could not be read!");

    let res = unsafe {
        OCIErrorGet(handle,
                    1,
                    ptr::null_mut(),
                    errcodep,
                    bufp.as_mut_ptr() as OraText,
                    bufp.capacity() as ub4,
                    OCIHandleType::OCI_HTYPE_ERROR.into());
        // this deref is safe
        error_code = *errcodep;

        // this code is taken from
        // https://github.com/rust-lang/rust/blob/master/src/libstd/sys/unix/os.rs#L106
        if let Ok(string) = CString::from_vec_unchecked(bufp).into_string() {
            message = string
        }
    };
    OracleError::new(error_code, &*message, location)
}
