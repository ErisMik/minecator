extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use na::{Point3, Translation3, UnitQuaternion, Vector3};
use std::path::Path;

pub fn main() {
    let mut window = Window::new("Kiss3d: cube");
    let mut camera = ArcBall::new(
        Point3::<f32>::new(10.0, 10.0, 0.0),
        Point3::<f32>::new(0.0, 0.0, 0.0),
    );

    let mut c = window.add_cube(1.0, 1.0, 1.0);
    let mut c2 = window.add_cube(1.0, 1.0, 1.0);

    c.append_translation(&Translation3::<f32>::new(0.0, 0.0, 0.0));
    c.set_texture_from_file(
        Path::new("/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/cobblestone.png"),
        "cobblestone",
    );

    c2.append_translation(&Translation3::<f32>::new(1.0, 0.0, 0.0));
    c2.set_texture_from_file(
        Path::new("/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/cobblestone.png"),
        "cobblestone",
    );

    window.set_light(Light::StickToCamera);

    while window.render_with_camera(&mut camera) {}
}
