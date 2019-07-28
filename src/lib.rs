extern crate gmsh_sys;

use std::env::Args;
// use std::io::prelude::*;
use std::io;
use std::os::raw::{c_char, c_int};

use std::ffi::CString;

pub mod geo;

/// Geometrical entities have a dimension and a tag
pub struct VectorPair(Vec<(i32, i32)>);

/// Gmsh context object
pub struct Gmsh;

impl Gmsh {
    pub fn initialize(args: Args, read_configs: bool) -> io::Result<Gmsh> {
        println!("opening Gmsh...");

        // memory leak (?)
        let mut argv: Vec<*mut c_char> = args
            .map(|arg| CString::new(arg).unwrap().into_raw())
            .collect();

        let iread_configs = read_configs as c_int;

        let mut ierr: c_int = 0;

        unsafe {
            gmsh_sys::gmshInitialize(
                argv.len() as c_int,
                argv.as_mut_ptr(),
                iread_configs,
                &mut ierr,
            );

            // retake vector pointers to free them
            for arg_str in &argv {
                let _ = CString::from_raw(*arg_str);
            }
        }

        Ok(Gmsh)
    }
}

impl Drop for Gmsh {
    fn drop(&mut self) {
        println!("finalizing Gmsh...");
    }
}
//pub fn initialize(args: Args, read_configs: bool) -> io::Result<()> {
//
//    // memory leak (?)
//    let mut argv: Vec<*mut c_char> = args
//        .map(|arg| CString::new(arg).unwrap().into_raw())
//        .collect();
//
//    let iread_configs = read_configs as c_int;
//
//    let mut ierr: c_int = 0;
//
//    unsafe {
//        gmsh_sys::gmshInitialize(
//            argv.len() as c_int,
//            argv.as_mut_ptr(),
//            iread_configs,
//            &mut ierr,
//        );
//
//        // retake vector pointers to free them
//        for arg_str in &argv {
//            let _ = CString::from_raw(*arg_str);
//        }
//    }
//
//    Ok(())
//}

// pub fn finalize() -> io::Result<()> {
//     println!("finalizing Gmsh...");
//
//     Ok(())
// }

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
