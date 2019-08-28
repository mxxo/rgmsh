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
    let p: PointTag = geom.add_point(0., 0., 0., None, None);

    // won't compile
    // let p = PointTag(1);

    println!("{:?}", p);

    // destructor takes ownership, so you can't use a PointTag after removing it
    geom.remove_point(p);

    // won't compile
    // println!("{:?}", p);

    // You could get around the safety checks by using PointTags from one geometry
    // on another, but why would you do that ;)?

    Ok(())

    // Gmsh context is dropped here, no more gmsh::finalize
}
