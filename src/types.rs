#![allow(non_camel_case_types, dead_code)]

use libc::{c_int, c_ulong, c_ushort, c_void, size_t};

// Add standard oracle types for more compliant external FFI description
pub type ub4 = c_ulong;
pub type ub2 = c_ushort;
pub type sword = c_int;

/// Standard way for defining structs that have no fields in the external C library
pub enum OCIEnv {}

/// Oracle handle
pub enum OCIHandle {}

/// all possible error codes as defined in the documentation
pub enum OCIError {
    OCI_SUCCESS,
    OCI_SUCCESS_WITH_INFO,
    OCI_NO_DATA,
    OCI_ERROR,
    OCI_INVALID_HANDLE,
    OCI_NEED_DATA,
    OCI_STILL_EXECUTING,
    OCI_CONTINUE,
    OCI_ROWCBK_DONE,
}

/// use more natural conversions for converting enums to error_codes
impl From<OCIError> for sword {
    fn from(error_code: OCIError) -> sword {
        match error_code {
            OCIError::OCI_SUCCESS => 0,
            OCIError::OCI_SUCCESS_WITH_INFO => 1,
            OCIError::OCI_NO_DATA => 100,
            OCIError::OCI_ERROR => -1,
            OCIError::OCI_INVALID_HANDLE => -2,
            OCIError::OCI_NEED_DATA => 99,
            OCIError::OCI_STILL_EXECUTING => -3123,
            OCIError::OCI_CONTINUE => -24200,
            OCIError::OCI_ROWCBK_DONE => -24201,
        }
    }
}

pub trait IntoOCIError {
    fn into_oci_error(self) -> OCIError;
}

impl IntoOCIError for sword {
    fn into_oci_error(self) -> OCIError {
        match self {
            0 => OCIError::OCI_SUCCESS,
            1 => OCIError::OCI_SUCCESS_WITH_INFO,
            100 => OCIError::OCI_NO_DATA,
            -1 => OCIError::OCI_ERROR,
            -2 => OCIError::OCI_INVALID_HANDLE,
            99 => OCIError::OCI_NEED_DATA,
            -3123 => OCIError::OCI_STILL_EXECUTING,
            -24200 => OCIError::OCI_CONTINUE,
            -24201 => OCIError::OCI_ROWCBK_DONE,
            _ => panic!("does not work!"),
        }
    }
}

/// all possible modes as defined
pub enum OCIMode {
    OCI_DEFAULT = 0,
    OCI_THREADED = 1,
    OCI_OBJECT = 2,
    OCI_EVENTS = 4,
    OCI_NO_UCB = 40,
    OCI_ENV_NO_MUTEX = 80,
    OCI_NEW_LENGTH_SEMANTICS = 20000,
    OCI_SUPPRESS_NLS_VALIDATION = 100000,
    OCI_NCHAR_LITERAL_REPLACE_ON = 400000,
    OCI_NCHAR_LITERAL_REPLACE_OFF = 800000,
    OCI_ENABLE_NLS_VALIDATION = 1000000,
}
