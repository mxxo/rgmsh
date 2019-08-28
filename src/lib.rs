extern crate gmsh_sys;
// extern crate gmsh_kernel;
// use gmsh_kernel::geometry;

use std::env::Args;
use std::io;
use std::os::raw::{c_char, c_int, c_void};
//use std::Box;

use std::ffi::CString;

pub mod geo;
pub mod model;
// pub mod open_cascade;

use model::Model;
use model::KernelType;
// use model::GeometryKernel;

use geo::Geo;

// mesh
struct FieldTag(i64);

// post-processing
struct ViewTag(i64);

/// Geometrical entities have a dimension and a tag
pub struct VectorPair<A, B>(Vec<(A, B)>);

// type vector_points = VectorPair<Dimension::Point, PointTag>;

/// Gmsh context object
pub struct Gmsh {
    // pub models: Vec<Model>,
    // current_model: Option<&'a mut Model<'b>>,
    // pub models: Vec<View>,
    //pub geo: Geo,
}


// #[geometry(native)]
// pub fn test() {}

/// gmsh {
///
///     model {
///         mesh {
///             field {}
///         }
///         geo {
///             mesh {}
///         }
///         occ {}
///     }
///
///     view {}
///     plugin {}
///     options {}
///     graphics {}
///     fltk {}
///     onelab {}
///     logger {}
///
///     }

impl Gmsh {
    pub fn initialize(args: Args, read_configs: bool) -> io::Result<Gmsh> {
        println!("opening Gmsh...");

        // memory leak (?)
        let mut argv: Vec<*mut c_char> = args
            .map(|arg| CString::new(arg).unwrap().into_raw())
            .collect();

        let iread_configs = read_configs as c_int;

        let mut ierr: c_int = 0;

        // causes segfault when bad options are passed in
        // e.g. "-v", verbosity level without a number afterwards.
        // The executable prints an error, this api segfaults
        unsafe {
            gmsh_sys::gmshInitialize(
                argv.len() as c_int,
                argv.as_mut_ptr(),
                iread_configs,
                &mut ierr,
            );
        }

        if ierr != 0 {
            eprintln!("error initializing Gmsh, exiting");
            panic!();
        }

        Ok(
            Gmsh {
       //        models: Vec::new(),
               // current_model: None,
            }
        )
    }

    pub fn new_native_model(&mut self, name: &'static str) -> Geo {
      //  self.models.push(Model::new(name, kernel_type));
        //self.current_model = self.models.last_mut();
        println!("added model {} ", name);
        Geo::new(self, name)
    }

    // pub fn new_occ_model(&mut self, name: &'static str) -> OCC {
    // }


}

impl Drop for Gmsh {
    fn drop(&mut self) {
        println!("finalizing Gmsh...");
        let mut ierr: c_int = 0;
        unsafe {
            // don't check finalization errors
            gmsh_sys::gmshFinalize(&mut ierr);
        }
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
