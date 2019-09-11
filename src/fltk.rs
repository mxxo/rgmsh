//! Manipulation of the FLTK graphical user interface.
//!
//! Use the Gmsh GUI to check your work.
//! ```no_run
//! # use gmsh::{Gmsh, GmshResult};
//! # fn main() -> GmshResult<()> {
//! let gmsh = Gmsh::initialize()?;
//! let mut geom = gmsh.create_native_model("model")?;
//!
//! // -- add a bunch of complicated shapes
//!
//! // look at your model in the GUI
//! let gui = gmsh.run_gui()?;
//!
//! // only resumes execution when you've closed the Gmsh window
//!
//! # Ok(())
//! # }
//! ```

use std::marker::PhantomData;
use crate::{Gmsh, GmshError, GmshResult, check_main_error, c_int};

/// The FLTK GUI object
pub struct Gui<'a> {
    phantom: PhantomData<&'a Gmsh>
}

impl Gmsh {

    // will crash with "Fltk internal error" without an X display, like plain WSL
    pub fn initialize_gui(&self) -> GmshResult<Gui> {
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshFltkInitialize(&mut ierr);
        }
        check_main_error!(ierr, Gui {phantom: PhantomData})
    }

    pub fn run_gui(&self) -> GmshResult<Gui> {
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshFltkRun(&mut ierr);
        }
        check_main_error!(ierr, Gui {phantom: PhantomData})
    }

}

// impl<'a> Gui<'a> {
//
//     pub fn run(&mut self) -> GmshResult<()> {
//         let mut ierr: c_int = 0;
//         unsafe {
//             gmsh_sys::gmshFltkRun(&mut ierr);
//         }
//         check_main_error!(ierr, ())
//     }
// }
