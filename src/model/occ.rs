//! The OpenCASCADE geometry kernel

use super::*;

/// An instance of the OpenCASCADE kernel
pub struct Occ<'a> {
    name: &'static str,
    c_name: CString,
    phantom: PhantomData<&'a Gmsh>,
}

impl<'a> Occ<'a> {

    /// Make a new instance of the OpenCASCADE kernel.
    // todo: fix me for the right model names
    #[must_use]
    pub fn new(gmsh: &'a Gmsh, name: &'static str) -> GmshResult<Occ<'a>> {
        let c_name = get_cstring(name)?;
        unsafe {
            let mut ierr: c_int = 0;
            // also sets the added model as the current model
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok( Occ { name, c_name, phantom: PhantomData, } ),
                -1 => Err(GmshError::Initialization),
                _ => Err(GmshError::Execution),
            }
        }
    }
}
