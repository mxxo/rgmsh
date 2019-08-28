/// Gmsh native geometry model crate
use crate::Gmsh;

enum Dimension {
    Point,
    Curve,
    Surface,
    Volume,
}

// all the different kinds of tags
// basic geometry shapes
#[derive(Debug)]
pub struct PointTag(i64);
pub struct CurveTag(i64);
pub struct SurfaceTag(i64);
pub struct VolumeTag(i64);

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
