//! The Gmsh geometry module
//!
//! There are two CAD engines you can use:
//! 1. The built-in Gmsh geometry kernel.
//! 2. The OpenCASCADE geometry kernel.
//!
//! The only way to get a geometry kernel is through a `Gmsh` context object.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # fn main() -> GmshResult<()> {
//! let gmsh = Gmsh::initialize()?;
//! let mut geom = gmsh.new_native_model("model")?;
//! # Ok(())
//! # }
//! ```
//!
//! The geometry kernel is only valid for the lifetime of `Gmsh`.
//! ```compile_fail
//! # use gmsh::{Gmsh, GmshResult};
//! # fn main() -> GmshResult<()> {
//! let gmsh = Gmsh::initialize()?;
//! let mut geom = gmsh.new_native_model("model")?;
//!
//! // -- do some stuff with geom
//!
//! // drop the Gmsh context
//! std::mem::drop(gmsh);
//! // try to use the kernel afterwards
//! geom.add_point(0., 0., 0.)?; // won't compile
//! # Ok(())
//! # }
//! ```
//!
//! ## Create, modify and delete shapes
//! You can define points, lines, 2D surfaces
//! and 3D volumes.  After defining a shape, you'll get a geometry tag to
//! identify[^unique] it.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # use gmsh::geo::{PointTag, CurveTag};
//! # fn main() -> GmshResult<()> {
//! # let gmsh = Gmsh::initialize()?;
//! // make a model using the default geometry kernel and call it `model`.
//! let mut geom = gmsh.new_native_model("model")?;
//!
//! // make a point
//! let p1 : PointTag = geom.add_point(0., 0., 0.)?;
//! // and another
//! let p2 : PointTag = geom.add_point(1., 1., 0.)?;
//!
//! // create a line from the two points
//! let l1 : CurveTag = geom.add_line(p1, p2)?;
//! # Ok(())
//! # }
//! ```
//! Geometry tags are used for:
//! * accessing shape information,
//! * making more complex shapes (like a line from two points),
//! * removing a shape from the model
//!
//! The different geometry tags are:
//! * `PointTag`
//! * `CurveTag`
//! * `WireTag`
//! * `SurfaceTag`
//! * `ShellTag`
//! * `VolumeTag`
//!
//! Since tags can only be created from successful geometry operations, you can't
//! use raw integers for tags.
//! ```compile_fail
//! # use gmsh::{Gmsh, GmshResult};
//! # use gmsh::geo::{PointTag, CurveTag};
//! # fn main() -> GmshResult<()> {
//! # let gmsh = Gmsh::initialize()?;
//! # let geom = gmsh.new_native_model("model")?;
//! // try to make a point from a raw integer
//! let p1 = PointTag(1); // won't compile
//! // try to make a line from two raw integers
//! let l1 = CurveTag(1, 2); // won't compile
//! # Ok(())
//! # }
//! ```
//!
//! This design differs from other Gmsh API
//! implementations. For example, using the `C++` API, the following example will compile and run without errors
//! (though this `Line` will probably cause a runtime error later on).
//! ```cpp
//! #include "gmsh.h"
//! int main() {
//!     gmsh::initialize();
//!     gmsh::model::geo::addLine(1, 2); // (!)
//!     gmsh::finalize();
//! }
//! ```
//! The Rust API avoids such bugs for a single model by only making tags available through API functions.
//! However, the Rust API has a similar issue if there are two or more models.
//! Since two models can have identical point tag values, tags from one can be used on the other.
//!
//! The root of this problem is that Gmsh tries very hard to keep going after errors.
//! This has a direct impact on our API's error handling capabilities, since some errors are only logged, not handled in code.
//!
//! It's your responsibility to make sure tags are used with the right model.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # fn main() -> GmshResult<()> {
//! #  let gmsh = Gmsh::initialize()?;
//! let mut geom_a = gmsh.new_native_model("jimbo")?;
//! let mut geom_b = gmsh.new_native_model("aircraft-carrier")?;
//!
//! let p_a = geom_a.add_point(0., 0., 0.)?;
//!
//! let p_b = geom_b.add_point(0., 1., 1.)?;
//! let p_c = geom_b.add_point(0., 1., 1.)?;
//!
//! // points from different models can have the same value
//! assert!(p_a == p_b, "Point tags are different!");
//!
//! // Very bad!
//! let line = geom_b.add_line(p_a, p_c)?;
//! // Will succeed! Gmsh will print an error message to console and try to keep going.
//! println!("Nonsense curve tag is {:?}", line);
//!
//! #  Ok(())
//! # }
//! ```
//!
//! Nearly all geometry functions can fail. Fallible functions will result a `GmshResult`.
//!
//! You can use the `?` operator for terse error handling.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! fn main() -> GmshResult<()> {
//!     let gmsh = Gmsh::initialize()?;
//!     let mut geom = gmsh.new_native_model("model")?;
//!
//!     let p1 = geom.add_point(0., 0., 0.)?;
//!
//!     Ok(())
//! }
//! ```
//!
//! [^unique]: In most circumstances, tags are a unique identifier. There are some
//! exceptions:
//! * If tags are removed from a model, they can be used again for other shapes.
//! * One Gmsh context can have many models. It's your responsibility to avoid
//!   using tags from one model in another.
//!

use crate::{Gmsh, GmshError, GmshResult, get_cstring};

use std::os::raw::c_int;
use std::ffi::{CString, CStr};

use std::ops::Neg;
use std::marker::PhantomData;

// enum Dimension {
//     Point,
//     Curve,
//     Surface,
//     Volume,
// }

// basic geometry shapes
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A point tag. Points are used to build larger shapes. 0D.
pub struct PointTag(i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A curve tag, built from points. The curve type includes straight lines. 1D.
pub struct CurveTag(i32);

/// Curves have a direction from start to end.
impl Neg for CurveTag {
    type Output = CurveTag;

    /// Reverse the curve's direction.
    fn neg(self) -> CurveTag {
        match self {
            CurveTag(i) => CurveTag(-i)
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A wire tag. Wires are built from curves. Wires are a path of multiple curves. 1.5D.
pub struct WireTag(i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A surface tag. Surfaces are built from closed wires. 2D.
pub struct SurfaceTag(i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A shell tag. Shells are built from surface loops. 2.5D.
pub struct ShellTag(i32);
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// A volume tag. Volumes are built from closed shells. 3D.
pub struct VolumeTag(i32);

/// A trait for the different tags used by Gmsh.
trait GmshTag {
    /// The raw tag integer passed to the Gmsh library.
    fn to_raw(&self) -> i32;
}

impl GmshTag for PointTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

impl GmshTag for CurveTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

impl GmshTag for WireTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

impl GmshTag for SurfaceTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

// idea of a geometry group => gmsh operations can be on multiple known types,
// => use an enum to group those types

#[derive(Debug, Copy, Clone)]
enum BasicGeometry {
    Point(PointTag),
    Curve(CurveTag),
    // Wire(WireTag),
    Surface(SurfaceTag),
    // Shell(ShellTag),
    Volume(VolumeTag),
}

impl From<PointTag> for BasicGeometry {
    fn from(t: PointTag) -> BasicGeometry {
        BasicGeometry::Point(t)
    }
}

impl From<CurveTag> for BasicGeometry {
    fn from(t: CurveTag) -> BasicGeometry {
        BasicGeometry::Curve(t)
    }
}


#[derive(Debug, Copy, Clone)]
enum CurveOrSurface {
    Curve(CurveTag),
    Surface(SurfaceTag),
}

type c_or_s = CurveOrSurface;

impl From<CurveTag> for c_or_s {
    fn from(t: CurveTag) -> c_or_s {
        CurveOrSurface::Curve(t)
    }
}

impl From<SurfaceTag> for CurveOrSurface {
    fn from(t: SurfaceTag) -> CurveOrSurface {
        CurveOrSurface::Surface(t)
    }
}


/// Associated geometry information.
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct PhysicalGroupTag(i32);

/// The native Gmsh geometry kernel.
pub struct Geo<'a> {
    name: &'static str,
    c_name: CString,
    phantom: PhantomData<&'a Gmsh>,
}

impl<'a> Geo<'a> {

    /// Make a new native geometry kernel named `name`.
    // todo: fix me for setting which model is the current one.
    // idea: keep a list of already used model names and only allow one at once
    #[must_use]
    pub fn new(_: &'a Gmsh, name: &'static str) -> GmshResult<Geo<'a>> {
        let c_name = get_cstring(name)?;
        unsafe {
            let mut ierr: c_int = 0;
            // also sets the added model as the current model
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok( Geo { name, c_name, phantom: PhantomData, } ),
                -1 => Err(GmshError::Initialization),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Set this model to be the current Gmsh model.
    fn set_to_current(&self) -> GmshResult<()> {
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelSetCurrent(self.c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok(()),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Remove this model from the Gmsh context.
    // todo: fix this for multiple models.
    // one name may be shared among many, so this will actually remove the first
    // model named whatever this name is.
    pub fn remove(self) -> GmshResult<()> {
        // first set this model to the current model.
        self.set_to_current()?;
        // now, remove the current model
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelRemove(&mut ierr);
            match ierr {
                0 => Ok(()),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Add a point to the model by specifying its coordinates.
    #[must_use]
    pub fn add_point(&mut self,  x: f64, y: f64, z: f64) -> GmshResult<PointTag> {
        self.set_to_current()?;
        self.add_point_gen((x,y,z), None)
    }

    /// Add a point to the model and specify a target mesh size `lc` there.
    #[must_use]
    pub fn add_point_with_lc(&mut self, x: f64, y: f64, z: f64, lc: f64) -> GmshResult<PointTag> {
        self.set_to_current()?;
        self.add_point_gen((x,y,z), Some(lc))
    }

    #[must_use]
    fn add_point_gen(&mut self,
        coords: (f64, f64, f64),
        mesh_size: Option<f64>,
        //tag: Option<i32>,
    ) -> GmshResult<PointTag> {

        let (x, y, z) = coords;
        let out_tag: i32 = 0;

        let lc = mesh_size.unwrap_or(0.);
        let auto_number = -1;
        // let tag = tag.unwrap_or(-1);

        unsafe {
            let mut ierr: c_int = 0;
            let out_tag = gmsh_sys::gmshModelGeoAddPoint(x, y, z, lc, auto_number, &mut ierr);
            match ierr {
                0 => Ok(PointTag(out_tag)),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }



    /// Delete a point from the Gmsh model.
    // todo: Genericize this for all GeometryTags
    pub fn remove_point(&mut self, p: PointTag) -> GmshResult<()> {
        self.set_to_current()?;

        let raw_tag = p.0;

        unsafe {
            let vec_len = 1;
            let is_recursive = 0;
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelGeoRemove([raw_tag].as_mut_ptr(), vec_len, is_recursive, &mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }

    /// Add a straight line between two points.
    #[must_use]
    pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> GmshResult<CurveTag> {
        self.set_to_current()?;

        let auto_number = -1;
        unsafe {
            let mut ierr: c_int = 0;
            let out_tag = gmsh_sys::gmshModelGeoAddLine(p1.to_raw(), p2.to_raw(), auto_number, &mut ierr);
            match ierr {
                0 => Ok(CurveTag(out_tag)),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }

    /// Add a surface from a set of closed, directed curves.
    #[must_use]
    pub fn add_surface(&mut self, curves: &[CurveTag]) -> GmshResult<SurfaceTag> {
        self.set_to_current()?;
        for CurveTag(i) in curves {
            println!("{:?}", i);
        }
        Ok(SurfaceTag(1))
    }

    // idea for a certain operation that only works for curves and surfaces
    pub fn curve_or_surface_op<T: Into<CurveOrSurface>> (&mut self, gen_entity: T) {
        let entity = gen_entity.into();
        match entity {
            CurveOrSurface::Curve(CurveTag(ct)) => println!("Curve with tag {:?}", ct),
            CurveOrSurface::Surface(SurfaceTag(ct)) => println!("Surface with tag {:?}", ct),
        }
    }

    /// Synchronize the geometry model.
    pub fn synchronize(&self) -> GmshResult<()> {
        self.set_to_current()?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelGeoSynchronize(&mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }

    // probably should move this to a dedicated model class
    // with an inner Option(Mesh) and Option(Geo)
    pub fn generate_mesh(&self, dim: i32) -> GmshResult<()> {
        self.set_to_current()?;
        // synchronize by default?
        self.synchronize()?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelMeshGenerate(dim, &mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }
}
