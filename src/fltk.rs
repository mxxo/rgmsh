//! Manipulation of the FLTK graphical user interface.
//!
//! Use the Gmsh GUI to check your work.
//! ```no_run
//! # use gmsh::{Gmsh, GmshResult};
//! # fn main() -> GmshResult<()> {
//! let gmsh = Gmsh::initialize()?;
//! let mut geom = gmsh.create_occ_model("model")?;
//!
//! // -- add a bunch of complicated shapes
//!
//! // look at your model in the GUI
//! let gui = gmsh.run_gui()?;
//! # Ok(())
//! # }
//! ```

use std::marker::PhantomData;
use crate::{Gmsh, GmshError, GmshResult, check_main_error, c_int};

/// The FLTK GUI object
// TODO this needs more thought, because it's a multithreaded object
pub struct Gui<'a> {
    phantom: PhantomData<&'a Gmsh>
}

impl Gmsh {

    /// Create the GUI. If successful, open the GUI for an instant before
    /// resuming execution.
    pub fn initialize_gui(&self) -> GmshResult<Gui> {
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshFltkInitialize(&mut ierr);
        }
        check_main_error!(ierr, Gui {phantom: PhantomData})
    }

    /// Run the GUI and block the calling thread until the GUI window is closed.
    pub fn run_gui(&self) -> GmshResult<Gui> {
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshFltkRun(&mut ierr);
        }
        check_main_error!(ierr, Gui {phantom: PhantomData})
    }

}

impl<'a> Gui<'a> {

    /// Draw all the OpenGL scenes
    pub fn draw(&mut self) -> GmshResult<()> {
        let mut ierr: c_int = 0;
        unsafe {
            gmsh_sys::gmshGraphicsDraw(&mut ierr);
        }
        check_main_error!(ierr, ())
    }

    // pub fn run(&mut self) -> GmshResult<()> {
    //     let mut ierr: c_int = 0;
    //     unsafe {
    //         gmsh_sys::gmshFltkRun(&mut ierr);
    //     }
    //     check_main_error!(ierr, ())
    // }
}
