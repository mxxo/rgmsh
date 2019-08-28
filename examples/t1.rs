/// This file reimplements gmsh/tutorial/t1.geo in Rust

extern crate gmsh;
use gmsh::Gmsh;

use gmsh::geo::Geo;
use gmsh::geo::PointTag;

use std::env;
use std::io;

fn main() -> io::Result<()> {
    let read_configs = true;

    // spin up Gmsh
    let mut g = Gmsh::initialize(env::args(), read_configs)?;
    // ask for a new native geometry instance
    let mut geom = g.new_native_model("hal");

    // only way to get PointTags is through geometry construction methods
    let p: PointTag = geom.add_point(0., 0., 0., None, None).unwrap();

    // won't compile
    // let p = PointTag(1);

    println!("{:?}", p);

    // Destructor doesn't take ownership, so you can use a PointTag after removing it
    // This will remove it in the internal Gmsh model however.
    geom.remove_point(p);

    // If you remove a point (line, surface, volume), you are in charge of making
    // sure you don't use that tag later on

    // will compile
    println!("{:?}", p);

    // You could also get around the safety checks by using PointTags from one geometry
    // on another, but why would you do that ;)?

    // To make a line, you need at least two points
    let p1 = geom.add_point(0., 0., 0., None, None).unwrap();
    let p2 = geom.add_point(1., 1., 0., None, None).unwrap();

    let l = geom.add_line(p1, p2);
    println!("{:?}", l);

    // lines (curves) have a direction, from start to end.
    // you can reverse that direction of a given CurveTag using a negative sign.
    // This is useful for making line loops, because Gmsh requires a
    // directed path for line loops
    let rev_l = -l;
    println!("{:?}", rev_l);

    // let ll = geom.add_curve_loop(1, -2, 3, 4);

    Ok(())

    // Gmsh context is dropped here, no more gmsh::finalize
}
