//! Idiomatic Rust error handling for the Gmsh API.

use std::error::Error;
use std::fmt::Display;
use std::fmt as fmt;

/// The error type for all Gmsh API functions.
#[derive(Debug)]
pub enum GmshError {
    /// The Gmsh context wasn't properly initialized, or a required library component is missing.
    /// For example, calling any `fltk` functions without a linked FLTK library.
    Initialization, // -1 everywhere
    /// One of Gmsh's "shell" methods couldn't run successfully.
    /// For example, a bad file path was given to the `open` function.
    Execution, // 1 in top-level Gmsh
    /// Errors from the Rust/C FFI interface.
    CInterface, // Problems from the Rust/C FFI interface
    /// A function that mutates the model couldn't complete successfully.
    /// For example, addPoint couldn't succeed because of a tag collision.
    ModelMutation, // 1 in a model
    /// A data lookup getter function failed.
    /// For example, tried to work on a view that doesn't exist.
    ModelLookup, // 2 in a model
    /// The function couldn't successfully use a required input parameter.
    /// For example,  a user-specified quadrature scheme couldn't be applied to the data.
    ModelBadInput, // 3 in a model
    /// A parallelizable mesh query function failed
    ModelParallelMeshQuery, // 4 in a model
    /// The given option doesn't exist in Gmsh.
    UnknownOption, // 1 in an option function
    /// Any unexpected error codes in the Gmsh API.
    UnknownError,
}

/// Type alias for Result using `GmshError`.
pub type GmshResult<T> = Result<T, GmshError>;

impl Display for GmshError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GmshError::Initialization => {
                write!(f, "initialization error for Gmsh or an associated library is missing")
            },
            _ => write!(f, "big ol error")
        }
    }
}

impl Error for GmshError {}

// Handle error codes from top-level Gmsh functions.
#[doc(hidden)]
#[macro_export]
macro_rules! check_main_error {
    ($ierr:expr, $return_val: expr) => {
        match $ierr {
            0 => Ok($return_val),
            -1 => Err(GmshError::Initialization),
            1 => Err(GmshError::Execution),
            _ => Err(GmshError::UnknownError),
        }
    };
}

// Handle error codes from Gmsh model functions.
#[doc(hidden)]
#[macro_export]
macro_rules! check_model_error {
    ($ierr:expr, $return_val: expr) => {
        match $ierr {
            0 => Ok($return_val),
            -1 => Err(GmshError::Initialization),
            1 => Err(GmshError::ModelMutation),
            2 => Err(GmshError::ModelLookup),
            3 => Err(GmshError::ModelBadInput),
            4 => Err(GmshError::ModelParallelMeshQuery),
            _ => Err(GmshError::UnknownError),
        }
    };
}

// Handle error codes from Gmsh option configuration functions.
#[doc(hidden)]
#[macro_export]
macro_rules! check_option_error {
    ($ierr:expr, $return_val: expr) => {
        match $ierr {
            0 => Ok($return_val),
            -1 => Err(GmshError::Initialization),
            1 => Err(GmshError::UnknownOption),
            _ => Err(GmshError::UnknownError),
        }
    };
}
