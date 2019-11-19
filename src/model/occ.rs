//! The `OpenCASCADE` geometry kernel

use super::*;
use crate::{GmshError, GmshResult, check_main_error, check_model_error};
use crate::interface::occ as factory;

/// All angle values are in radians, commonly given as fractions of π.

impl<'a> OccModel<'a> {

    #[must_use]
    fn add_point_gen(
         &mut self,
         coords: (f64, f64, f64),
         mesh_size: Option<f64>,
     ) -> GmshResult<PointTag> {
         self.set_current()?;

         let (x, y, z) = coords;

         let lc = mesh_size.unwrap_or(0.);
         let auto_number = -1;

         unsafe {
             let mut ierr: c_int = 0;
             //let add_point_fn = impl_kernel!(@kernel_prefix $kernel_name, add_point);
             let out_tag = factory::add_point(x, y, z, lc, auto_number, &mut ierr);
             check_model_error!(ierr, PointTag(out_tag))
         }
     }


    /// Add a point to the model by specifying its coordinates.
    #[must_use]
    pub fn add_point(&mut self, x: f64, y: f64, z: f64) -> GmshResult<PointTag> {
        println!("added basic point");
        self.add_point_gen((x, y, z), None)
    }

    /// Add a point to the model and specify a target mesh size `lc` there.
    #[must_use]
    pub fn add_point_with_lc(&mut self, x: f64, y: f64, z: f64, lc: f64) -> GmshResult<PointTag> {
        println!("added point with lc");
        self.add_point_gen((x, y, z), Some(lc))
    }

    /// Add a straight line between two points.
    #[must_use]
    pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> GmshResult<CurveTag> {
        self.set_current()?;
        let auto_number = -1;
        unsafe {
            let mut ierr: c_int = 0;
            let out_tag = factory::add_line(p1.to_raw(), p2.to_raw(), auto_number, &mut ierr);
            check_model_error!(ierr, CurveTag(out_tag))
        }
    }

    /// Add a box with a starting point and side lengths from that point.
    #[must_use]
    pub fn add_box(&mut self, start_point: (f64, f64, f64),
                   extents: (f64, f64, f64)) -> GmshResult<VolumeTag> {
        self.set_current()?;
        let mut ierr: c_int = 0;
        let automatic_tag: c_int = -1;
        unsafe {
            let out_tag =
                factory::add_box(start_point.0,
                                               start_point.1,
                                               start_point.2,
                                               extents.0,
                                               extents.1,
                                               extents.2,
                                               automatic_tag,
                                               &mut ierr);
            check_model_error!(ierr, VolumeTag(out_tag))
        }
    }

    /// Add a sphere with a centroid and radius.
    #[must_use]
    pub fn add_sphere(&mut self, centroid: (f64, f64, f64), radius: f64) -> GmshResult<VolumeTag> {
        let polar_min = -std::f64::consts::FRAC_PI_2;
        let polar_max = std::f64::consts::FRAC_PI_2;
        let azimuth = 2. * std::f64::consts::PI;
        self.add_sphere_gen(centroid, radius, (polar_min, polar_max), azimuth)
    }

    /// Add an angular section of a sphere.
    /// The polar angles are `(min, max)` relative to the x-axis on the xy-plane,
    /// and the azimuth is relative to the z-axis.
    /// ```
    /// # use gmsh::{Gmsh, GmshResult, add_points};
    /// # use gmsh::model::{GeoKernel, occ::*};
    /// # fn main() -> GmshResult<()> {
    /// # let gmsh = Gmsh::initialize()?;
    /// # let mut geom = gmsh.create_occ_model("model")?;
    /// use std::f64::consts;
    ///
    /// // make a 1/8th sphere section
    /// let sphere_section = geom.add_sphere_section(
    ///                          (0., 0., 0.), 1.,
    ///                          (0., consts::FRAC_PI_2),
    ///                          consts::FRAC_PI_2)?;
    ///
    /// // recover a basic sphere by setting polar = (-π/2, π/2) and azimuth = 2π.
    /// let basic_sphere = geom.add_sphere_section(
    ///                        (10., 10., 10.), 1.,
    ///                        (-consts::FRAC_PI_2, consts::FRAC_PI_2),
    ///                        2. * consts::PI)?;
    /// # Ok(())
    /// # }
    /// ```
    #[must_use]
    pub fn add_sphere_section(&mut self, centroid: (f64, f64, f64), radius: f64, polar: (f64, f64), azimuth: f64 // fx: f64, y: f64, z: f64,
    ) -> GmshResult<VolumeTag> {
        self.add_sphere_gen(centroid, radius, polar, azimuth)
    }



    #[doc(hidden)]
    #[must_use]
    fn add_sphere_gen(
        &mut self,
        centroid: (f64, f64, f64),
        radius: f64,
        polar: (f64, f64),
        azimuth: f64,
    ) -> GmshResult<VolumeTag> {
        self.set_current()?;
        unsafe {
            let mut ierr: c_int = 0;
            let automatic_tag: c_int = -1;
            let out_tag = factory::add_sphere(
                centroid.0,
                centroid.1,
                centroid.2,
                radius,
                automatic_tag,
                polar.0,
                polar.1,
                azimuth,
                &mut ierr,
            );
            check_model_error!(ierr, VolumeTag(out_tag))
        }
    }

    /// Add a torus with a centroid and radii values `(main_radius, pipe_radius)`.
    #[must_use]
    pub fn add_torus(
        &mut self, centroid: (f64, f64, f64),
        radii: (f64, f64)
    ) -> GmshResult<VolumeTag> {
        let angle = 2. * std::f64::consts::PI;
        self.add_torus_gen(centroid, radii, angle)
    }

    /// Add an angular torus section using the main torus radius. To recover
    /// a basic torus, set the `angle` parameter to 2π.
    #[must_use]
    pub fn add_torus_section(
        &mut self, centroid: (f64, f64, f64), radii: (f64, f64), angle: f64
    ) -> GmshResult<VolumeTag> {
        self.add_torus_gen(centroid, radii, angle)
    }

    #[doc(hidden)]
    #[must_use]
    fn add_torus_gen(
        &mut self,
        centroid: (f64, f64, f64),
        radii: (f64, f64),
        angle: f64
    ) -> GmshResult<VolumeTag> {
        self.set_current()?;
        unsafe {
            let mut ierr: c_int = 0;
            let automatic_tag: c_int = -1;
            let out_tag = factory::add_torus(
                centroid.0,
                centroid.1,
                centroid.2,
                radii.0,
                radii.1,
                automatic_tag,
                angle,
                &mut ierr,
            );
            check_model_error!(ierr, VolumeTag(out_tag))
        }
    }
}
