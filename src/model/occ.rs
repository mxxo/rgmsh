//! The `OpenCASCADE` geometry kernel

use super::*;
use crate::{impl_kernel, GmshError, GmshResult, check_main_error, check_model_error};

/// An instance of the `OpenCASCADE` kernel
pub struct Occ<'a> {
    name: &'static str,
    c_name: CString,
    phantom: PhantomData<&'a Gmsh>,
}

impl_kernel!(Occ);

impl<'a> Occ<'a> {
    /// Create a new Gmsh model using the `OpenCASCADE` kernel.
    // todo: fix me for the right model names
    #[must_use]
    pub fn create(_: &'a Gmsh, name: &'static str) -> GmshResult<Occ<'a>> {
        let mut ierr: c_int = 0;
        let c_name = get_cstring(name)?;
        unsafe {
            // also sets the added model as the current model
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
        }
        check_main_error!(ierr, Occ { name, c_name, phantom: PhantomData } )
    }

    #[must_use]
    pub fn add_box(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        dx: f64,
        dy: f64,
        dz: f64,
    ) -> GmshResult<VolumeTag> {
        self.set_current()?;
        let mut ierr: c_int = 0;
        let automatic_tag: c_int = -1;
        unsafe {
            let out_tag =
                crate::interface::occ::add_box(x, y, z, dx, dy, dz, automatic_tag, &mut ierr);
            check_model_error!(ierr, VolumeTag(out_tag))
        }
    }

    #[must_use]
    pub fn add_sphere(&mut self, x: f64, y: f64, z: f64, r: f64) -> GmshResult<VolumeTag> {
        self.add_sphere_gen(x, y, z, r, None, None, None)
    }

    #[must_use]
    pub fn add_sphere_section(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        r: f64,
        polar1: f64,
        polar2: f64,
        azimuth: f64,
    ) -> GmshResult<VolumeTag> {
        self.add_sphere_gen(x, y, z, r, Some(polar1), Some(polar2), Some(azimuth))
    }

    #[doc(hidden)]
    #[must_use]
    fn add_sphere_gen(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        r: f64,
        polar1: Option<f64>,
        polar2: Option<f64>,
        azimuth: Option<f64>,
    ) -> GmshResult<VolumeTag> {
        self.set_current()?;
        let angle1 = polar1.unwrap_or(-std::f64::consts::FRAC_PI_2);
        let angle2 = polar2.unwrap_or(std::f64::consts::FRAC_PI_2);
        let angle3 = azimuth.unwrap_or(2. * std::f64::consts::PI);
        unsafe {
            let mut ierr: c_int = 0;
            let automatic_tag: c_int = -1;
            let out_tag = crate::interface::occ::add_sphere(
                x,
                y,
                z,
                r,
                automatic_tag,
                angle1,
                angle2,
                angle3,
                &mut ierr,
            );
            check_model_error!(ierr, VolumeTag(out_tag))
        }
    }

    #[must_use]
    pub fn add_torus(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        r_maj: f64,
        r_min: f64,
    ) -> GmshResult<VolumeTag> {
        self.add_torus_gen(x, y, z, r_maj, r_min, None)
    }

    #[must_use]
    pub fn add_torus_section(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        r_maj: f64,
        r_min: f64,
        angle: f64,
    ) -> GmshResult<VolumeTag> {
        self.add_torus_gen(x, y, z, r_maj, r_min, Some(angle))
    }

    #[doc(hidden)]
    #[must_use]
    fn add_torus_gen(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        r_maj: f64,
        r_min: f64,
        angle: Option<f64>,
    ) -> GmshResult<VolumeTag> {
        self.set_current()?;
        let angle = angle.unwrap_or(2. * std::f64::consts::PI);
        unsafe {
            let mut ierr: c_int = 0;
            let automatic_tag: c_int = -1;
            let out_tag = crate::interface::occ::add_torus(
                x,
                y,
                z,
                r_maj,
                r_min,
                automatic_tag,
                angle,
                &mut ierr,
            );
            check_model_error!(ierr, VolumeTag(out_tag))
        }
    }
}
