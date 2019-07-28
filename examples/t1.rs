/// This file reimplements gmsh/tutorial/t1.geo in Rust
extern crate gmsh;
//use gmsh::geo;
use gmsh::Gmsh;

use std::env;
use std::io;
use std::io::prelude::*;

fn main() -> io::Result<()> {
    let read_configs = true;

    {
        let mut g = Gmsh::initialize(env::args(), read_configs)?;
        g.add_point(1.0, 1.0, 1.0, None, None);
    }

    //Gmsh::add_point(1.0, 1.0, 1.0, None, None);

    // g.addPoint(1.0, 1.0, 1.0, None, None);

    Ok(())
}
