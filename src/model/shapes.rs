//! Shape objects.

/// A point.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    /// x-coordinate
    pub x: f64,
    /// y-coordinate
    pub y: f64,
    /// z-coordinate
    pub z: f64,
}

/// A torus.
#[derive(Debug, Copy, Clone)]
pub struct Torus {
    /// Centroid
    pub centroid: Point,
    /// Major radius (radius of the donut)
    pub main_radius: f64,
    /// Minor radius (radius of the tube)
    pub pipe_radius: f64,
}
