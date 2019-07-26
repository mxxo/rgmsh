extern crate gmsh_sys;

use std::env::Args;
use std::io::prelude::*;
use std::io;

pub fn initialize(args: Args, read_configs: bool) -> io::Result<()> {
    println!("opening Gmsh...");
    Ok(())
}

pub fn finalize() -> io::Result<()> {
    println!("finalizing Gmsh...");
    Ok(())
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
