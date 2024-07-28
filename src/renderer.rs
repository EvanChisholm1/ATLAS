use crate::linalg::{multiply_matrix_vector, Matrix4D, Vector3D};

pub struct Renderer {
    scene: Scene,
    framebuffer: FrameBuffer,
    camera: Camera,
}

impl Renderer {
    fn render(&mut self) {}
}

pub struct FrameBuffer {
    pub color_buffer: Vec<u32>,
    pub depth_buffer: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl FrameBuffer {
    pub fn new(width: usize, height: usize) -> FrameBuffer {
        FrameBuffer {
            color_buffer: vec![0; width * height],
            depth_buffer: vec![0.0; width * height],
            width,
            height
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color, depth: f32) {
        self.color_buffer[y * self.width + x] = color.to_u32();
        self.depth_buffer[y * self.width + x] = depth;
    }

    pub fn drawline(&mut self, x1: i32, y1: i32, x2: i32, y2: i32, color: &Color) {
        let dx = (x2 - x1).abs();
        let dy = -(y2 - y1).abs();

        let sx = if x1 < x2 { 1 } else { -1 };
        let sy = if y1 < y2 { 1 } else { -1 };

        let mut err = dx + dy;

        let mut x1 = x1;
        let mut y1 = y1;

        loop {
            if x1 >= 0 && x1 < self.width as i32 && y1 >= 0 && y1 < self.height as i32 {
                self.set_pixel(x1 as usize, y1 as usize, color, 0.0);
            }

            if x1 == x2 && y1 == y2 {
                break;
            }

            let e2 = 2 * err;
            if e2 >= dy {
                err +=  dy;
                x1 += sx;
            }

            if e2 <= dx {
                err += dx;
                y1 += sy
            }
        }
    }

    pub fn draw_triangle(&mut self, tri: &Triangle) {

    }
}

pub struct Scene {
    objects: Vec<Object>,
}

pub struct Object {
    mesh: Mesh,
    transform: Transform,
}

pub struct Mesh {
    pub triangles: Vec<Triangle>,
}

impl Mesh {
    pub fn apply_transformation(&self, mat: &Matrix4D) -> Mesh {
        let updated_triangles = self
            .triangles
            .iter()
            .map(|t| {
                let updated_vertices = t
                    .vertices
                    .iter()
                    .map(|v| multiply_matrix_vector(v, mat))
                    .collect();

                Triangle {
                    vertices: updated_vertices,
                }
            })
            .collect();

        Mesh {
            triangles: updated_triangles,
        }
    }
}

pub struct Triangle {
    pub vertices: Vec<Vector3D>,
}

impl Triangle {
    pub fn new(a: Vector3D, b: Vector3D, c: Vector3D) -> Self {
        Triangle {
            vertices: vec![a, b, c],
        }
    }
}

// pub struct Vertex {
//     pub position: Vector3D,
// }

pub struct Camera {
    position: Vector3D,
    aspect_ration: f32,
    near_clip: f32,
    far_clip: f32,
}

pub struct Transform {}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    // pub fn new(r: u8, g: u8, b: u8, a: u8) {
    //     Color
    // }

    pub fn to_u32(&self) -> u32 {
        let (r, g, b, a) = (self.r as u32, self.g as u32, self.b as u32, self.a as u32);
        (a << 24) | (r << 16) | (g << 8) | b
    }
}
