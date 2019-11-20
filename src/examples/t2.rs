//! Geometry transformations, extrusions and physical groups.
//!
//! This file reimplements `gmsh/tutorial/t2.geo` in Rust.
//!
//! ```
//! extern crate rgmsh;
//! use rgmsh::{Gmsh, GmshResult, model::{PointTag, CurveTag}};
//!
//! fn main() -> GmshResult<()> {
//!
//!     // TODO FIXME for command line args
//!     let mut gmsh = Gmsh::initialize()?;
//!     let mut model = gmsh.create_native_model("t2")?;
//!
//!     // copied from t1.rs...
//!     let lc = 1e-2;
//!     let p1 = model.add_point_with_lc(0., 0., 0., lc)?;
//!     let p2 = model.add_point_with_lc(0.1, 0.,  0., lc)?;
//!     let p3 = model.add_point_with_lc(0.1, 0.3, 0., lc)?;
//!     let p4 = model.add_point_with_lc(0., 0.3, 0., lc)?;
//!     let l1 = model.add_line(p1, p2)?;
//!     let l2 = model.add_line(p3, p2)?;
//!     let l3 = model.add_line(p3, p4)?;
//!     let l4 = model.add_line(p4, p1)?;
//!     let cl = model.add_curve_loop(&[l4, l1, -l2, l3])?;
//!     let pl = model.add_plane_surface(cl)?;
//!
//!     //let physical_curve = model.add_physical_group(&[l1, l2, l4])?;
//!     //let physical_surf = model.add_physical_group(&[pl])?;
//!     // ...end of copy
//!
//!     Ok(())
//! }
//! ```
