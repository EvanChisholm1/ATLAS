use crate::{linalg::cross, renderer::Triangle};

pub fn get_depth_func(tri: &Triangle) -> impl Fn(f64, f64) -> f64 {
    // side vectors
    let v1v2 = tri.vertices[1].sub(&tri.vertices[0]);
    let v2v3 = tri.vertices[2].sub(&tri.vertices[1]);

    // find the norm of the triangle
    let norm = cross(&v1v2, &v2v3);

    // px, py, pz represent a point on the triangle
    let (px, py, pz) = (tri.vertices[0].x, tri.vertices[0].y, tri.vertices[0].z);
    // the values of the norm
    let (nx, ny, nz) = (norm.x, norm.y, norm.z);

    // rearranged scalar equation of a plane to solve for c
    let c = -(nx * px + ny * py + nz * pz);

    // create a closure
    move |x: f64, y: f64| (-(nx * x) - (ny * y) - c) / nz
}
