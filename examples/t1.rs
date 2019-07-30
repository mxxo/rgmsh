/// This file reimplements gmsh/tutorial/t1.geo in Rust
extern crate gmsh;
use gmsh::Gmsh;
use gmsh::model::KernelType::*;
// use gmsh::model::NativeKernel;
// use gmsh::model::OCCKernel;

use std::env;
use std::io;
use std::io::prelude::*;



fn main() -> io::Result<()> {
    let read_configs = true;

    let mut g = Gmsh::initialize(env::args(), read_configs)?;

    g.add_model("hal", Native(NativeKernel{})); // .add_point(1.0, 1.0, 1.0, None, None);
    g.add_model("bella", OCC(OCCKernel{}));

    // g.models[1].kernel.test();

    //Gmsh::add_point(1.0, 1.0, 1.0, None, None);

    // g.addPoint(1.0, 1.0, 1.0, None, None);

    Ok(())
}
