/// Gmsh native geometry kernel
use crate::Gmsh;
use std::ops::Neg;

enum Dimension {
    Point,
    Curve,
    Surface,
    Volume,
}

// basic geometry shapes
#[derive(Debug, Copy, Clone)]
pub struct PointTag(i64);
#[derive(Debug, Copy, Clone)]
pub struct CurveTag(i64);
#[derive(Debug, Copy, Clone)]
pub struct SurfaceTag(i64);
#[derive(Debug, Copy, Clone)]
pub struct VolumeTag(i64);

// curves have a direction and can be reversed
impl Neg for CurveTag {
    type Output = CurveTag;

    fn neg(self) -> CurveTag {
        match self {
            CurveTag(i) => CurveTag(-i)
        }
    }
}

impl PointTag {
    // no public constructor, so only our methods can make valid point tags
}

// associated geometry information
struct PhysicalGroupTag(i64);

// type aliases for vector methods
type points = Vec<PointTag>;

pub struct Geo {
    name: &'static str,
}

impl Geo {

    pub fn new(gmsh: &mut Gmsh, name: &'static str) -> Geo {
        Geo {
            name
        }
    }

    pub fn add_point(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        mesh_size: Option<f64>,
        tag: Option<i32>,
    ) -> PointTag {
        PointTag(1)
    }

    // take ownership of the PointTag
    pub fn remove_point(&mut self, p: PointTag) {

    }
}
