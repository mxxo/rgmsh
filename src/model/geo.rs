//! The built-in geometry kernel

use super::*;

/// An instance of the built-in kernel
pub struct Geo<'a> {
    name: &'static str,
    c_name: CString,
    phantom: PhantomData<&'a Gmsh>,
}

impl<'a> Geo<'a> {

    /// Make a new native geometry kernel.
    // todo: fix me for setting which model is the current one.
    // idea: keep a list of already used model names and only allow one at once
    #[must_use]
    pub fn new(_: &'a Gmsh, name: &'static str) -> GmshResult<Geo<'a>> {
        let c_name = get_cstring(name)?;
        unsafe {
            let mut ierr: c_int = 0;
            // also sets the added model as the current model
            gmsh_sys::gmshModelAdd(c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok( Geo { name, c_name, phantom: PhantomData, } ),
                -1 => Err(GmshError::Initialization),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Set this model to be the current Gmsh model.
    fn set_to_current(&self) -> GmshResult<()> {
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelSetCurrent(self.c_name.as_ptr(), &mut ierr);
            match ierr {
                0 => Ok(()),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Remove this model from the Gmsh context.
    // todo: fix this for multiple models.
    // one name may be shared among many, so this will actually remove the first
    // model named whatever this name is.
    pub fn remove(self) -> GmshResult<()> {
        // first set this model to the current model.
        self.set_to_current()?;
        // now, remove the current model
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelRemove(&mut ierr);
            match ierr {
                0 => Ok(()),
                _ => Err(GmshError::Execution),
            }
        }
    }

    /// Add a point to the model by specifying its coordinates.
    #[must_use]
    pub fn add_point(&mut self,  x: f64, y: f64, z: f64) -> GmshResult<PointTag> {
        self.set_to_current()?;
        self.add_point_gen((x,y,z), None)
    }

    /// Add a point to the model and specify a target mesh size `lc` there.
    #[must_use]
    pub fn add_point_with_lc(&mut self, x: f64, y: f64, z: f64, lc: f64) -> GmshResult<PointTag> {
        self.set_to_current()?;
        self.add_point_gen((x,y,z), Some(lc))
    }

    #[must_use]
    fn add_point_gen(&mut self,
        coords: (f64, f64, f64),
        mesh_size: Option<f64>,
        //tag: Option<i32>,
    ) -> GmshResult<PointTag> {

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
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }



    /// Delete a point from the Gmsh model.
    // todo: Genericize this for all GeometryTags
    pub fn remove_point(&mut self, p: PointTag) -> GmshResult<()> {
        self.set_to_current()?;

        let raw_tag = p.0;

        unsafe {
            let vec_len = 1;
            let is_recursive = 0;
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelGeoRemove([raw_tag].as_mut_ptr(), vec_len, is_recursive, &mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }

    /// Add a straight line between two points.
    #[must_use]
    pub fn add_line(&mut self, p1: PointTag, p2: PointTag) -> GmshResult<CurveTag> {
        self.set_to_current()?;

        let auto_number = -1;
        unsafe {
            let mut ierr: c_int = 0;
            let out_tag = gmsh_sys::gmshModelGeoAddLine(p1.to_raw(), p2.to_raw(), auto_number, &mut ierr);
            match ierr {
                0 => Ok(CurveTag(out_tag)),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }

    /// Add a surface from a set of closed, directed curves.
    #[must_use]
    pub fn add_surface(&mut self, curves: &[CurveTag]) -> GmshResult<SurfaceTag> {
        self.set_to_current()?;
        for CurveTag(i) in curves {
            println!("{:?}", i);
        }
        Ok(SurfaceTag(1))
    }

    // idea for a certain operation that only works for curves and surfaces
    pub fn curve_or_surface_op<T: Into<CurveOrSurface>> (&mut self, gen_entity: T) {
        let entity = gen_entity.into();
        match entity {
            CurveOrSurface::Curve(CurveTag(ct)) => println!("Curve with tag {:?}", ct),
            CurveOrSurface::Surface(SurfaceTag(ct)) => println!("Surface with tag {:?}", ct),
        }
    }

    /// Synchronize the geometry model.
    pub fn synchronize(&self) -> GmshResult<()> {
        self.set_to_current()?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelGeoSynchronize(&mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }

    // probably should move this to a dedicated model class
    // with an inner Option(Mesh) and Option(Geo)
    pub fn generate_mesh(&self, dim: i32) -> GmshResult<()> {
        self.set_to_current()?;
        // synchronize by default?
        self.synchronize()?;
        unsafe {
            let mut ierr: c_int = 0;
            gmsh_sys::gmshModelMeshGenerate(dim, &mut ierr);
            match ierr {
                0 => Ok(()),
                -1 => Err(GmshError::Initialization),
                1  => Err(GmshError::ModelMutation),
                2  => Err(GmshError::ModelLookup),
                3  => Err(GmshError::ModelBadInput),
                4  => Err(GmshError::ModelParallelMeshQuery),
                _  => Err(GmshError::Execution),
            }
        }
    }
}
