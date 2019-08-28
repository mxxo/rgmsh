/// Gmsh native geometry kernel
use crate::{Gmsh, GmshError, ModelError};

use std::os::raw::c_int;
use std::ffi::{CString, CStr};

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
pub struct PointTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct CurveTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct SurfaceTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct VolumeTag(i32);

pub struct xyz(f64, f64, f64);

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
struct PhysicalGroupTag(i32);

// type aliases for vector methods
type points = Vec<PointTag>;

pub struct Geo<'a> {
    name: &'static str,
    phantom: PhantomData<&'a Gmsh>,
}

type ModelResult<T> = Result<T, ModelError>;

impl<'a> Geo<'a> {

    pub fn new(_: &'a Gmsh, name: &'static str) -> ModelResult<Geo<'a>> {
        let c_name: CString = match CString::new(String::from(name)) {
            Ok(c_name) => c_name,
            Err(err) => return Err(ModelError::Initialization),
        };
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok( Geo { name, phantom: PhantomData, } ),
                -1 => Err(ModelError::Initialization),
                _ => Err(ModelError::Mutation),
            }
        }
    }

    pub fn add_point(&mut self,
        coords: (f64, f64, f64),
        mesh_size: Option<f64>,
        tag: Option<i32>,
    ) -> ModelResult<PointTag> {

        let (x, y, z) = coords;
        let out_tag: i32 = 0;

        let lc = mesh_size.unwrap_or(0.);
        let tag = tag.unwrap_or(-1);
        let mut ierr: c_int = 0;

        unsafe {
            let out_tag = gmsh_sys::gmshModelGeoAddPoint(x, y, z, lc, tag, &mut ierr);
            match ierr {
                0 => Ok(PointTag(out_tag)),
                -1 => Err(ModelError::Initialization),
                1  => Err(ModelError::Mutation),
                2  => Err(ModelError::Lookup),
                3  => Err(ModelError::BadInput),
                4  => Err(ModelError::MeshQuery),
                _  => panic!()
            }
        }
    }

    // delete a point from the Gmsh model.
    pub fn remove_point(&mut self, p: PointTag) {

    }

    // add a straight line between two points
    pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> ModelResult<CurveTag> {
        Ok(CurveTag(1))
    }

}
