//! Unofficial Rust bindings to the Gmsh API.
//!
//! From the [Gmsh website](http://gmsh.info/):
//! > Gmsh is a free 3D finite element mesh generator with a built-in CAD engine and post-processor.
//!
//! Gmsh is copyright (C) 1997-2019 by C. Geuzaine and J.-F. Remacle
//!
//! The full Gmsh reference manual is available at: [http://gmsh.info/doc/texinfo/gmsh.html](http://gmsh.info/doc/texinfo/gmsh.html)
//!
//! ## The Gmsh API
//!
//! Gmsh has four main modules:
//! 1. CAD geometry
//! 2. Mesh generation
//! 3. External solver interfacing
//! 4. Post-processing
//!
//! ## Rust API design
//! Gmsh uses tags to keep track of different components. For example, adding a
//! point to a geometry model will return a `PointTag`.
//!
//! Tags are used everywhere in Gmsh, not just for geometry.
//! Other examples are:
//! * Mesh elements,
//! * Post-processing data sets,
//! * Mesh refinement fields.

extern crate gmsh_sys;

use std::env::Args;
use std::os::raw::{c_char, c_int, c_void};

use std::ffi::CString;

pub mod err;
pub use err::{GmshError};

pub type GmshResult<T> = Result<T, GmshError>;

pub mod geo;
use geo::Geo;

mod log;
use log::GmshLog;

// mesh
struct FieldTag(i64);

// post-processing
struct ViewTag(i64);

/// Gmsh context object
pub struct Gmsh {}

pub fn get_cstring(istr: &str) -> GmshResult<CString> {
    let c_str = CString::new(String::from(istr));
    match c_str {
        Ok(c_str) => Ok(c_str),
        Err(_) => Err(GmshError::CInterface),
    }
}

// gmsh {
//
//     model {
//         mesh {
//             field {}
//         }
//         geo {
//             mesh {}
//         }
//         occ {}
//     }
//
//     view {}
//     plugin {}
//     options {}
//     graphics {}
//     fltk {}
//     onelab {}
//     logger {}
//
//    }

impl Gmsh {
    pub fn initialize() -> GmshResult<Gmsh> {
        println!("opening Gmsh...");

        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshInitialize(
                // argc
                1 as c_int,
                // argv
                [CString::new("gmsh").unwrap().into_raw()].as_mut_ptr(),
                // don't read configuration files
                0,
                // error out-parameter
                &mut ierr,
            );

            if ierr == 0 {
                // send logs to terminal
                Gmsh::set_number_option("General.Terminal", 1.)?;
                Ok( Gmsh{} )
            } else {
                Err(GmshError::Initialization)
            }
        }
    }

    /// Make a new model using the built-in Gmsh geometry kernel
    pub fn new_native_model(&self, name: &'static str) -> GmshResult<Geo> {
        println!("added built-in geometry model {} ", name);
        Geo::new(self, name)
    }

    // /// Make a new model using the OpenCASCADE geometry kernel
    // pub fn new_occ_model(&mut self, name: &'static str) -> GmshResult<Occ> {
    //     println!("added OpenCASCADE model {} ", name);
    //     Occ::new(self, name)
    // }

    /// Set a numeric option
    pub fn set_number_option(name: &str, value: f64) -> GmshResult<()> {
        let cname = get_cstring(name)?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshOptionSetNumber(cname.as_ptr(), value, &mut ierr);
            match ierr {
                0 => Ok(()),
               -1 => Err(GmshError::from(GmshError::Initialization)),
                1 => Err(GmshError::from(GmshError::UnknownOption)),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Set a string option
    pub fn set_string_option(name: &str, value: &str) -> GmshResult<()> {
        let cname = get_cstring(name)?;
        let cvalue = get_cstring(value)?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshOptionSetString(cname.as_ptr(), cvalue.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok(()),
                _ => Err(GmshError::from(GmshError::UnknownOption)),
            }
        }
    }


}

impl Drop for Gmsh {
    fn drop(&mut self) {
        println!("finalizing Gmsh...");
        unsafe {
           // don't check finalization errors
            let mut ierr: c_int = 0;
            gmsh_sys::gmshFinalize(&mut ierr);
        }
    }
}


#[cfg(test)]
mod tests {

    // import all names from the outer scope
    use super::*;

    #[test]
    // #[should_panic]
    pub fn dangling_geo() {

    }
}
