//! The Gmsh different error types

#[derive(Debug)]
pub enum GmshError {
    Initialization, // -1: Gmsh wasn't initialized, or a required library component is missing
    Execution, // 1 in top-level Gmsh: Gmsh couldn't perform the requested operation successfully (e.g. a bad file path was passed in)
    CInterface, // Problems from the Rust/C FFI interface
    ModelMutation,  // 1 in a model | a function that mutates the model couldn't complete successfully (e.g. addPoint couldn't succeed because of a tag collision)
    ModelLookup,    // 2 in a model | a data lookup getter function failed, e.g. tried to work on a view that doesn't exist
    ModelBadInput,  // 3 in a model | The function couldn't successfully use a required input parameter (rare) (e.g. A requested quadrature scheme couldn't be applied to the data)
    ModelParallelMeshQuery, // 4 in a model | A parallelizable mesh query function failed (rare)
    UnknownOption, // 1 in an option function
}
