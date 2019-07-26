/// This file reimplements gmsh/tutorial/t1.geo in Rust

extern crate gmsh;
use std::env;
use std::io::prelude::*;
use std::io;

fn main() -> io::Result<()> {

    let read_configs = true;

    // initialize the Gmsh instance
    gmsh::initialize(env::args(), read_configs)?;

    // close Gmsh
    gmsh::finalize()?;

    Ok(())
}
