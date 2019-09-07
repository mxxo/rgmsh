/// This file reimplements gmsh/tutorial/t1.geo in Rust
///
/// Elementary entities and physical groups

extern crate gmsh;
use gmsh::{Gmsh, GmshResult, model::{GeoKernel, PointTag, CurveTag}};

// main function returns a Gmsh result for error handling
fn main() -> GmshResult<()> {

    // spin up Gmsh
    let mut gmsh = Gmsh::initialize()?;

    // make a new model
    let mut model = gmsh.create_native_model("t1")?;

    // you can specify a target mesh size, or "characteristic length"
    // at a point
    let lc = 1e-2;

    // Point tags are a dedicated type and are assigned automatically
    let p1: PointTag = model.add_point_with_lc(0., 0., 0., lc)?;
    let p2 = model.add_point_with_lc(0.1, 0.,  0., lc)?;
    let p3 = model.add_point_with_lc(0.1, 0.3, 0., lc)?;
    let p4 = model.add_point_with_lc(0., 0.3, 0., lc)?;

    // Curves can be built up from points
    let l1: CurveTag = model.add_line(p1, p2)?;
    let l2 = model.add_line(p3, p2)?;
    let l3 = model.add_line(p3, p4)?;
    let l4 = model.add_line(p4, p1)?;

    let cl = model.add_curve_loop(&[l4, l1, -l2, l3])?;
    let pl = model.add_plane_surface(cl)?;

    //let physical_curve = model.add_physical_group(&[l1, l2, l4])?;
    //let physical_surf = model.add_physical_group(&[pl])?;

    model.generate_mesh(3)?;

    // or, we could use the OpenCASCADE kernel and define the shape directly
    // let mut occ_model = gmsh.create_occ_model("t1_occ")?;
    // occ_model.add_rectangle((.2, 0., 0.), (0.1, 0.3, 0.))?;

    // show the GUI
    // gmsh.gui.show();

    Ok(())

    // Gmsh context is automatically dropped here, no more gmsh::finalize
}
