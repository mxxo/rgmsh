//! The built-in geometry kernel

use super::*;
use crate::{impl_kernel, GmshError, GmshResult};

/// An instance of the built-in kernel
pub struct Geo<'a> {
    name: &'static str,
    c_name: CString,
    phantom: PhantomData<&'a Gmsh>,
}

impl_kernel!(Geo);

impl<'a> Geo<'a> {
    /// Create a new Gmsh model using the built-in kernel.
    // todo: fix me for setting which model is the current one.
    // idea: keep a list of already used model names and only allow one at once
    #[must_use]
    pub fn create(_: &'a Gmsh, name: &'static str) -> GmshResult<Geo<'a>> {
        let c_name = get_cstring(name)?;
        unsafe {
            let mut ierr: c_int = 0;
            // also sets the added model as the current model
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
            let geo = Geo {
                name,
                c_name,
                phantom: PhantomData,
            };
            check_main_error!(ierr, geo)
        }
    }
}
