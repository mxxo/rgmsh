#![doc(html_logo_url = "https://gitlab.onelab.info/gmsh/gmsh/blob/master/utils/icons/gmsh.svg")]
// #![deny(missing_docs)]
//!
//!
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
//! 1. Geometry
//! 2. Mesh
//! 3. Solvers
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


// todo figure out where this import belongs
extern crate gmsh_sys;

use std::os::raw::c_int;

use std::ffi::{CStr, CString};

pub mod err;
pub use err::{GmshError, GmshResult};

pub mod interface;

pub mod model;
use model::geo::Geo;
use model::occ::Occ;

// mesh
struct FieldTag(i64);

// post-processing
struct ViewTag(i64);

/// Gmsh context object
pub struct Gmsh {
    // todo add a log for used-model names
}

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
        // println!("opening Gmsh...");

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
                let mut gmsh = Gmsh {};
                gmsh.set_number_option("General.Terminal", 1.)?;
                eprintln!("Gmsh {}", gmsh.get_string_option("General.Version")?);

                Ok(gmsh)

            } else {
                Err(GmshError::Initialization)
            }
        }
    }

    /// Make a new model using the built-in Gmsh geometry kernel
    pub fn new_native_model(&self, name: &'static str) -> GmshResult<Geo> {
      //  println!("added built-in geometry model {} ", name);
        Geo::new(self, name)
    }

    /// Make a new model using the OpenCASCADE geometry kernel
    pub fn new_occ_model(&self, name: &'static str) -> GmshResult<Occ> {
      //  println!("added OpenCASCADE model {} ", name);
        Occ::new(self, name)
    }

    /// Get a numeric option.
    pub fn get_number_option(&self, name: &str) -> GmshResult<f64> {
        let cname = get_cstring(name)?;
        let mut value: f64 = 0.;
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshOptionGetNumber(cname.as_ptr(), &mut value, &mut ierr);
        }
        check_option_error!(ierr, value)
    }

    /// Set a numeric option.
    pub fn set_number_option(&mut self, name: &str, value: f64) -> GmshResult<()> {
        let cname = get_cstring(name)?;
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshOptionSetNumber(cname.as_ptr(), value, &mut ierr);
        }
        check_option_error!(ierr, ())
    }

    /// Get a string option.
    pub fn get_string_option(&self, name: &str) -> GmshResult<String> {
        let cname = get_cstring(name)?;
        let mut ierr: c_int = 0;
        // make a raw pointer from a CString
        // I don't know if I should be allocating more memory here?
        let buffer = CString::new("").unwrap();
        let mut p_buffer = buffer.into_raw();
        unsafe {
            gmsh_sys::gmshOptionGetString(cname.as_ptr(), &mut p_buffer, &mut ierr);
            // buffer length is recalculated from the modified pointer
            let str_value = CString::from_raw(p_buffer).into_string();
            match str_value {
                Ok(val) => check_option_error!(ierr, val),
                Err(_) => Err(GmshError::CInterface),
            }
        }
    }

    /// Set a string option.
    pub fn set_string_option(&mut self, name: &str, value: &str) -> GmshResult<()> {
        let cname = get_cstring(name)?;
        let cvalue = get_cstring(value)?;
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshOptionSetString(cname.as_ptr(), cvalue.as_ptr(), &mut ierr);
        }
        check_option_error!(ierr, ())
    }
}

impl Drop for Gmsh {
    fn drop(&mut self) {
       // println!("finalizing Gmsh...");
        unsafe {
            // don't check finalization errors
            let mut ierr: c_int = 0;
            gmsh_sys::gmshFinalize(&mut ierr);
        }
    }
}

/// Tests must be run with `--test-threads=1` since they depend on the shared Gmsh state.
#[cfg(test)]
mod tests {

    // import all names from the outer scope
    use super::*;
    use crate::model::*;

    // Will SIGSEGV if run in parallel
    /// Check multiple models can be made and follow the same numbering rules
    #[test]
    pub fn multiple_models() -> GmshResult<()> {
        let gmsh = Gmsh::initialize()?;
        let mut occ_geom = gmsh.new_occ_model("box")?;
        let p1 = occ_geom.add_point(0.,0.,0.)?;

        let mut native_geom = gmsh.new_native_model("bella")?;
        let p2 = native_geom.add_point(1., 1., 1.)?;

        let mut another_native_geom = gmsh.new_native_model("plane")?;
        let p3 = another_native_geom.add_point(2., 2., 2.)?;

        assert!((p1 == p2) && (p1 == p3));
        Ok(())
    }

    #[test]
    pub fn catch_unknown_options() -> GmshResult<()> {
        let mut gmsh = Gmsh::initialize()?;
        let geom = gmsh.new_occ_model("model")?;
        let bad_opt = "Bad.Option";

        let get_num_err = gmsh.get_number_option(bad_opt);
        let get_str_err = gmsh.get_string_option(bad_opt);
        let set_num_err = gmsh.set_number_option(bad_opt, 1.);
        let set_str_err = gmsh.set_string_option(bad_opt, "Garbo");

        macro_rules! is_unknown_err {
            ($err:ident) => {
                match $err {
                    Err(GmshError::UnknownOption) => (),
                    _ => panic!(),
                }
            }
        }

        is_unknown_err!(get_num_err);
        is_unknown_err!(get_str_err);
        is_unknown_err!(set_num_err);
        is_unknown_err!(set_str_err);

        Ok(())
    }

    #[test]
    pub fn set_and_return_opts() -> GmshResult<()> {
        let mut gmsh = Gmsh::initialize()?;
        let geom = gmsh.new_occ_model("model")?;

        let opt = "Solver.Name0";
        // has default value of GetDP
        let str_val = "TEST_NAME_1";
        gmsh.set_string_option(opt, str_val)?;
        assert!(str_val == gmsh.get_string_option(opt)?);

        // has default value of 0
        gmsh.set_number_option("General.Axes", 5.)?;
        assert!(5. == gmsh.get_number_option("General.Axes")?);

        Ok(())
    }

}
