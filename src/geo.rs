/// Gmsh native geometry kernel
use crate::{Gmsh, GmshError, ModelError, GmshResult, ModelResult, get_CString};

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

// #[derive(Debug, Copy, Clone)]
// enum GeometryTag {
//     Point(i32),
//     Curve(i32),
//     Wire(i32),
//     Surface(i32),
//     Shell(i32),
//     Volume(i32),
// }
//
// impl GmshTag for GeometryTag {
//     fn to_raw(&self) -> i32 {
//         match self {
//             GeometryTag::Point(i) => *i,
//             GeometryTag::Curve(i) => *i,
//             GeometryTag::Wire(i) => *i,
//             GeometryTag::Surface(i) => *i,
//             GeometryTag::Shell(i) => *i,
//             GeometryTag::Volume(i) => *i,
//         }
//     }
// }

// basic geometry shapes
#[derive(Debug, Copy, Clone)]
pub struct PointTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct CurveTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct WireTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct SurfaceTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct ShellTag(i32);
#[derive(Debug, Copy, Clone)]
pub struct VolumeTag(i32);

trait GmshTag {
    fn to_raw(&self) -> i32;
}

impl GmshTag for PointTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

impl GmshTag for CurveTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

impl GmshTag for WireTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

impl GmshTag for SurfaceTag {
    fn to_raw(&self) -> i32 {
        self.0
    }
}

// idea of a geometry group => gmsh operations can be on multiple known types,
// => use an enum to group those types

#[derive(Debug, Copy, Clone)]
enum CurveOrSurface {
    Curve(CurveTag),
    Surface(SurfaceTag),
}

type c_or_s = CurveOrSurface;

impl From<CurveTag> for c_or_s {
    fn from(ct: CurveTag) -> c_or_s {
        CurveOrSurface::Curve(ct)
    }
}

impl From<SurfaceTag> for CurveOrSurface {
    fn from(ct: SurfaceTag) -> CurveOrSurface {
        CurveOrSurface::Surface(ct)
    }
}

// idea for a certain operation that only works for curves and surfaces
pub fn curve_or_surface_op<T: Into<CurveOrSurface>> (gen_entity: T) {
    let entity = gen_entity.into();
    match entity {
        CurveOrSurface::Curve(CurveTag(ct)) => println!("Curve with tag {:?}", ct),
        CurveOrSurface::Surface(SurfaceTag(ct)) => println!("Surface with tag {:?}", ct),
    }
}

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

// associated geometry information
struct PhysicalGroupTag(i32);

// type aliases for vector methods
type points = Vec<PointTag>;

pub struct Geo<'a> {
    name: &'static str,
    phantom: PhantomData<&'a Gmsh>,
}

impl<'a> Geo<'a> {

    #[must_use]
    pub fn new(_: &'a Gmsh, name: &'static str) -> GmshResult<Geo<'a>> {
        let c_name = get_CString(name)?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok( Geo { name, phantom: PhantomData, } ),
                -1 => Err(GmshError::Initialization),
                _ => Err(GmshError::from(ModelError::Unknown)),
            }
        }
    }

    #[must_use]
    pub fn add_point(&mut self, coords: (f64, f64, f64)) -> ModelResult<PointTag> {
        self.add_point_gen(coords, None)
    }

    #[must_use]
    pub fn add_point_with_lc(&mut self, coords: (f64, f64, f64), lc: f64) -> ModelResult<PointTag> {
        self.add_point_gen(coords, Some(lc))
    }

    #[must_use]
    fn add_point_gen(&mut self,
        coords: (f64, f64, f64),
        mesh_size: Option<f64>,
        //tag: Option<i32>,
    ) -> ModelResult<PointTag> {

        let (x, y, z) = coords;
        let out_tag: i32 = 0;

        let lc = mesh_size.unwrap_or(0.);
        let auto_number = -1;
        // let tag = tag.unwrap_or(-1);

        unsafe {
            let mut ierr: c_int = 0;
            let out_tag = gmsh_sys::gmshModelGeoAddPoint(x, y, z, lc, auto_number, &mut ierr);
            match ierr {
                0 => Ok(PointTag(out_tag)),
                -1 => Err(ModelError::Initialization),
                1  => Err(ModelError::Mutation),
                2  => Err(ModelError::Lookup),
                3  => Err(ModelError::BadInput),
                4  => Err(ModelError::MeshQuery),
                _  => Err(ModelError::Unknown),
            }
        }
    }

    // delete a point from the Gmsh model.
    // Genericize this for all GeometryTags
    pub fn remove_point(&mut self, p: PointTag) -> ModelResult<()> {

        let raw_tag = p.0;

        unsafe {
            let vec_len = 1;
            let is_recursive = 0;
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelGeoRemove([raw_tag].as_mut_ptr(), vec_len, is_recursive, &mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(ModelError::Initialization),
                1  => Err(ModelError::Mutation),
                2  => Err(ModelError::Lookup),
                3  => Err(ModelError::BadInput),
                4  => Err(ModelError::MeshQuery),
                _  => Err(ModelError::Unknown),
            }
        }
    }

    // add a straight line between two points
    #[must_use]
    pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> ModelResult<CurveTag> {
        let auto_number = -1;
        unsafe {
            let mut ierr: c_int = 0;
            let out_tag = gmsh_sys::gmshModelGeoAddLine(p1.to_raw(), p2.to_raw(), auto_number, &mut ierr);
            match ierr {
                0 => Ok(CurveTag(out_tag)),
                -1 => Err(ModelError::Initialization),
                1  => Err(ModelError::Mutation),
                2  => Err(ModelError::Lookup),
                3  => Err(ModelError::BadInput),
                4  => Err(ModelError::MeshQuery),
                _  => Err(ModelError::Unknown),
            }
        }
    }

    #[must_use]
    pub fn add_surface(&mut self, curves: &[CurveTag]) -> ModelResult<SurfaceTag> {
        for CurveTag(i) in curves {
            println!("{:?}", i);
        }
        Ok(SurfaceTag(1))
    }

    pub fn synchronize(&self) -> ModelResult<()> {
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelGeoSynchronize(&mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(ModelError::Initialization),
                1  => Err(ModelError::Mutation),
                2  => Err(ModelError::Lookup),
                3  => Err(ModelError::BadInput),
                4  => Err(ModelError::MeshQuery),
                _  => Err(ModelError::Unknown),
            }
        }
    }

    // probably should move this to a dedicated model class
    // with an inner Option(Mesh) and Option(Geo)
    pub fn generate_mesh(&self, dim: i32) -> ModelResult<()> {
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelMeshGenerate(dim, &mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(ModelError::Initialization),
                1  => Err(ModelError::Mutation),
                2  => Err(ModelError::Lookup),
                3  => Err(ModelError::BadInput),
                4  => Err(ModelError::MeshQuery),
                _  => Err(ModelError::Unknown),
            }
        }
    }

}
