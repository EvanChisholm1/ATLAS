use crate::linalg::{
    cross, multiply_matrix_vector, multiply_matrix_vector_perspective_div, Matrix4D, Vector2D,
    Vector3D,
};
use std::cmp::{max, min};
use std::{cmp::Ordering, f64::consts::PI};

pub struct Renderer {
    // scene: Scene,
    pub framebuffer: FrameBuffer,
    pub camera: Camera,
}

impl Renderer {
    // fn render(&mut self) {}

    pub fn fill_triangle(&mut self, v1: &Vector2D, v2: &Vector2D, v3: &Vector2D, color: &Color) {
        let mut vertices = vec![v1, v2, v3];
        vertices.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());

        let v1 = vertices[0];
        let v2 = vertices[1];
        let v3 = vertices[2];

        let slope_1 = (v3.x - v1.x) / (v3.y - v1.y);
        let slope_2 = (v2.x - v1.x) / (v2.y - v1.y);
        let slope_3 = (v3.x - v2.x) / (v3.y - v2.y);

        let mut x1 = v1.x;
        let mut x2 = v1.x;

        for y in (v1.y as i32)..=(v2.y as i32) {
            if (y as usize) < self.framebuffer.height {
                self.draw_scanline(x1.round() as i32, x2.round() as i32, y, color);
            }
            x1 += slope_1;
            x2 += slope_2;
        }

        x2 = v2.x;

        for y in (v2.y as i32)..=(v3.y as i32) {
            if (y as usize) < self.framebuffer.height {
                self.draw_scanline(x1.round() as i32, x2.round() as i32, y, color);
                // self.draw_scanline(x1 as i32, x2 as i32, y, color);
            }
            x1 += slope_1;
            x2 += slope_3;
        }
    }

    pub fn draw_scanline(&mut self, x1: i32, x2: i32, y: i32, color: &Color) {
        let start = max(0, min(x1, x2));
        let end = min((self.framebuffer.width - 1) as i32, max(x1, x2));

        for x in start..(end+1) {
            self.framebuffer
                .set_pixel(x as usize, y as usize, color, 0.0);
        }
    }
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
            height,
        }
    }

    pub fn clear(&mut self) {
        let len = self.width * self.height;

        for i in 0..len {
            self.color_buffer[i] = 0;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: &Color, depth: f32) {
        if x > self.width || y > self.height {
            return;
        }

        if y * self.width + x >= self.width * self.height {
            return;
        }

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
                err += dy;
                x1 += sx;
            }

            if e2 <= dx {
                err += dx;
                y1 += sy
            }
        }
    }

    pub fn draw_triangle(&mut self, tri: &Triangle) {}
}

pub struct Scene {
    pub objects: Vec<Object>,
}

pub struct Object {
    pub mesh: Mesh,
    pub transform: Matrix4D,
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
                    color: t.color.clone(),
                }
            })
            .collect();

        Mesh {
            triangles: updated_triangles,
        }
    }

    pub fn apply_transformation_with_perspective_div(&self, mat: &Matrix4D) -> Mesh {
        let updated_triangles = self
            .triangles
            .iter()
            .filter_map(|t| {
                let updated_vertices: Vec<Vector3D> = t
                    .vertices
                    .iter()
                    .filter_map(|v| multiply_matrix_vector_perspective_div(v, mat))
                    .collect();

                if updated_vertices.len() < 3 {
                    return None;
                }

                Some(Triangle {
                    vertices: updated_vertices,
                    color: t.color.clone(),
                })
            })
            .collect();

        Mesh {
            triangles: updated_triangles,
        }
    }

    pub fn translate(&self, translator: &Vector3D) -> Mesh {
        let updated_triangles = self
            .triangles
            .iter()
            .map(|t| {
                let updated_vertices = t.vertices.iter().map(|v| v.add(translator)).collect();

                Triangle {
                    vertices: updated_vertices,
                    color: t.color.clone(),
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
    pub color: Color,
}

impl Triangle {
    pub fn new(a: Vector3D, b: Vector3D, c: Vector3D, color: &Color) -> Self {
        Triangle {
            vertices: vec![a, b, c],
            color: color.clone(),
        }
    }
}

// pub struct Vertex {
//     pub position: Vector3D,
// }

pub struct Input {
    pub forward: bool,
    pub backward: bool,
    pub left: bool,
    pub right: bool,
    pub up: bool,
    pub down: bool,
    pub mouse_dx: f64,
    pub mouse_dy: f64,
}

pub struct Camera {
    pub position: Vector3D,
    pub aspect_ratio: f64,
    pub near_clip: f64,
    pub far_clip: f64,
    pub front: Vector3D,
    pub up: Vector3D,
    pub yaw: f64,
    pub pitch: f64,
}

impl Camera {
    pub fn update(&mut self, input: &Input, delta_time: f64) {
        let mouse_sensitivity = 0.004;
        self.yaw += input.mouse_dx * mouse_sensitivity;
        self.pitch += input.mouse_dy * mouse_sensitivity;
        self.pitch = self.pitch.clamp(-PI / 4.0, PI / 4.0);

        let forward = Vector3D::new(self.yaw.sin(), 0.0, self.yaw.cos());
        let mut movement = Vector3D::new(0.0, 0.0, 0.0);

        let right_yaw = self.yaw + PI / 2.0;
        let right = Vector3D::new(right_yaw.sin(), 0.0, right_yaw.cos());

        if input.forward {
            movement = movement.add(&forward);
        }
        if input.backward {
            movement = movement.add(&forward.scale(-1.0));
        }
        if input.right {
            movement = movement.add(&right);
        }
        if input.left {
            movement = movement.add(&right.scale(-1.0));
        }

        if !movement.is_zero() {
            movement = movement.normalize();
        }

        if input.up {
            movement = movement.add(&Vector3D::new(0.0, 1.0, 0.0));
        }
        if input.down {
            movement = movement.add(&Vector3D::new(0.0, -1.0, 0.0));
        }

        let move_speed = 10.0;
        let speed = move_speed * delta_time;
        self.position = self.position.add(&movement.scale(speed));
    }

    pub fn create_view_matrix(&self) -> Matrix4D {
        let (sin_ud, cos_ud) = self.yaw.sin_cos();
        let (sin_lr, cos_lr) = self.pitch.sin_cos();

        let rotation_ud = Matrix4D::new([
            [cos_ud, 0.0, sin_ud, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [-sin_ud, 0.0, cos_ud, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let rotation_lr = Matrix4D::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, cos_lr, -sin_lr, 0.0],
            [0.0, sin_lr, cos_lr, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        &rotation_ud * &rotation_lr
    }

    pub fn get_proj_matrix(&self, aspect_ratio: f64, fov: f64, near: f64, far: f64) -> Matrix4D {
        let fov_rad = fov.to_radians();

        Matrix4D::new([
            [aspect_ratio * fov_rad, 0.0, 0.0, 0.0],
            [0.0, fov_rad, 0.0, 0.0],
            [0.0, 0.0, far / (far - near), 1.0],
            [0.0, 0.0, (-far * near) / (far - near), 0.0],
        ])
    }
}

#[derive(Clone)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn to_u32(&self) -> u32 {
        let (r, g, b, a) = (self.r as u32, self.g as u32, self.b as u32, self.a as u32);
        (a << 24) | (r << 16) | (g << 8) | b
    }
}
