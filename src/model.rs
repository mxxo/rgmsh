/// Gmsh model crate
use crate::Gmsh;

pub struct Model {
    name: &'static str,
    pub kernel: KernelType,
}

pub enum KernelType {
    Native(NativeKernel),
    OCC(OCCKernel),
}

impl Model {
    pub fn new(name: &'static str, kernel: KernelType) -> Model {

        // gmsh_sys::gmshModelAdd()

        Model {
            name,
            kernel,
        }
    }
}

//pub trait GeometryKernel {
//    fn add_point(
//        &mut self,
//        x: f64,
//        y: f64,
//        z: f64,
//        mesh_size: Option<f64>,
//        tag: Option<i32>,
//    ) -> i32 {
//        unimplemented!();
//    }
//}


pub struct NativeKernel {}
pub struct OCCKernel {}


impl OCCKernel {

    pub fn test() {}

    pub fn addDisk(&mut self, xc: f64, yc: f64, zc: f64, rx: f64, ry: f64, tag: usize) {
       unimplemented!();
    }

    pub fn add_point(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        mesh_size: Option<f64>,
        tag: Option<i32>,
    ) -> i32 {
        println!("occ adding point");
        unimplemented!();
    }

}

//impl GeometryKernel for OCCKernel {
//    fn add_point(
//        &mut self,
//        x: f64,
//        y: f64,
//        z: f64,
//        mesh_size: Option<f64>,
//        tag: Option<i32>,
//    ) -> i32 {
//        println!("occ adding point");
//       unimplemented!();
//    }
//
//}
