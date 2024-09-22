use linalg::{Matrix4D, Vector2D, Vector3D};
use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use renderer::{Camera, Color, FrameBuffer, Input, Mesh, Object, Renderer, Scene, Triangle};
use zbuf::get_depth_func;

mod linalg;
pub mod renderer;
pub mod vector;
pub mod zbuf;

fn geometric_to_screen(vec: &Vector3D, width: usize, height: usize) -> Vector2D {
    let x_screen = (vec.x + 1.0) * (width as f64) / 2.0;
    let y_screen = (1.0 - vec.y) * (height as f64) / 2.0;

    Vector2D {
        x: x_screen,
        y: y_screen,
    }
}

/*
   xs = (gx + 1) * width / 2.0
   xs * 2.0 / width = gx + 1
   xs * 2.0 / width - 1 = gx

   ys = (1.0 - yg) * height / 2.0
   ys * 2.0 / height = (1.0 - yg)
   ys * 2.0 / height - 1.0 = - yg
   -(ys * 2.0 / height - 1.0) = yg
*/

fn screen_to_geo(x: i32, y: i32, width: usize, height: usize) -> Vector2D {
    let x_geo = ((x as f64 * 2.0) / width as f64) - 1.0;
    let y_geo = -(y as f64 * 2.0 / height as f64 - 1.0);

    Vector2D { x: x_geo, y: y_geo }
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

const WIDTH: usize = 1280;
const HEIGHT: usize = 720;

fn main() {
    let screen_coords = Vector2D::new(1280.0, 10.0);
    println!("{}, {}", screen_coords.x, screen_coords.y);
    let geo_coords = screen_to_geo(screen_coords.x as i32, screen_coords.y as i32, WIDTH, HEIGHT);
    println!("{}, {}", geo_coords.x, geo_coords.y);
    let intermediate = Vector3D::new(geo_coords.x, geo_coords.y, 0.0);
    let screen_coords2 = geometric_to_screen(&intermediate, WIDTH, HEIGHT);
    println!("{}, {}", screen_coords2.x, screen_coords2.y);

    let white = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    let gray = Color::new(128, 128, 128, 255);

    let cube_mesh: Mesh = Mesh {
        triangles: vec![
            // south
            Triangle::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(1.0, 1.0, 0.0),
                &gray,
            ),
            Triangle::new(
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
                &gray,
            ),
            // east
            Triangle::new(
                Vector3D::new(1.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
                &gray,
            ),
            Triangle::new(
                Vector3D::new(1.0, 0.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
                Vector3D::new(1.0, 0.0, 1.0),
                &gray,
            ),
            // north
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(1.0, 1.0, 1.0),
                Vector3D::new(0.0, 1.0, 1.0),
                &gray,
            ),
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(0.0, 1.0, 1.0),
                Vector3D::new(0.0, 0.0, 1.0),
                &gray,
            ),
            // west
            Triangle::new(
                Vector3D::new(0.0, 0.0, 1.0),
                Vector3D::new(0.0, 1.0, 1.0),
                Vector3D::new(0.0, 1.0, 0.0),
                &gray,
            ),
            Triangle::new(
                Vector3D::new(0.0, 0.0, 1.0),
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(0.0, 0.0, 0.0),
                &gray,
            ),
            // top
            Triangle::new(
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(0.0, 1.0, 1.0),
                Vector3D::new(1.0, 1.0, 1.0),
                &white,
            ),
            Triangle::new(
                Vector3D::new(0.0, 1.0, 0.0),
                Vector3D::new(1.0, 1.0, 1.0),
                Vector3D::new(1.0, 1.0, 0.0),
                &white,
            ),
            // bottom
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(0.0, 0.0, 1.0),
                Vector3D::new(0.0, 0.0, 0.0),
                &white,
            ),
            Triangle::new(
                Vector3D::new(1.0, 0.0, 1.0),
                Vector3D::new(0.0, 0.0, 0.0),
                Vector3D::new(1.0, 0.0, 0.0),
                &white,
            ),
        ],
    };

    let f_near = 0.1;
    let f_far = 1000.0;
    let f_fov = 90.0;
    let f_aspect_ratio = HEIGHT as f64 / WIDTH as f64;

    let mut cam = Camera {
        position: Vector3D::new(0.0, 0.0, 0.0),
        front: Vector3D::new(0.0, 0.0, -1.0),
        up: Vector3D::new(0.0, 1.0, 0.0),
        yaw: 0.0,
        pitch: 0.0,
        near_clip: f_near,
        far_clip: f_far,
        aspect_ratio: f_aspect_ratio,
    };

    let proj_mat = cam.get_proj_matrix(f_aspect_ratio, f_fov, f_near, f_far);
    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    let mut renderer = Renderer {
        camera: cam,
        framebuffer: frame_buffer,
    };

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

    let mut prev_mouse_x = 0.0;
    let mut prev_mouse_y = 0.0;

    let mut has_initialized_mouse_pos = false;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let mut input = Input {
            backward: false,
            forward: false,
            left: false,
            right: false,
            up: false,
            down: false,
            mouse_dx: 0.0,
            mouse_dy: 0.0,
        };

        if let Some((x, y)) = window.get_mouse_pos(minifb::MouseMode::Pass) {
            if !has_initialized_mouse_pos {
                prev_mouse_x = x;
                prev_mouse_y = y;
                has_initialized_mouse_pos = true;
            }

            let delta_x = x - prev_mouse_x;
            let delta_y = y - prev_mouse_y;

            input.mouse_dx = delta_x as f64;
            input.mouse_dy = delta_y as f64;

            prev_mouse_x = x;
            prev_mouse_y = y;
        }

        if window.is_key_down(Key::A) {
            input.left = true;
        }
        if window.is_key_down(Key::D) {
            input.right = true;
        }

        if window.is_key_down(Key::W) {
            input.forward = true;
        }
        if window.is_key_down(Key::S) {
            input.backward = true;
        }

        if window.is_key_down(Key::Space) {
            input.up = true;
        }
        if window.is_key_down(Key::LeftCtrl) {
            input.down = true;
        }

        renderer.camera.update(&input, 0.01);

        renderer.framebuffer.clear();
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

        let view_matrix = renderer.camera.create_view_matrix();

        for i in 0..3 {
            let proj_2d = cube_mesh
                // .apply_transformation(&mat_rot_z)
                // .apply_transformation(&mat_rot_x)
                // .apply_transformation(&get_x_rotation_matrix(cam_x_theta))
                // .apply_transformation(&get_y_rotation_matrix(cam_y_theta))
                .translate(&Vector3D {
                    x: i as f64,
                    y: 0.0,
                    z: 3.0,
                })
                .translate(&renderer.camera.position.scale(-1.0))
                .apply_transformation(&view_matrix)
                .apply_transformation_with_perspective_div(&proj_mat);

            for triangle in proj_2d.triangles {
                // let mut on_screen = true;
                // renderer.fill_triangle(&triangle);

                let depth = get_depth_func(&triangle);
                renderer.framebuffer.depth_func = Box::new(depth);

                let v1 = geometric_to_screen(&triangle.vertices[0], WIDTH, HEIGHT);
                let v2 = geometric_to_screen(&triangle.vertices[1], WIDTH, HEIGHT);
                let v3 = geometric_to_screen(&triangle.vertices[2], WIDTH, HEIGHT);

                // renderer.framebuffer.drawline(
                //     v1.x as i32,
                //     v1.y as i32,
                //     v2.x as i32,
                //     v2.y as i32,
                //     &white,
                // );
                // renderer.framebuffer.drawline(
                //     v2.x as i32,
                //     v2.y as i32,
                //     v3.x as i32,
                //     v3.y as i32,
                //     &white,
                // );
                // renderer.framebuffer.drawline(
                //     v3.x as i32,
                //     v3.y as i32,
                //     v1.x as i32,
                //     v1.y as i32,
                //     &white,
                // );

                // let vertices_2d = vec![v1, v2, v3].iter().map(|v| Vector3D::new(v.x as f64, v.y as f64, 0.0)).collect();

                // let tri_2d = Triangle {
                //     vertices: vertices_2d,
                // };
                // println!("{}", triangle.vertices[0].z);

                renderer.fill_triangle(&v1, &v2, &v3, &triangle.color);
            }
        }

        window
            .update_with_buffer(
                &renderer.framebuffer.color_buffer,
                renderer.framebuffer.width,
                renderer.framebuffer.height,
            )
            .unwrap();
    }
}
