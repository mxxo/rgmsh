use crate::{model::*, GmshError, GmshResult};
use crate::interface::geo as factory;

include!("common_geo.rs");
// impl<'a> GeoModel<'a> {
//
//     #[must_use]
//     fn add_point_gen(
//          &mut self,
//          coords: (f64, f64, f64),
//          mesh_size: Option<f64>,
//      ) -> GmshResult<PointTag> {
//          self.set_current()?;
//
//          let (x, y, z) = coords;
//
//          let lc = mesh_size.unwrap_or(0.);
//          let auto_number = -1;
//
//          unsafe {
//              let mut ierr: c_int = 0;
//              let out_tag = factory::add_point(x, y, z, lc, auto_number, &mut ierr);
//              check_model_error!(ierr, PointTag(out_tag))
//          }
//      }
//
//
//     /// Add a point to the model by specifying its coordinates.
//     #[must_use]
//     pub fn add_point(&mut self, x: f64, y: f64, z: f64) -> GmshResult<PointTag> {
//         println!("added basic point");
//         self.add_point_gen((x, y, z), None)
//     }
//
//     /// Add a point to the model and specify a target mesh size `lc` there.
//     #[must_use]
//     pub fn add_point_with_lc(&mut self, x: f64, y: f64, z: f64, lc: f64) -> GmshResult<PointTag> {
//         println!("added point with lc");
//         self.add_point_gen((x, y, z), Some(lc))
//     }
//
//
//     /// Add a straight line between two points.
//     #[must_use]
//     pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> GmshResult<CurveTag> {
//         self.set_current()?;
//         let auto_number = -1;
//         unsafe {
//             let mut ierr: c_int = 0;
//             let out_tag = factory::add_line(p1.to_raw(), p2.to_raw(), auto_number, &mut ierr);
//             check_model_error!(ierr, CurveTag(out_tag))
//         }
//     }
//
//
//     /// Add a curve loop from a closed set of curves.
//     #[must_use]
//     pub fn add_curve_loop(&mut self, curves: &[CurveTag]) -> GmshResult<WireTag> {
//         self.set_current()?;
//         let mut raw_tags: Vec<_> = curves.iter().map(|c| c.to_raw()).collect();
//         let auto_number = -1;
//         unsafe {
//             let mut ierr: c_int = 0;
//             let out_tag = factory::add_curve_loop(raw_tags.as_mut_ptr(), raw_tags.len() as usize, auto_number, &mut ierr);
//             check_model_error!(ierr, WireTag(out_tag))
//         }
//     }
//
//     /// Add a surface from a WireTag of a closed curve set.
//     #[must_use]
//     pub fn add_plane_surface(&mut self, boundary: WireTag) -> GmshResult<SurfaceTag> {
//         self.add_plane_surface_gen(&[boundary])
//     }
//
//     /// Add a surface with holes.
//     #[must_use]
//     pub fn add_plane_surface_with_holes(&mut self, boundary: WireTag, holes: &[WireTag]) -> GmshResult<SurfaceTag> {
//         self.add_plane_surface_gen(&[&[boundary], holes].concat())
//     }
//
//     #[doc(hidden)]
//     fn add_plane_surface_gen(&mut self, curves: &[WireTag]) -> GmshResult<SurfaceTag> {
//         self.set_current()?;
//         let mut raw_tags: Vec<_> = curves.iter().map(|c| c.to_raw()).collect();
//         let auto_number = -1;
//         unsafe {
//             let mut ierr: c_int = 0;
//             let out_tag = factory::add_plane_surface(raw_tags.as_mut_ptr(), raw_tags.len() as usize, auto_number, &mut ierr);
//             check_model_error!(ierr, SurfaceTag(out_tag))
//         }
//     }
//
//
// }
