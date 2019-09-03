//! Gmsh geometry models.
//!
//! There are two CAD engines you can use:
//! 1. The built-in Gmsh geometry kernel.
//! 2. The OpenCASCADE geometry kernel.
//!
//! The relevant [Gmsh manual section](http://gmsh.info/doc/texinfo/gmsh.html#Geometry-module)
//! explains the differences between the two kernels:
//!
//! > The built-in CAD kernel provides a simple CAD engine based on a bottom-up boundary representation approach:
//! > you need to first define points, then curves, then surfaces and finally volumes.
//!
//! > The OpenCASCADE kernel allows one to build models in the same bottom-up manner, or by using a
//! > constructive solid geometry approach where solids are defined first.
//! > Boolean operations can then be performed to modify them.
//!
//! Either kernel should suffice for most projects.
//!
//! OpenCASCADE is a widely-used CAD engine, so it's a good default choice. You can directly define larger shapes without making their smaller components first.
//! You also get access to powerful Boolean geometry operations for making complex shapes.
//!
//! The only way to get a model is through a `Gmsh` context object.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # fn main() -> GmshResult<()> {
//! let gmsh = Gmsh::initialize()?;
//! let mut geom = gmsh.new_native_model("model")?;
//! # Ok(())
//! # }
//! ```
//!
//! The model is only valid for the lifetime of `Gmsh`.
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
//! // try to use the model afterwards
//! geom.add_point(0., 0., 0.)?; // won't compile
//! # Ok(())
//! # }
//! ```
//!
//! ## Create, modify and delete shapes
//! You can define points, lines, 2D surfaces and 3D volumes.
//! After defining a shape, you'll get a geometry tag to identify[^unique] it.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # use gmsh::model::{PointTag, CurveTag};
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
//! # use gmsh::model::{PointTag, CurveTag};
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
//! implementations. For example, using the `C++` API, the following example will
//! compile but cause a runtime error.
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
//! It's your responsibility to make sure tags are used with the right model.
//!
//! If you're lucky, using the wrong tags will cause a runtime error.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # use std::result::Result;
//! # fn main() -> GmshResult<()> {
//! #  let gmsh = Gmsh::initialize()?;
//! let mut geom_a = gmsh.new_native_model("jimbo")?;
//! let mut geom_b = gmsh.new_native_model("aircraft-carrier")?;
//!
//! let p_a = geom_a.add_point(0., 0., 0.)?;
//!
//! let p_b1 = geom_b.add_point(0., 1., 0.)?;
//! let p_b2 = geom_b.add_point(1., 1., 0.)?;
//!
//! // points from different models can have the same value
//! assert!(p_a == p_b1, "Point tags are different!");
//!
//! // Bad! Using tags from one model with another.
//! let line = geom_a.add_line(p_b1, p_b2);
//! assert!(line.is_err());
//! #  Ok(())
//! # }
//! ```
//!
//! If you're unlucky, the tags will exist in both models, causing a silent logic error in your program.
//! In the API's eyes, you've given it valid tags, and it's going to go ahead and do what you asked for.
//! ```
//! # use gmsh::{Gmsh, GmshResult};
//! # use std::result::Result;
//! # fn main() -> GmshResult<()> {
//! #  let gmsh = Gmsh::initialize()?;
//! let mut geom_a = gmsh.new_native_model("jimbo")?;
//! let p_a1 = geom_a.add_point(0., 0., 0.)?;
//! let p_a2 = geom_a.add_point(1., 0., 0.)?;
//!
//! let mut geom_b = gmsh.new_native_model("aircraft-carrier")?;
//! let p_b1 = geom_b.add_point(0., 1., 1.)?;
//! let p_b2 = geom_b.add_point(0., 1., 1.)?;
//!
//! // Very bad! A silent logic error. You're on your own debugging this one.
//! let line = geom_a.add_line(p_b1, p_b2);
//! assert!(line.is_ok());
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

pub mod geo;
pub mod occ;

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

impl From<PointTag> for BasicShape {
    fn from(t: PointTag) -> BasicShape {
        BasicShape::Point(t)
    }
}

impl From<CurveTag> for BasicShape {
    fn from(t: CurveTag) -> BasicShape {
        BasicShape::Curve(t)
    }
}


/// Private module for sets of geometries passed and returned from functions.
///
/// Gmsh operations can be on multiple known types. We use enums for a compile-time
/// check that the type is OK to use with that function.
mod geometry_groups {
    use super::*;

    #[derive(Debug, Copy, Clone)]
    /// The basic geometry types (points, curves, surfaces, and volumes).
    pub enum BasicShape {
        Point(PointTag),
        Curve(CurveTag),
        Surface(SurfaceTag),
        Volume(VolumeTag),
    }

    #[derive(Debug, Copy, Clone)]
    /// The full set of geometry types (`BasicGeometries` + wires + shells).
    pub enum GeneralShape {
        Point(PointTag),
        Curve(CurveTag),
        Wire(WireTag),
        Surface(SurfaceTag),
        Shell(ShellTag),
        Volume(VolumeTag),
    }

    #[derive(Debug, Copy, Clone)]
    /// Only curves or surfaces.
    pub enum CurveOrSurface {
        Curve(CurveTag),
        Surface(SurfaceTag),
    }
}

use geometry_groups::BasicShape;
use geometry_groups::CurveOrSurface;

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

