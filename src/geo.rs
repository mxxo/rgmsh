/// Gmsh native geometry model crate
use crate::Gmsh;

pub struct Geo;

impl Geo {
    pub fn add_point(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        mesh_size: Option<f64>,
        tag: Option<i32>,
    ) -> i32 {
        unimplemented!();
    }
}
