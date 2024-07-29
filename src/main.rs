use linalg::{Matrix4D, Vector2D, Vector3D};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
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

    while window.is_open() && !window.is_key_down(Key::Escape) {
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

        let z_translator = Vector3D::new(0.0, 0.0, 3.0);
        let proj_2d = cube_mesh
            .apply_transformation(&mat_rot_z)
            .apply_transformation(&mat_rot_x)
            .translate(&z_translator)
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
