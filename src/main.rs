use minifb::{Key, MouseButton, MouseMode, Window, WindowOptions};
use linalg:: Vector3D;
use renderer::{Color, FrameBuffer, Mesh, Triangle};

mod linalg;
pub mod renderer;


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
        ]
    };


    let mut frame_buffer = FrameBuffer::new(WIDTH, HEIGHT);

    // let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];
    let mut window =
        Window::new("ATLAS", WIDTH, HEIGHT, WindowOptions::default()).unwrap_or_else(|e| {
            panic!("{}", e);
        });

    window.set_target_fps(60);

    while window.is_open() && !window.is_key_down(Key::Escape) {

        frame_buffer.drawline(0, 0, WIDTH as i32, HEIGHT as i32, &Color { r: 255, g: 255, b: 255, a: 255 });

        window.update_with_buffer(&frame_buffer.color_buffer, frame_buffer.width, frame_buffer.height).unwrap();
    }

    let u = linalg::Vector2D::new(1.0, 2.0);
    let v = linalg::Vector2D::new(10.0, -22.0);

    let o = u + v;

    println!("{}, {}", o.x, o.y);
}
