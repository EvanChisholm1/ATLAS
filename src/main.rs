use std::f64::consts::PI;

use linalg::{Matrix4D, Vector2D, Vector3D};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions, CursorStyle};
use renderer::{Color, FrameBuffer, Mesh, Triangle};

mod linalg;
pub mod renderer;

fn geometric_to_screen(vec: &Vector3D, width: usize, height: usize) -> Vector2D {
    let x_screen = (vec.x + 1.0) * (width as f64) / 2.0;
    let y_screen = (1.0 - vec.y) * (height as f64) / 2.0;

    Vector2D {
        x: x_screen,
        y: y_screen,
    }
}

fn get_z_rotation_matrix(theta: f64) -> Matrix4D {
    Matrix4D::new([
        [theta.cos(), theta.sin(), 0.0, 0.0],
        [-theta.sin(), theta.cos(), 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn get_x_rotation_matrix(theta: f64) -> Matrix4D {
    Matrix4D::new([
        [1.0, 0.0, 0.0, 0.0],
        [0.0, theta.cos(), theta.sin(), 0.0],
        [0.0, -theta.sin(), theta.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}

fn get_y_rotation_matrix(theta: f64) -> Matrix4D {
    Matrix4D::new([
        [theta.cos(), 0.0, -theta.sin(), 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [theta.sin(), 0.0, theta.cos(), 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ])
}




const WIDTH: usize = 640;
const HEIGHT: usize = 360;

fn main() {
    let cube_mesh: Mesh = Mesh {
        triangles: vec![
            // south
            Triangle::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(1.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
            // east
            Triangle::new(
                Vector3D::new(1.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vector3D::new(1.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
                Vector3D::new(1.0, 0.0, 1.0),
            ),
            // north
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(1.0, 1.0, 1.0),
                Vector3D::new(0.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(0.0, 1.0, 1.0),
                Vector3D::new(0.0, 0.0, 1.0),
            ),
            // west
            Triangle::new(
                Vector3D::new(0.0, 0.0, 1.0),
                Vector3D::new(0.0, 1.0, 1.0),
                Vector3D::new(0.0, 1.0, 0.0),
            ),
            Triangle::new(
                Vector3D::new(0.0, 0.0, 1.0),
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(0.0, 0.0, 0.0),
            ),
            // top
            Triangle::new(
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(0.0, 1.0, 1.0),
                Vector3D::new(1.0, 1.0, 1.0),
            ),
            Triangle::new(
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
                Vector3D::new(1.0, 1.0, 0.0),
            ),
            // bottom
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(0.0, 0.0, 1.0),
                Vector3D::new(0.0, 0.0, 0.0),
            ),
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
            ),
        ],
    };

    let f_near = 0.1;
    let f_far = 1000.0;
    let f_fov = 90.0;
    let f_aspect_ratio = HEIGHT as f64 / WIDTH as f64;
    let f_fov_rad = 1.0 / ((f_fov * 0.5 / 180.0 * 3.14159) as f64).tan();

    let proj_mat = Matrix4D::new([
        [f_aspect_ratio * f_fov_rad, 0.0, 0.0, 0.0],
        [0.0, f_fov_rad, 0.0, 0.0],
        [0.0, 0.0, f_far / (f_far - f_near), 1.0],
        [0.0, 0.0, (-f_far * f_near) / (f_far - f_near), 0.0],
    ]);

    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let mut window =
        Window::new("ATLAS", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);
    let white = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let mut theta: f64 = 0.0;
    let mut cam_loc = Vector3D::new(0.0, 0.0, 3.0);
    let mut cam_x_theta: f64 = 0.0;
    let mut cam_y_theta: f64 = 0.0;

    let mut prev_mouse_x = 0.0;
    let mut prev_mouse_y = 0.0;

    let mut has_initialized_mouse_pos = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            if !has_initialized_mouse_pos {
                prev_mouse_x = x;
                prev_mouse_y = y;
                has_initialized_mouse_pos = true;
            }

            let delta_x = x - prev_mouse_x;
            let delta_y = y - prev_mouse_y;

            cam_x_theta += -(delta_y * 0.01) as f64;
            cam_y_theta += -(delta_x * 0.01) as f64;
            // cam_y_theta = cam

            cam_x_theta = cam_x_theta.clamp(-PI / 4.0, PI / 4.0);

            prev_mouse_x = x;
            prev_mouse_y = y;
        }

        if window.is_key_down(Key::A) {
            cam_loc.x += 0.1;
        }
        if window.is_key_down(Key::D) {
            cam_loc.x -= 0.1;
        }

        if window.is_key_down(Key::W) {
            cam_loc.z -= 0.1;
        }
        if window.is_key_down(Key::S) {
            cam_loc.z += 0.1;
        }

        if window.is_key_down(Key::Space) {
            cam_loc.y -= 0.1;
        }
        if window.is_key_down(Key::LeftCtrl) {
            cam_loc.y += 0.1;
        }


        frame_buffer.clear();
        theta += 0.03;

        let mat_rot_z = Matrix4D::new([
            [theta.cos(), theta.sin(), 0.0, 0.0],
            [-theta.sin(), theta.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        let mat_rot_x = Matrix4D::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, (0.5 * theta).cos(), (0.5 * theta).sin(), 0.0],
            [0.0, -(0.5 * theta).sin(), (theta * 0.5).cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]);

        // let z_translator = Vector3D::new(0.0, 0.0, 3.0);
        let proj_2d = cube_mesh
            // .apply_transformation(&mat_rot_z)
            // .apply_transformation(&mat_rot_x)
            .translate(&cam_loc)
            .apply_transformation(&get_x_rotation_matrix(cam_x_theta))
            .apply_transformation(&get_y_rotation_matrix(cam_y_theta))
            .apply_transformation(&proj_mat);

        for triangle in proj_2d.triangles {
            let v1 = geometric_to_screen(&triangle.vertices[0], WIDTH, HEIGHT);
            let v2 = geometric_to_screen(&triangle.vertices[1], WIDTH, HEIGHT);
            let v3 = geometric_to_screen(&triangle.vertices[2], WIDTH, HEIGHT);

            frame_buffer.drawline(v1.x as i32, v1.y as i32, v2.x as i32, v2.y as i32, &white);
            frame_buffer.drawline(v2.x as i32, v2.y as i32, v3.x as i32, v3.y as i32, &white);
            frame_buffer.drawline(v3.x as i32, v3.y as i32, v1.x as i32, v1.y as i32, &white);
        }

        window
            .update_with_buffer(
                &frame_buffer.color_buffer,
                frame_buffer.width,
                frame_buffer.height,
            )
            .unwrap();
    }
}
