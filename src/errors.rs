use types::*;

use std::ptr;

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

/// Result of an oracle function call
pub type OracleResult<T> = Result<T, OracleError>;

/// convenience function for converting errors and getting more information
pub fn check_error(result_code: sword, handle: Option<*mut OCIHandle>, location: &str) -> Option<OracleError> {
    match result_code.into_oci_error() {
        OCIError::OCI_SUCCESS => None,
        OCIError::OCI_SUCCESS_WITH_INFO => Some(check_error_with_handle(handle, result_code, "Success with info", location)),
        OCIError::OCI_NO_DATA => Some(OracleError::new(result_code, "No data", location)),
        OCIError::OCI_ERROR => Some(check_error_with_handle(handle, result_code, "Error without details", location)),
        OCIError::OCI_INVALID_HANDLE => Some(OracleError::new(result_code, "Invalid handle", location)),
        OCIError::OCI_NEED_DATA => Some(OracleError::new(result_code, "Need data", location)),
        OCIError::OCI_STILL_EXECUTING => Some(OracleError::new(result_code, "Still executing", location)),
        /* this is to be investigated, it seems these cannot be returned from oracle funcations */
        OCIError::OCI_CONTINUE => None,
        OCIError::OCI_ROWCBK_DONE => None,
    }
}

fn check_error_with_handle(handle: Option<*mut OCIHandle>, result_code: sword, default_msg: &str, location: &str) -> OracleError {
    if let Some(handle) = handle {
        let msg = oci_error_get(handle, location);
        OracleError::new(result_code, &*msg, location)
    } else {
        OracleError::new(result_code, default_msg, location)
    }

}

#[link(name = "clntsh")]
extern "system" {

    // get the error information out of the DB
    fn OCIErrorGet(hndlp: *mut OCIHandle,
                   recordno: ub4,
                   sqlstate: OraText,
                   errcodep: *mut sb4,
                   bufp: OraText,
                   bufsize: ub4,
                   error_type: ub4) -> sword;

}

fn oci_error_get(handle: *mut OCIHandle, location: &str) -> OracleError {
    let errcodep: *mut sword = &mut 0;
    let mut bufp = String::with_capacity(512);
    let mut error_code: sword;

    let res = unsafe {
        OCIErrorGet(handle,
                    1,
                    ptr::null_mut(),
                    errcodep,
                    mut bufp.as_ptr(),
                    bufp.capacity(),
                    OCIHandleType::OCI_HTYPE_ERROR.into());
        error_code = *errcodep;
    };

    OracleError::new(error_code, &*bufp, location)
}

