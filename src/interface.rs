//! The interface to the low-level gmsh_sys crate.

/// The set of OpenCASCADE kernel functions.
pub mod occ {

    // unique functions
    pub use gmsh_sys::gmshModelOccAddBox as add_box;
    pub use gmsh_sys::gmshModelOccAddSphere as add_sphere;
    pub use gmsh_sys::gmshModelOccAddTorus as add_torus;

    // shared functions
    pub use gmsh_sys::gmshModelOccSynchronize as synchronize;
    pub use gmsh_sys::gmshModelOccRemove as remove_point;
    pub use gmsh_sys::gmshModelOccAddPoint as add_point;
    pub use gmsh_sys::gmshModelOccAddLine as add_line;

}

/// The set of built-in kernel functions.
pub mod geo {

    // unique functions

    // shared functions
    pub use gmsh_sys::gmshModelGeoSynchronize as synchronize;
    pub use gmsh_sys::gmshModelGeoRemove as remove_point;
    pub use gmsh_sys::gmshModelGeoAddPoint as add_point;
    pub use gmsh_sys::gmshModelGeoAddLine as add_line;

}
