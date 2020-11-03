extern crate kiss3d;
extern crate nalgebra as na;

use kiss3d::camera::ArcBall;
use kiss3d::light::Light;
use kiss3d::window::Window;
use log::*;
use na::{Point3, Translation3};
use std::path::Path;

use crate::minecraft;

pub fn render_chunk(chunk: minecraft::chunk::Chunk) {
    let mut window = Window::new("Chunk");
    let mut camera = ArcBall::new(
        Point3::<f32>::new(32.0, 256.0, 0.0),
        Point3::<f32>::new(16.0, 128.0, 0.0),
    );

    for (coord, block) in chunk.blocks.iter() {
        let mut c = window.add_cube(1.0, 1.0, 1.0);
        c.append_translation(&Translation3::<f32>::new(
            coord.z as f32,
            coord.y as f32,
            -coord.x as f32,
        ));
        match block.id.as_str() {
            "minecraft:bedrock" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/bedrock.png",
                    ),
                    "bedrock",
                );
            }
            "minecraft:stone" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/stone.png",
                    ),
                    "stone",
                );
            }
            "minecraft:dirt" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/dirt.png",
                    ),
                    "dirt",
                );
            }
            "minecraft:grass_block" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/grass_block_side.png",
                    ),
                    "grass_block",
                );
            }
            "minecraft:diorite" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/diorite.png",
                    ),
                    "diorite",
                );
            }
            "minecraft:coal_ore" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/coal_ore.png",
                    ),
                    "coal_ore",
                );
            }
            "minecraft:granite" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/granite.png",
                    ),
                    "granite",
                );
            }
            "minecraft:iron_ore" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/iron_ore.png",
                    ),
                    "iron_ore",
                );
            }
            "minecraft:andesite" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/andesite.png",
                    ),
                    "andesite",
                );
            }
            "minecraft:diamond_ore" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/diamond_ore.png",
                    ),
                    "diamond_ore",
                );
            }
            "minecraft:gold_ore" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/gold_ore.png",
                    ),
                    "gold_ore",
                );
            }
            "minecraft:gravel" => {
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/gravel.png",
                    ),
                    "gravel",
                );
            }
            _ => {
                // info!("Untextruedblock: {}", block.id);
                c.set_texture_from_file(
                    Path::new(
                        "/home/ericm/Downloads/1.16.3/assets/minecraft/textures/block/cyan_wool.png",
                    ),
                    "cyan_wool",
                );
            }
        }
    }

    window.set_light(Light::StickToCamera);

    while window.render_with_camera(&mut camera) {}
}
