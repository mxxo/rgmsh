/// Gmsh model crate

pub enum GeometryKernel {
    BuiltIn,
    OpenCascade,
}

pub struct Model{
    name: String
    kernel: GeometryKernel,
}

