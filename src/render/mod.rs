use hashbrown::HashMap;
use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageFormat, Rgba, RgbaImage};
use lazy_static::lazy_static;
use log::*;
use rayon::prelude::*;

use crate::minecraft::{chunk, region};

const ASSETS_BASE_PATH: &str =
    "/home/ericm/.minecraft/versions/1.17.1/1.17.1/assets/minecraft/textures/block";
const TEXTURE_RESOLUTION: u32 = 16;

fn load_texture(filename: &str) -> DynamicImage {
    return ImageReader::open(format!("{}/{}.png", ASSETS_BASE_PATH, filename))
        .unwrap()
        .decode()
        .unwrap();
}

lazy_static! {
    static ref TEXTURE_HASHMAP: HashMap<String, DynamicImage> = {
        let mut textures = HashMap::new();
        textures.insert(String::from("minecraft:stone"), load_texture("stone"));
        textures.insert(
            String::from("minecraft:oak_leaves"),
            load_texture("oak_leaves"),
        );
        textures.insert(
            String::from("minecraft:oak_log"),
            load_texture("oak_log_top"),
        );
        textures.insert(String::from("minecraft:dirt"), load_texture("dirt"));
        textures.insert(
            String::from("minecraft:grass_block"),
            load_texture("grass_block_top"),
        );
        textures.insert(
            String::from("minecraft:water"),
            load_texture("water_overlay"),
        );
        textures.insert(String::from("minecraft:bedrock"), load_texture("bedrock"));
        textures.insert(String::from("minecraft:andesite"), load_texture("andesite"));
        textures.insert(String::from("minecraft:granite"), load_texture("granite"));
        textures.insert(String::from("minecraft:iron_ore"), load_texture("iron_ore"));
        textures.insert(String::from("minecraft:coal_ore"), load_texture("coal_ore"));
        textures.insert(String::from("minecraft:gravel"), load_texture("gravel"));
        textures.insert(String::from("minecraft:diorite"), load_texture("diorite"));
        textures.insert(
            String::from("minecraft:redstone_ore"),
            load_texture("redstone_ore"),
        );
        textures.insert(
            String::from("minecraft:spruce_leaves"),
            load_texture("spruce_leaves"),
        );
        textures.insert(
            String::from("minecraft:birch_leaves"),
            load_texture("birch_leaves"),
        );
        textures.insert(String::from("minecraft:cobweb"), load_texture("cobweb"));
        textures.insert(
            String::from("minecraft:deepslate_iron_ore"),
            load_texture("deepslate_iron_ore"),
        );
        textures.insert(
            String::from("minecraft:lapis_ore"),
            load_texture("lapis_ore"),
        );
        textures.insert(
            String::from("minecraft:copper_ore"),
            load_texture("copper_ore"),
        );
        textures.insert(
            String::from("minecraft:lapis_ore"),
            load_texture("lapis_ore"),
        );
        textures.insert(String::from("minecraft:gold_ore"), load_texture("gold_ore"));
        textures.insert(
            String::from("minecraft:oak_planks"),
            load_texture("oak_planks"),
        );
        textures.insert(
            String::from("minecraft:spruce_log"),
            load_texture("spruce_log_top"),
        );
        textures.insert(
            String::from("minecraft:birch_log"),
            load_texture("birch_log_top"),
        );
        textures.insert(
            String::from("minecraft:deepslate"),
            load_texture("deepslate_top"),
        );

        textures
    };
}

pub fn render_image(regions: Vec<region::Region>) {
    regions.par_iter().for_each(|region| {
        let region_image_wh =
            region::CHUNKS_PER_REGION_WH * chunk::CHUNK_WIDTH * TEXTURE_RESOLUTION;
        let mut region_image = RgbaImage::new(region_image_wh, region_image_wh);

        for chunk in region.chunks.iter() {
            let chunk_image_wh = chunk::CHUNK_WIDTH * TEXTURE_RESOLUTION;
            let mut image_layers = Vec::new();
            for _ in 0..chunk::CHUNK_HEIGHT {
                image_layers.push(RgbaImage::new(chunk_image_wh, chunk_image_wh));
            }

            for (coord, block) in chunk.blocks.iter() {
                let texture = match TEXTURE_HASHMAP.get(&block.id) {
                    Some(texture) => texture.clone(),
                    _ => {
                        // info!("Block not accounted for: {:?}", block);
                        DynamicImage::ImageRgba8(RgbaImage::new(
                            TEXTURE_RESOLUTION,
                            TEXTURE_RESOLUTION,
                        ))
                    }
                };

                let x = ((chunk::CHUNK_WIDTH - 1) - coord.x as u32) * TEXTURE_RESOLUTION;
                let z = coord.z as u32 * TEXTURE_RESOLUTION;
                image::imageops::overlay(&mut image_layers[coord.y as usize], &texture, x, z);
            }

            for layer in image_layers {
                let x: u32 = chunk.coordinate.x * chunk::CHUNK_WIDTH * TEXTURE_RESOLUTION;
                let z: u32 = chunk.coordinate.z * chunk::CHUNK_WIDTH * TEXTURE_RESOLUTION;
                image::imageops::overlay(&mut region_image, &layer, x, z);
            }
        }

        region_image
            .save_with_format(
                format!("images/{}_{}.png", region.x, region.z),
                ImageFormat::Png,
            )
            .unwrap();
    });
}
