/// The different error types

// used by:
// top level gmsh functions like gmsh::open, gmsh::merge, gmsh::finalize,
// gmsh::logger
// gmsh::onelab (?)
// gmsh::graphics (?)
// gmsh::fltk (?)
#[derive(Debug)]
pub enum GmshError {
    Initialization, // -1: Gmsh wasn't initialized, or a required library component is missing
    CInterface, // Problems from the Rust/C FFI interface
    GmshModelError(ModelError),
    GmshOptionError(OptionError),
    Execution, // 1: Gmsh couldn't perform the requested operation successfully (e.g. a bad file path was passed in)
}

#[derive(Debug)]
// Used in:
// gmsh::model (model::geo, model::occ, model::mesh, model::geo::mesh)
// gmsh::view
pub enum ModelError {
    Initialization, // -1: Gmsh wasn't initialized, or a required library component is missing
    Mutation,       // 1: a function that mutates the model couldn't complete successfully (e.g. addPoint couldn't succeed because of a tag collision)
    Lookup,         // 2: a data lookup getter function failed, e.g. tried to work on a view that doesn't exist
    BadInput,       // 3: The function couldn't successfully use a required input parameter (rare) (e.g. A requested quadrature scheme couldn't be applied to the data)
    MeshQuery,      // 4: A parallelizable mesh query function failed (rare)
    Unknown,        // ?: Any other error number (I couldn't find any others, but they must be out there)
}

impl From<ModelError> for GmshError {
    fn from(err: ModelError) -> GmshError {
        GmshError::GmshModelError(err)
    }
}

// used by gmsh::option
#[derive(Debug)]
pub enum OptionError {
    Initialization,  // -1: Gmsh wasn't initialized, or a required library component is missing
    UnknownOption,   // 1: The given option wasn't successfully processd, perhaps it doesn't exist
}

impl From<OptionError> for GmshError {
    fn from(err: OptionError) -> GmshError {
        GmshError::GmshOptionError(err)
    }
}

