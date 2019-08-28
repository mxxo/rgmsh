/// Gmsh native geometry kernel
use crate::{Gmsh, GmshError, ModelError, OptionError};

use std::ops::Neg;
use std::marker::PhantomData;

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

pub struct Geo<'a> {
    name: &'static str,
    phantom: PhantomData<&'a Gmsh>,
}

impl<'a> Geo<'a> {

    pub fn new(_: &'a Gmsh, name: &'static str) -> Result<Geo<'a>, ModelError> {
        Ok(
            Geo {
                name,
                phantom: PhantomData,
            }
        )
    }

    pub fn add_point(
        &mut self,
        x: f64,
        y: f64,
        z: f64,
        mesh_size: Option<f64>,
        tag: Option<i32>,
    ) -> Result<PointTag, ModelError> {
        Ok(PointTag(1))
    }

    // delete a point from the Gmsh model.
    pub fn remove_point(&mut self, p: PointTag) {

    }

    // add a straight line between two points
    pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> CurveTag {
        CurveTag(1)
    }

}
