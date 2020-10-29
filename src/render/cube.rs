use luminance::{Semantics, Vertex};

pub const VS: &'static str = include_str!("cube.vert");
pub const FS: &'static str = include_str!("cube.frag");

#[derive(Clone, Copy, Debug, Eq, PartialEq, Semantics)]
pub enum Semantics {
    // reference vertex positions with the co variable in vertex shaders
    #[sem(name = "co", repr = "[f32; 2]", wrapper = "VertexPosition")]
    Position,
    // reference vertex positions with the co3 variable in vertex shaders
    #[sem(name = "co3", repr = "[f32; 3]", wrapper = "VertexPosition3")]
    Position3,
    // reference vertex colors with the color variable in vertex shaders
    #[sem(name = "color", repr = "[f32; 3]", wrapper = "VertexColor")]
    Color,
    // reference vertex normals with the nor variable in vertex shaders
    #[sem(name = "nor", repr = "[f32; 3]", wrapper = "VertexNormal")]
    Normal,
    // reference vertex instance’s position on screen
    #[sem(
        name = "position",
        repr = "[f32; 2]",
        wrapper = "VertexInstancePosition"
    )]
    InstancePosition,
    // reference vertex size in vertex shaders (used for vertex instancing)
    #[sem(name = "weight", repr = "f32", wrapper = "VertexWeight")]
    Weight,
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, Vertex)]
#[vertex(sem = "Semantics")]
pub struct CubeVertex {
    pub pos: VertexPosition3,
    pub nor: VertexNormal,
}

pub type VertexIndex = u8;

#[rustfmt::skip]
pub fn cube(size: f32) -> ([CubeVertex; 24], [VertexIndex; 30]) {
  let s = size * 0.5;

  let vertices = [
    // first face
    CubeVertex::new([-s, -s,  s].into(), [ 0.,  0.,  1.].into()),
    CubeVertex::new([ s, -s,  s].into(), [ 0.,  0.,  1.].into()),
    CubeVertex::new([-s,  s,  s].into(), [ 0.,  0.,  1.].into()),
    CubeVertex::new([ s,  s,  s].into(), [ 0.,  0.,  1.].into()),
    // second face
    CubeVertex::new([ s, -s, -s].into(), [ 0.,  0., -1.].into()),
    CubeVertex::new([-s, -s, -s].into(), [ 0.,  0., -1.].into()),
    CubeVertex::new([ s,  s, -s].into(), [ 0.,  0., -1.].into()),
    CubeVertex::new([-s,  s, -s].into(), [ 0.,  0., -1.].into()),
    // third face
    CubeVertex::new([ s, -s,  s].into(), [ 1.,  0.,  0.].into()),
    CubeVertex::new([ s, -s, -s].into(), [ 1.,  0.,  0.].into()),
    CubeVertex::new([ s,  s,  s].into(), [ 1.,  0.,  0.].into()),
    CubeVertex::new([ s,  s, -s].into(), [ 1.,  0.,  0.].into()),
    // forth face
    CubeVertex::new([-s, -s, -s].into(), [-1.,  0.,  0.].into()),
    CubeVertex::new([-s, -s,  s].into(), [-1.,  0.,  0.].into()),
    CubeVertex::new([-s,  s, -s].into(), [-1.,  0.,  0.].into()),
    CubeVertex::new([-s,  s,  s].into(), [-1.,  0.,  0.].into()),
    // fifth face
    CubeVertex::new([-s,  s,  s].into(), [ 0.,  1.,  0.].into()),
    CubeVertex::new([ s,  s,  s].into(), [ 0.,  1.,  0.].into()),
    CubeVertex::new([-s,  s, -s].into(), [ 0.,  1.,  0.].into()),
    CubeVertex::new([ s,  s, -s].into(), [ 0.,  1.,  0.].into()),
    // sixth face
    CubeVertex::new([-s, -s, -s].into(), [ 0., -1.,  0.].into()),
    CubeVertex::new([ s, -s, -s].into(), [ 0., -1.,  0.].into()),
    CubeVertex::new([-s, -s,  s].into(), [ 0., -1.,  0.].into()),
    CubeVertex::new([ s, -s,  s].into(), [ 0., -1.,  0.].into()),
  ];

  let indices = [
    0, 1, 2, 3, VertexIndex::max_value(),
    4, 5, 6, 7, VertexIndex::max_value(),
    8, 9, 10,  11, VertexIndex::max_value(),
    12, 13, 14, 15, VertexIndex::max_value(),
    16, 17, 18, 19, VertexIndex::max_value(),
    20, 21, 22, 23, VertexIndex::max_value(),
  ];

  (vertices, indices)
}
