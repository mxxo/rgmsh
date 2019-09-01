/// This file reimplements gmsh/tutorial/t1.geo in Rust

extern crate gmsh;
use gmsh::{Gmsh, GmshResult};

use gmsh::geo::PointTag;
// use gmsh::geo::curve_or_surface_op;

use std::env;

fn main() -> GmshResult<()> {

    // spin up Gmsh
    let mut gmsh = Gmsh::initialize()?;

    // ask for a new native geometry instance
    let mut geom = gmsh.new_native_model("hal")?;
    let p: PointTag = geom.add_point(0., 0., 0.)?;
    println!("{:?}", p);

    // ask for another model
    let mut geom2 = gmsh.new_native_model("bella")?;
    let p2: PointTag = geom2.add_point(0., 0., 0.)?;
    println!("{:?}", p2);

    // only way to get PointTags is through geometry construction methods

    // won't compile
    //let p = PointTag(1);

    // Destructor doesn't take ownership, so you can use a PointTag after removing it
    // This will remove it in the internal Gmsh model however.
    geom.remove_point(p);

    // If you remove a point (line, surface, volume), you are in charge of making
    // sure you don't use that tag later on

    // will compile
    println!("Point tag from deleted model is {:?}", p);

    // To make a line, you need at least two points
    let p1 = geom.add_point(0., 0., 0.)?;
    let p2 = geom.add_point(1., 1., 0.)?;

    println!("p1 = {:?}", p1);
    println!("p2 = {:?}", p2);

    let line = geom.add_line(p1, p2)?;
    println!("{:?}", line);

    // You can't use LineTags (SurfaceTags, VolumeTags,...) for PointTag methods
    // won't compile
    //geom.remove_point(line);

    // most gmsh operations are limited to a group of specific geometry types

    // this next operation only works on curves and lines, so calling it with a
    // point tag won't compile.
    // curve_or_surface_op(p1);

    // either curves or surfaces are both fine though, without any obligation on
    // the caller for an explicit conversion into the right type that we
    // use behind the scenes

    geom.curve_or_surface_op(line);

    let l1 = geom.add_line(p1, p2)?;
    let l2 = geom.add_line(p1, p2)?;
    let l3 = geom.add_line(p1, p2)?;
    let l4 = geom.add_line(p1, p2)?;

    let s = geom.add_surface(&[l1, -l2, l3, l4])?;
    geom.curve_or_surface_op(s);

    // lines (curves) have a direction, from start to end.
    // you can reverse that direction of a given CurveTag using a negative sign.
    // This is useful for making line loops, because Gmsh requires a
    // directed path for line loops
    let rev_l = -line;
    println!("{:?}", rev_l);

    // let ll = geom.add_curve_loop(1, -2, 3, 4);

    // when you're ready to mesh, make sure the geometry kernel is synchronized
    // geom.synchronize()?;

    // ? you'll get a handle to a new mesh object
    geom.generate_mesh(1)?;

    // You could also get around the safety checks by using PointTags from one geometry
    // on another, but why would you do that ;)?

    // Ok(())

    // models can't be used after their context is dropped
    // won't compile
    // std::mem::drop(gmsh);
    // geom.generate_mesh(2);

    // but this is fine
    geom.generate_mesh(2);
    std::mem::drop(gmsh);

    Ok(())

    // Gmsh context is automatically dropped here, no more gmsh::finalize
}
