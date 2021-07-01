use std::{path::Path, fs, fs::File, io::Write};
use image::{GenericImage, RgbaImage, imageops};
use json::{object};
use uuid::Uuid;

pub fn copy(source: &str, copy: &str) {
    if Path::new(source).is_file() {
        fs::copy(source, copy).expect("Failed to copy file to new location.");
    } else {
        copy_dir(source, copy).expect("Failed to copy directory to new location.");
    }
}

fn copy_dir(from: impl AsRef<Path>, to: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), to.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

pub fn unstitch(path: &str) {
    let names = vec![format!("{}{}", path, "grass_top.png"),format!("{}{}", path, "stone.png"),format!("{}{}", path, "dirt.png"),format!("{}{}", path, "grass_side.png"),format!("{}{}", path, "planks_oak.png"),format!("{}{}", path, "stone_slab_side.png"),format!("{}{}", path, "stone_slab_top.png"),format!("{}{}", path, "brick.png"),format!("{}{}", path, "tnt_side.png"),format!("{}{}", path, "tnt_top.png"),format!("{}{}", path, "tnt_bottom.png"),format!("{}{}", path, "web.png"),format!("{}{}", path, "flower_rose.png"),format!("{}{}", path, "flower_dandelion.png"),"none".to_string(),format!("{}{}", path, "sapling_oak.png"),format!("{}{}", path, "cobblestone.png"),format!("{}{}", path, "bedrock.png"),format!("{}{}", path, "sand.png"),format!("{}{}", path, "gravel.png"),format!("{}{}", path, "log_oak.png"),format!("{}{}", path, "log_oak_top.png"),format!("{}{}", path, "iron_block.png"),format!("{}{}", path, "gold_block.png"),format!("{}{}", path, "diamond_block.png"),format!("{}{}", path, "emerald_block.png"),"none".to_string(),"none".to_string(),format!("{}{}", path, "mushroom_red.png"),format!("{}{}", path, "mushroom_brown.png"),format!("{}{}", path, "sapling_jungle.png"),"none".to_string(),format!("{}{}", path, "gold_ore.png"),format!("{}{}", path, "iron_ore.png"),format!("{}{}", path, "coal_ore.png"),format!("{}{}", path, "bookshelf.png"),format!("{}{}", path, "cobblestone_mossy.png"),format!("{}{}", path, "obsidian.png"),format!("{}{}", path, "grass_side_overlay.png"),format!("{}{}", path, "tallgrass.png"),format!("{}{}", path, "grass_top.png"),format!("{}{}", path, "beacon.png"),"none".to_string(),format!("{}{}", path, "crafting_table_top.png"),format!("{}{}", path, "furnace_front_off.png"),format!("{}{}", path, "furnace_side.png"),format!("{}{}", path, "dispenser_front_horizontal.png"),"none".to_string(),format!("{}{}", path, "sponge.png"),format!("{}{}", path, "glass.png"),format!("{}{}", path, "diamond_ore.png"),format!("{}{}", path, "redstone_ore.png"),format!("{}{}", path, "leaves_oak.png"),format!("{}{}", path, "leaves_oak_opaque.png"),format!("{}{}", path, "stonebrick.png"),format!("{}{}", path, "deadbush.png"),format!("{}{}", path, "fern.png"),"none".to_string(),"none".to_string(),format!("{}{}", path, "crafting_table_side.png"),format!("{}{}", path, "crafting_table_front.png"),format!("{}{}", path, "furnace_front_on.png"),format!("{}{}", path, "furnace_top.png"),format!("{}{}", path, "sapling_spruce.png"),format!("{}{}", path, "wool_colored_white.png"),format!("{}{}", path, "mob_spawner.png"),format!("{}{}", path, "snow.png"),format!("{}{}", path, "ice.png"),format!("{}{}", path, "grass_side_snowed.png"),format!("{}{}", path, "cactus_top.png"),format!("{}{}", path, "cactus_side.png"),format!("{}{}", path, "cactus_bottom.png"),format!("{}{}", path, "clay.png"),format!("{}{}", path, "reeds.png"),format!("{}{}", path, "jukebox_side.png"),format!("{}{}", path, "jukebox_top.png"),format!("{}{}", path, "waterlily.png"),format!("{}{}", path, "mycelium_side.png"),format!("{}{}", path, "mycelium_top.png"),format!("{}{}", path, "sapling_birch.png"),format!("{}{}", path, "torch_on.png"),format!("{}{}", path, "door_wood_upper.png"),format!("{}{}", path, "door_iron_upper.png"),format!("{}{}", path, "ladder.png"),format!("{}{}", path, "trapdoor.png"),format!("{}{}", path, "iron_bars.png"),format!("{}{}", path, "farmland_wet.png"),format!("{}{}", path, "farmland_dry.png"),format!("{}{}", path, "wheat_stage_0.png"),format!("{}{}", path, "wheat_stage_1.png"),format!("{}{}", path, "wheat_stage_2.png"),format!("{}{}", path, "wheat_stage_3.png"),format!("{}{}", path, "wheat_stage_4.png"),format!("{}{}", path, "wheat_stage_5.png"),format!("{}{}", path, "wheat_stage_6.png"),format!("{}{}", path, "wheat_stage_7.png"),format!("{}{}", path, "lever.png"),format!("{}{}", path, "door_wood_lower.png"),format!("{}{}", path, "door_iron_lower.png"),format!("{}{}", path, "redstone_torch_on.png"),format!("{}{}", path, "stonebrick_mossy.png"),format!("{}{}", path, "stonebrick_cracked.png"),format!("{}{}", path, "pumpkin_top.png"),format!("{}{}", path, "netherrack.png"),format!("{}{}", path, "soul_sand.png"),format!("{}{}", path, "glowstone.png"),format!("{}{}", path, "piston_top_sticky.png"),format!("{}{}", path, "piston_top_normal.png"),format!("{}{}", path, "piston_side.png"),format!("{}{}", path, "piston_bottom.png"),format!("{}{}", path, "piston_inner.png"),format!("{}{}", path, "pumpkin_stem_disconnected.png"),format!("{}{}", path, "rail_normal_turned.png"),format!("{}{}", path, "wool_colored_black.png"),format!("{}{}", path, "wool_colored_gray.png"),format!("{}{}", path, "redstone_torch_off.png"),format!("{}{}", path, "log_spruce.png"),format!("{}{}", path, "log_birch.png"),format!("{}{}", path, "pumpkin_side.png"),format!("{}{}", path, "pumpkin_face_off.png"),format!("{}{}", path, "pumpkin_face_on.png"),format!("{}{}", path, "cake_top.png"),format!("{}{}", path, "cake_side.png"),format!("{}{}", path, "cake_inner.png"),format!("{}{}", path, "cake_bottom.png"),format!("{}{}", path, "mushroom_block_skin_red.png"),format!("{}{}", path, "mushroom_block_skin_brown.png"),format!("{}{}", path, "pumpkin_stem_connected.png"),format!("{}{}", path, "rail_normal.png"),format!("{}{}", path, "wool_colored_red.png"),format!("{}{}", path, "wool_colored_pink.png"),format!("{}{}", path, "repeater_off.png"),format!("{}{}", path, "leaves_spruce.png"),format!("{}{}", path, "leaves_spruce_opaque.png"),format!("{}{}", path, "bed_feet_top.png"),format!("{}{}", path, "bed_head_top.png"),format!("{}{}", path, "melon_side.png"),format!("{}{}", path, "melon_top.png"),format!("{}{}", path, "cauldron_top.png"),format!("{}{}", path, "cauldron_inner.png"),"none".to_string(),format!("{}{}", path, "mushroom_block_skin_stem.png"),format!("{}{}", path, "mushroom_block_inside.png"),format!("{}{}", path, "vine.png"),format!("{}{}", path, "lapis_block.png"),format!("{}{}", path, "wool_colored_green.png"),format!("{}{}", path, "wool_colored_lime.png"),format!("{}{}", path, "repeater_on.png"),format!("{}{}", path, "glass_pane_top.png"),format!("{}{}", path, "bed_feet_end.png"),format!("{}{}", path, "bed_feet_side.png"),format!("{}{}", path, "bed_head_side.png"),format!("{}{}", path, "bed_head_end.png"),format!("{}{}", path, "log_jungle.png"),format!("{}{}", path, "cauldron_side.png"),format!("{}{}", path, "cauldron_bottom.png"),format!("{}{}", path, "brewing_stand_base.png"),format!("{}{}", path, "brewing_stand.png"),format!("{}{}", path, "endframe_top.png"),format!("{}{}", path, "endframe_side.png"),format!("{}{}", path, "lapis_ore.png"),format!("{}{}", path, "wool_colored_brown.png"),format!("{}{}", path, "wool_colored_yellow.png"),format!("{}{}", path, "rail_golden.png"),format!("{}{}", path, "redstone_dust_dot.png"),format!("{}{}", path, "redstone_dust_line0.png"),format!("{}{}", path, "enchanting_table_top.png"),format!("{}{}", path, "dragon_egg.png"),format!("{}{}", path, "cocoa_stage_2.png"),format!("{}{}", path, "cocoa_stage_1.png"),format!("{}{}", path, "cocoa_stage_0.png"),format!("{}{}", path, "emerald_ore.png"),format!("{}{}", path, "trip_wire_source.png"),format!("{}{}", path, "trip_wire.png"),format!("{}{}", path, "endframe_eye.png"),format!("{}{}", path, "end_stone.png"),format!("{}{}", path, "sandstone_top.png"),format!("{}{}", path, "wool_colored_blue.png"),format!("{}{}", path, "wool_colored_light_blue.png"),format!("{}{}", path, "rail_golden_powered.png"),"none".to_string(),"none".to_string(),format!("{}{}", path, "enchanting_table_side.png"),format!("{}{}", path, "enchanting_table_bottom.png"),format!("{}{}", path, "command_block_back.png"),format!("{}{}", path, "itemframe_background.png"),format!("{}{}", path, "flower_pot.png"),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),format!("{}{}", path, "sandstone_normal.png"),format!("{}{}", path, "wool_colored_purple.png"),format!("{}{}", path, "wool_colored_magenta.png"),format!("{}{}", path, "rail_detector.png"),format!("{}{}", path, "leaves_jungle.png"),format!("{}{}", path, "leaves_jungle_opaque.png"),format!("{}{}", path, "planks_spruce.png"),format!("{}{}", path, "planks_jungle.png"),format!("{}{}", path, "carrots_stage_0.png"),format!("{}{}", path, "carrots_stage_1.png"),format!("{}{}", path, "carrots_stage_2.png"),format!("{}{}", path, "carrots_stage_3.png"),format!("{}{}", path, "potatoes_stage_3.png"),"none".to_string(),"none".to_string(),"none".to_string(),format!("{}{}", path, "sandstone_bottom.png"),format!("{}{}", path, "wool_colored_cyan.png"),format!("{}{}", path, "wool_colored_orange.png"),format!("{}{}", path, "redstone_lamp_off.png"),format!("{}{}", path, "redstone_lamp_on.png"),format!("{}{}", path, "stonebrick_carved.png"),format!("{}{}", path, "planks_birch.png"),format!("{}{}", path, "anvil_base.png"),format!("{}{}", path, "anvil_top_damaged_1.png"),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),format!("{}{}", path, "nether_brick.png"),format!("{}{}", path, "wool_colored_silver.png"),format!("{}{}", path, "nether_wart_stage_0.png"),format!("{}{}", path, "nether_wart_stage_1.png"),format!("{}{}", path, "nether_wart_stage_2.png"),format!("{}{}", path, "sandstone_carved.png"),format!("{}{}", path, "sandstone_smooth.png"),format!("{}{}", path, "anvil_top_damaged_0.png"),format!("{}{}", path, "anvil_top_damaged_2.png"),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),format!("{}{}", path, "destroy_stage_0.png"),format!("{}{}", path, "destroy_stage_1.png"),format!("{}{}", path, "destroy_stage_2.png"),format!("{}{}", path, "destroy_stage_3.png"),format!("{}{}", path, "destroy_stage_4.png"),format!("{}{}", path, "destroy_stage_5.png"),format!("{}{}", path, "destroy_stage_6.png"),format!("{}{}", path, "destroy_stage_7.png"),format!("{}{}", path, "destroy_stage_8.png"),format!("{}{}", path, "destroy_stage_9.png"),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string(),"none".to_string()];
    let mut base = image::open(format!("{}{}", path, "terrain.png")).unwrap();
    let mut i: usize = 0;

    for y in (0..256).step_by(16) {
        for x in (0..256).step_by(16) {
            if names[i] != "none".to_string() {
                let texture: RgbaImage = base.sub_image(x, y, 16, 16).to_image();
                texture.save(&names[i]).unwrap();
            }
            i += 1;
        }
    }
}

pub fn stitch(path: &str) {
    let names = vec![format!("{}{}", path, "grass_top.png"),format!("{}{}", path, "stone.png"),format!("{}{}", path, "dirt.png"),format!("{}{}", path, "grass_side.png"),format!("{}{}", path, "planks_oak.png"),format!("{}{}", path, "stone_slab_side.png"),format!("{}{}", path, "stone_slab_top.png"),format!("{}{}", path, "brick.png"),format!("{}{}", path, "tnt_side.png"),format!("{}{}", path, "tnt_top.png"),format!("{}{}", path, "tnt_bottom.png"),format!("{}{}", path, "web.png"),format!("{}{}", path, "flower_rose.png"),format!("{}{}", path, "flower_dandelion.png"),"blank.png".to_string(),format!("{}{}", path, "sapling_oak.png"),format!("{}{}", path, "cobblestone.png"),format!("{}{}", path, "bedrock.png"),format!("{}{}", path, "sand.png"),format!("{}{}", path, "gravel.png"),format!("{}{}", path, "log_oak.png"),format!("{}{}", path, "log_oak_top.png"),format!("{}{}", path, "iron_block.png"),format!("{}{}", path, "gold_block.png"),format!("{}{}", path, "diamond_block.png"),format!("{}{}", path, "emerald_block.png"),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "mushroom_red.png"),format!("{}{}", path, "mushroom_brown.png"),format!("{}{}", path, "sapling_jungle.png"),"blank.png".to_string(),format!("{}{}", path, "gold_ore.png"),format!("{}{}", path, "iron_ore.png"),format!("{}{}", path, "coal_ore.png"),format!("{}{}", path, "bookshelf.png"),format!("{}{}", path, "cobblestone_mossy.png"),format!("{}{}", path, "obsidian.png"),format!("{}{}", path, "grass_side_overlay.png"),format!("{}{}", path, "tallgrass.png"),format!("{}{}", path, "grass_top.png"),format!("{}{}", path, "beacon.png"),"blank.png".to_string(),format!("{}{}", path, "crafting_table_top.png"),format!("{}{}", path, "furnace_front_off.png"),format!("{}{}", path, "furnace_side.png"),format!("{}{}", path, "dispenser_front_horizontal.png"),"blank.png".to_string(),format!("{}{}", path, "sponge.png"),format!("{}{}", path, "glass.png"),format!("{}{}", path, "diamond_ore.png"),format!("{}{}", path, "redstone_ore.png"),format!("{}{}", path, "leaves_oak.png"),format!("{}{}", path, "leaves_oak_opaque.png"),format!("{}{}", path, "stonebrick.png"),format!("{}{}", path, "deadbush.png"),format!("{}{}", path, "fern.png"),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "crafting_table_side.png"),format!("{}{}", path, "crafting_table_front.png"),format!("{}{}", path, "furnace_front_on.png"),format!("{}{}", path, "furnace_top.png"),format!("{}{}", path, "sapling_spruce.png"),format!("{}{}", path, "wool_colored_white.png"),format!("{}{}", path, "mob_spawner.png"),format!("{}{}", path, "snow.png"),format!("{}{}", path, "ice.png"),format!("{}{}", path, "grass_side_snowed.png"),format!("{}{}", path, "cactus_top.png"),format!("{}{}", path, "cactus_side.png"),format!("{}{}", path, "cactus_bottom.png"),format!("{}{}", path, "clay.png"),format!("{}{}", path, "reeds.png"),format!("{}{}", path, "jukebox_side.png"),format!("{}{}", path, "jukebox_top.png"),format!("{}{}", path, "waterlily.png"),format!("{}{}", path, "mycelium_side.png"),format!("{}{}", path, "mycelium_top.png"),format!("{}{}", path, "sapling_birch.png"),format!("{}{}", path, "torch_on.png"),format!("{}{}", path, "door_wood_upper.png"),format!("{}{}", path, "door_iron_upper.png"),format!("{}{}", path, "ladder.png"),format!("{}{}", path, "trapdoor.png"),format!("{}{}", path, "iron_bars.png"),format!("{}{}", path, "farmland_wet.png"),format!("{}{}", path, "farmland_dry.png"),format!("{}{}", path, "wheat_stage_0.png"),format!("{}{}", path, "wheat_stage_1.png"),format!("{}{}", path, "wheat_stage_2.png"),format!("{}{}", path, "wheat_stage_3.png"),format!("{}{}", path, "wheat_stage_4.png"),format!("{}{}", path, "wheat_stage_5.png"),format!("{}{}", path, "wheat_stage_6.png"),format!("{}{}", path, "wheat_stage_7.png"),format!("{}{}", path, "lever.png"),format!("{}{}", path, "door_wood_lower.png"),format!("{}{}", path, "door_iron_lower.png"),format!("{}{}", path, "redstone_torch_on.png"),format!("{}{}", path, "stonebrick_mossy.png"),format!("{}{}", path, "stonebrick_cracked.png"),format!("{}{}", path, "pumpkin_top.png"),format!("{}{}", path, "netherrack.png"),format!("{}{}", path, "soul_sand.png"),format!("{}{}", path, "glowstone.png"),format!("{}{}", path, "piston_top_sticky.png"),format!("{}{}", path, "piston_top_normal.png"),format!("{}{}", path, "piston_side.png"),format!("{}{}", path, "piston_bottom.png"),format!("{}{}", path, "piston_inner.png"),format!("{}{}", path, "pumpkin_stem_disconnected.png"),format!("{}{}", path, "rail_normal_turned.png"),format!("{}{}", path, "wool_colored_black.png"),format!("{}{}", path, "wool_colored_gray.png"),format!("{}{}", path, "redstone_torch_off.png"),format!("{}{}", path, "log_spruce.png"),format!("{}{}", path, "log_birch.png"),format!("{}{}", path, "pumpkin_side.png"),format!("{}{}", path, "pumpkin_face_off.png"),format!("{}{}", path, "pumpkin_face_on.png"),format!("{}{}", path, "cake_top.png"),format!("{}{}", path, "cake_side.png"),format!("{}{}", path, "cake_inner.png"),format!("{}{}", path, "cake_bottom.png"),format!("{}{}", path, "mushroom_block_skin_red.png"),format!("{}{}", path, "mushroom_block_skin_brown.png"),format!("{}{}", path, "pumpkin_stem_connected.png"),format!("{}{}", path, "rail_normal.png"),format!("{}{}", path, "wool_colored_red.png"),format!("{}{}", path, "wool_colored_pink.png"),format!("{}{}", path, "repeater_off.png"),format!("{}{}", path, "leaves_spruce.png"),format!("{}{}", path, "leaves_spruce_opaque.png"),format!("{}{}", path, "bed_feet_top.png"),format!("{}{}", path, "bed_head_top.png"),format!("{}{}", path, "melon_side.png"),format!("{}{}", path, "melon_top.png"),format!("{}{}", path, "cauldron_top.png"),format!("{}{}", path, "cauldron_inner.png"),"blank.png".to_string(),format!("{}{}", path, "mushroom_block_skin_stem.png"),format!("{}{}", path, "mushroom_block_inside.png"),format!("{}{}", path, "vine.png"),format!("{}{}", path, "lapis_block.png"),format!("{}{}", path, "wool_colored_green.png"),format!("{}{}", path, "wool_colored_lime.png"),format!("{}{}", path, "repeater_on.png"),format!("{}{}", path, "glass_pane_top.png"),format!("{}{}", path, "bed_feet_end.png"),format!("{}{}", path, "bed_feet_side.png"),format!("{}{}", path, "bed_head_side.png"),format!("{}{}", path, "bed_head_end.png"),format!("{}{}", path, "log_jungle.png"),format!("{}{}", path, "cauldron_side.png"),format!("{}{}", path, "cauldron_bottom.png"),format!("{}{}", path, "brewing_stand_base.png"),format!("{}{}", path, "brewing_stand.png"),format!("{}{}", path, "endframe_top.png"),format!("{}{}", path, "endframe_side.png"),format!("{}{}", path, "lapis_ore.png"),format!("{}{}", path, "wool_colored_brown.png"),format!("{}{}", path, "wool_colored_yellow.png"),format!("{}{}", path, "rail_golden.png"),format!("{}{}", path, "redstone_dust_dot.png"),format!("{}{}", path, "redstone_dust_line0.png"),format!("{}{}", path, "enchanting_table_top.png"),format!("{}{}", path, "dragon_egg.png"),format!("{}{}", path, "cocoa_stage_2.png"),format!("{}{}", path, "cocoa_stage_1.png"),format!("{}{}", path, "cocoa_stage_0.png"),format!("{}{}", path, "emerald_ore.png"),format!("{}{}", path, "trip_wire_source.png"),format!("{}{}", path, "trip_wire.png"),format!("{}{}", path, "endframe_eye.png"),format!("{}{}", path, "end_stone.png"),format!("{}{}", path, "sandstone_top.png"),format!("{}{}", path, "wool_colored_blue.png"),format!("{}{}", path, "wool_colored_light_blue.png"),format!("{}{}", path, "rail_golden_powered.png"),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "enchanting_table_side.png"),format!("{}{}", path, "enchanting_table_bottom.png"),format!("{}{}", path, "command_block_back.png"),format!("{}{}", path, "itemframe_background.png"),format!("{}{}", path, "flower_pot.png"),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "sandstone_normal.png"),format!("{}{}", path, "wool_colored_purple.png"),format!("{}{}", path, "wool_colored_magenta.png"),format!("{}{}", path, "rail_detector.png"),format!("{}{}", path, "leaves_jungle.png"),format!("{}{}", path, "leaves_jungle_opaque.png"),format!("{}{}", path, "planks_spruce.png"),format!("{}{}", path, "planks_jungle.png"),format!("{}{}", path, "carrots_stage_0.png"),format!("{}{}", path, "carrots_stage_1.png"),format!("{}{}", path, "carrots_stage_2.png"),format!("{}{}", path, "carrots_stage_3.png"),format!("{}{}", path, "potatoes_stage_3.png"),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "sandstone_bottom.png"),format!("{}{}", path, "wool_colored_cyan.png"),format!("{}{}", path, "wool_colored_orange.png"),format!("{}{}", path, "redstone_lamp_off.png"),format!("{}{}", path, "redstone_lamp_on.png"),format!("{}{}", path, "stonebrick_carved.png"),format!("{}{}", path, "planks_birch.png"),format!("{}{}", path, "anvil_base.png"),format!("{}{}", path, "anvil_top_damaged_1.png"),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "nether_brick.png"),format!("{}{}", path, "wool_colored_silver.png"),format!("{}{}", path, "nether_wart_stage_0.png"),format!("{}{}", path, "nether_wart_stage_1.png"),format!("{}{}", path, "nether_wart_stage_2.png"),format!("{}{}", path, "sandstone_carved.png"),format!("{}{}", path, "sandstone_smooth.png"),format!("{}{}", path, "anvil_top_damaged_0.png"),format!("{}{}", path, "anvil_top_damaged_2.png"),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),format!("{}{}", path, "destroy_stage_0.png"),format!("{}{}", path, "destroy_stage_1.png"),format!("{}{}", path, "destroy_stage_2.png"),format!("{}{}", path, "destroy_stage_3.png"),format!("{}{}", path, "destroy_stage_4.png"),format!("{}{}", path, "destroy_stage_5.png"),format!("{}{}", path, "destroy_stage_6.png"),format!("{}{}", path, "destroy_stage_7.png"),format!("{}{}", path, "destroy_stage_8.png"),format!("{}{}", path, "destroy_stage_9.png"),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string(),"blank.png".to_string()];
    let mut terrain = RgbaImage::new(256,256);
    RgbaImage::new(16, 16).save("blank.png").unwrap();
    create_missing_texture();
    let mut i: usize = 0;

    for y in (0..256).step_by(16) {
        for x in (0..256).step_by(16) {
            let texture = image::open(&names[i]).unwrap_or(image::open("missing_texture.png").unwrap()).to_rgba8();
            terrain.copy_from(&texture, x, y).expect("Image does not fit into new Image");
            terrain.save(format!("{}{}", path, "terrain.png")).unwrap();
            i += 1;
        }
    }
    std::fs::remove_file("blank.png").expect("Failed to remove temp image \"blank.png\".");
    std::fs::remove_file("missing_texture.png").expect("Failed to remove temp image \"missing_texture.png\".");

    fn create_missing_texture() {
        let mut missing = RgbaImage::new(16,16);
        for x in 0..16 {
            for y in 0..16 {
                match (x, y) {
                    // Top Left
                    (0..=7, 0..=7) => {
                        missing.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                    },
                    // Top Right
                    (8..=16, 0..=7) => {
                        missing.put_pixel(x, y, image::Rgba([248, 0, 248, 255]));
                    },
                    // Bottom Left
                    (0..=7, 8..=16) => {
                        missing.put_pixel(x, y, image::Rgba([248, 0, 248, 255]));
                    },
                    // Bottom Right
                    (8..=16, 8..=16) => {
                        missing.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
                    },             
                    _ => println!("Something went wrong!")
                }
            }
        }
        missing.save("missing_texture.png").unwrap();
    }
}

pub fn convert_pack(from_type: u8, to_type: u8, file_copied: &str) {
    let mut description = "";
    let parsed: json::JsonValue;
    let data: String;
    if to_type == 1 {
        if from_type == 2 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.txt")).unwrap();
            description = &data.as_str();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.txt")).expect("Failed to remove temp image \"pack.txt\".");
        } else if from_type == 3 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/manifest.json")).unwrap();
            parsed = json::parse(&data.as_str()).unwrap();
            description = parsed["header"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/manifest.json")).expect("Failed to remove temp image \"manifest.json\".");
        }

        let pack = json::stringify_pretty(object!{
            "pack": {
                "description": description.trim(),
                "pack_format": 6
            }
        }, 4);
        let mut file = File::create(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
        writeln!(&mut file, "{}", &pack).unwrap();

    } else if to_type == 2 {
        if from_type == 1 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
            parsed = json::parse(&data.as_str()).unwrap();
            description = parsed["pack"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
        } else if from_type == 3 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/manifest.json")).unwrap();
            parsed = json::parse(&data.as_str()).unwrap();
            description = parsed["header"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/manifest.json")).expect("Failed to remove temp image \"manifest.json\".");
        }

        let mut file = File::create(format!("{}{}", &file_copied, "/pack.txt")).unwrap();
        writeln!(&mut file, "{}", &description.trim()).unwrap();

    } else if to_type == 3 {
        if from_type == 1 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
            parsed = json::parse(data.as_str()).unwrap();
            description = parsed["pack"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.mcmeta")).expect("Failed to remove temp image \"pack.mcmeta\".");
        } else if from_type == 2 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.txt")).unwrap();
            description = data.as_str();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.txt")).expect("Failed to remove temp image \"pack.txt\".");
        }

        let replaced = file_copied.replace(".zip", "").replace(".mcpack", "");
        let split: Vec<&str> = replaced.split("/").collect();
        let name = split[split.len() - 1];
        let manifest = json::stringify_pretty(object!{
            "pack_version": 1,
            "header": {
                "description": description.trim().clone(),
                "name": name,
                "uuid": Uuid::new_v4().to_string().as_str(),
                "version": [1, 0, 0],
                "min_engine_version": [1, 10, 1],
            },
            "modules": {
                "description": description.trim(),
                "type": "resource",
                "uuid": Uuid::new_v4().to_string().as_str(),
                "version": [1, 0, 0],
            }
        }, 4);
        let mut file = File::create(format!("{}{}", &file_copied, "/manifest.json")).unwrap();
        writeln!(&mut file, "{}", &manifest).unwrap();
    }
}

pub fn crop(from_type: u8, file_copied: &str) {
    if from_type == 1 {
        let path = format!("{}{}", file_copied, "/assets/minecraft/textures/");
        let textures = vec![format!("{}{}", path, "entity/steve.png"),format!("{}{}", path, "entity/zombie_pigman.png"),format!("{}{}", path, "entity/zombie/zombie.png")];
        
        let mut img = image::open(format!("{}{}", path, "blocks/command_block_back.png")).unwrap().to_rgba8();
        let cropped = img.sub_image(0, 0, 16, 16).to_image();
        cropped.save(format!("{}{}", path, "blocks/command_block_back.png")).unwrap();

        let mut img1 = image::open(format!("{}{}", path, "environment/moon_phases.png")).unwrap().to_rgba8();
        let cropped1 = img1.sub_image(0, 0, 32, 32).to_image();
        cropped1.save(format!("{}{}", path, "environment/moon_phases.png")).unwrap();

        for i in 0..textures.len() {
            let mut img2 = image::open(&textures[i]).unwrap();
            let cropped2 = img2.sub_image(0, 0, 64, 32).to_image();
            cropped2.save(&textures[i]).unwrap();
        }
    } else if from_type == 3 {
        let path = format!("{}{}", file_copied, "/textures/");
        let textures = vec![format!("{}{}", path, "entity/steve.png"),format!("{}{}", path, "entity/zombie_pigman.png"),format!("{}{}", path, "entity/zombie/zombie.png")];
        
        let mut img = image::open(format!("{}{}", path, "blocks/glass_pane_top.png")).unwrap().to_rgba8();
        let cropped = img.sub_image(7, 0, 9, 16).to_image();
        let mut pane = RgbaImage::new(16, 16);
        pane.copy_from(&cropped, 7, 0).expect("Could not copy image");
        pane.save(format!("{}{}", path, "blocks/glass_pane_top.png")).unwrap();

        let mut img1 = image::open(format!("{}{}", path, "environment/moon_phases.png")).unwrap().to_rgba8();
        let cropped1 = img1.sub_image(0, 0, 32, 32).to_image();
        cropped1.save(format!("{}{}", path, "environment/moon_phases.png")).unwrap();

        for i in 0..textures.len() {
            let mut img2 = image::open(&textures[i]).unwrap();
            let cropped2 = img2.sub_image(0, 0, 64, 32).to_image();
            cropped2.save(&textures[i]).unwrap();
        }
    }
    convert_bed(from_type, file_copied);
    tga_to_png(file_copied);
}

fn convert_bed(from_type: u8, file_copied: &str) {
    // enum Rotation {
    //     Ninety,
    //     OneEighty,
    //     TwoSeventy,
    // }    
    // if from_type == 1 {
    //     let path = format!("{}{}", file_copied, "/assets/minecraft/textures/");
        
    //     let mut img = image::open(format!("{}{}", path, "entity/bed/red.png")).unwrap();
    //     let bedl = img.sub_image(50, 3, 3, 3).to_image();
    //     let flipped = imageops::rotate180(&bedl);

    //     let data = vec![
    //         (6, 28, 16, 16, "entity/bed/bed_feet_top.png", Rotation::Ninety),
    //         (6, 6, 16, 16, "entity/bed/bed_head_top.png", Rotation::Ninety),
    //         (22, 28, 6, 16, "entity/bed/bed_feet_side.png", Rotation::Ninety),
    //         (22, 6, 6, 16, "entity/bed/bed_head_side.png", Rotation::Ninety),
    //         (22, 22, 16, 6, "entity/bed/bed_feet_end.png", Rotation::OneEighty),
    //         (6, 0, 16, 6, "entity/bed/bed_head_end.png", Rotation::OneEighty),
    //     ];
    //     data.iter().for_each(|(a, b, c, d, p, rotation)| {
    //         let mut img = image::open(format!("{}{}", path, p)).unwrap();
    //         let img_item = img.sub_image(*a, *b, *c, *d).to_image();
    //         let flipped = match rotation {
    //             Rotation::OneEighty => imageops::rotate180(&img_item),
    //             Rotation::Ninety => imageops::rotate90(&img_item),
    //             Rotation::TwoSeventy => imageops::rotate270(&img_item),
    //         };
    //         flipped.save(format!("{}{}", path, p)).unwrap();
    //     });
    // }   
    if from_type == 1 {
        // Move all this to a for loop
        let path = format!("{}{}", file_copied, "/assets/minecraft/textures/");

        let mut img = image::open(format!("{}{}", path, "entity/bed/red.png")).unwrap();
        let bedl = img.sub_image(50, 3, 3, 3).to_image();
        let flipped = imageops::rotate180(&bedl);

        let bedft = img.sub_image(6, 28, 16, 16).to_image();
        let flipped1 = imageops::rotate90(&bedft);
        flipped1.save(format!("{}{}", path, "entity/bed/bed_feet_top.png")).unwrap();

        let bedht = img.sub_image(6, 6, 16, 16).to_image();
        let flipped2 = imageops::rotate90(&bedht);
        flipped2.save(format!("{}{}", path, "entity/bed/bed_head_top.png")).unwrap();

        let bedfs = img.sub_image(22, 28, 6, 16).to_image();
        let mut bedfs1 = RgbaImage::new(16,16);
        bedfs1.copy_from(&bedfs, 7, 0).expect("Could not copy image");
        bedfs1.copy_from(&imageops::rotate270(&flipped), 13, 13).expect("Could not copy image");
        let flipped3 = imageops::rotate90(&bedfs1);
        flipped3.save(format!("{}{}", path, "entity/bed/bed_feet_side.png")).unwrap();

        let bedhs = img.sub_image(22, 6, 6, 16).to_image();
        let mut bedhs1 = RgbaImage::new(16,16);
        bedhs1.copy_from(&bedhs, 7, 0).expect("Could not copy image");
        bedhs1.copy_from(&imageops::rotate90(&flipped), 13, 0).expect("Could not copy image");
        let flipped4 = imageops::rotate90(&bedhs1);
        flipped4.save(format!("{}{}", path, "entity/bed/bed_head_side.png")).unwrap();

        let bedfe = img.sub_image(22, 22, 16, 6).to_image();
        let mut bedfe1 = RgbaImage::new(16,16);
        bedfe1.copy_from(&bedfe, 0, 3).expect("Could not copy image");
        bedfe1.copy_from(&flipped, 0, 0).expect("Could not copy image");
        bedfe1.copy_from(&imageops::rotate180(&flipped), 13, 0).expect("Could not copy image");
        let flipped5 = imageops::rotate180(&bedfe1);
        flipped5.save(format!("{}{}", path, "entity/bed/bed_feet_end.png")).unwrap();

        let bedhe = img.sub_image(6, 0, 16, 6).to_image();
        let mut bedhe1 = RgbaImage::new(16,16);
        bedhe1.copy_from(&bedhe, 0, 3).expect("Could not copy image");
        bedhe1.copy_from(&flipped, 0, 0).expect("Could not copy image");
        bedhe1.copy_from(&imageops::rotate180(&flipped), 13, 0).expect("Could not copy image");
        let flipped6 = imageops::rotate180(&bedhe1);
        flipped6.save(format!("{}{}", path, "entity/bed/bed_head_end.png")).unwrap();

    } else if from_type == 3 {
        // Move all this to a for loop
        let path = format!("{}{}", file_copied, "/textures/");

        let mut img = image::open(format!("{}{}", path, "entity/bed/red.png")).unwrap();
        let bedl = img.sub_image(15, 38, 3, 3).to_image();
        let flipped = imageops::rotate180(&bedl);

        let bedft = img.sub_image(6, 22, 16, 16).to_image();
        let flipped1 = imageops::rotate90(&bedft);
        flipped1.save(format!("{}{}", path, "entity/bed/bed_feet_top.png")).unwrap();

        let bedht = img.sub_image(6, 6, 16, 16).to_image();
        let flipped2 = imageops::rotate90(&bedht);
        flipped2.save(format!("{}{}", path, "entity/bed/bed_head_top.png")).unwrap();

        let bedfs = img.sub_image(22, 22, 6, 16).to_image();
        let mut bedfs1 = RgbaImage::new(16,16);
        bedfs1.copy_from(&bedfs, 7, 0).expect("Could not copy image");
        bedfs1.copy_from(&imageops::rotate270(&flipped), 13, 13).expect("Could not copy image");
        let flipped3 = imageops::rotate90(&bedfs1);
        flipped3.save(format!("{}{}", path, "entity/bed/bed_feet_side.png")).unwrap();

        let bedhs = img.sub_image(22, 6, 6, 16).to_image();
        let mut bedhs1 = RgbaImage::new(16,16);
        bedhs1.copy_from(&bedhs, 7, 0).expect("Could not copy image");
        bedhs1.copy_from(&imageops::rotate90(&flipped), 13, 0).expect("Could not copy image");
        let flipped4 = imageops::rotate90(&bedhs1);
        flipped4.save(format!("{}{}", path, "entity/bed/bed_head_side.png")).unwrap();

        let bedfe = img.sub_image(22, 0, 16, 6).to_image();
        let mut bedfe1 = RgbaImage::new(16,16);
        bedfe1.copy_from(&bedfe, 0, 3).expect("Could not copy image");
        bedfe1.copy_from(&flipped, 0, 0).expect("Could not copy image");
        bedfe1.copy_from(&imageops::rotate180(&flipped), 13, 0).expect("Could not copy image");
        let flipped5 = imageops::rotate180(&bedfe1);
        flipped5.save(format!("{}{}", path, "entity/bed/bed_feet_end.png")).unwrap();

        let bedhe = img.sub_image(6, 0, 16, 6).to_image();
        let mut bedhe1 = RgbaImage::new(16,16);
        bedhe1.copy_from(&bedhe, 0, 3).expect("Could not copy image");
        bedhe1.copy_from(&flipped, 0, 0).expect("Could not copy image");
        bedhe1.copy_from(&imageops::rotate180(&flipped), 13, 0).expect("Could not copy image");
        let flipped6 = imageops::rotate180(&bedhe1);
        flipped6.save(format!("{}{}", path, "entity/bed/bed_head_end.png")).unwrap();
    }
}

pub fn tga_to_png(file_copied: &str) {
    let path = format!("{}{}", file_copied, "/textures/");
    let tga_textures = vec![format!("{}{}", path, "entity/blaze.tga"),format!("{}{}", path, "entity/ghast/ghast_shooting.tga"),format!("{}{}", path, "entity/sheep/sheep.tga")];
    for i in 0..tga_textures.len() {
        let png_texture = tga_textures[i].replace(".tga", ".png");
        let tga: RgbaImage = image::open(&tga_textures[i]).unwrap().to_rgba8();
        let mut png = RgbaImage::new(tga.width(), tga.height());

        png.copy_from(&tga, 0, 0).expect("Could not copy image");
        png.save(png_texture).unwrap();
        std::fs::remove_file(&tga_textures[i]).expect("Failed to remove a image.");
    }
}

pub fn png_to_tga(file_copied: &str) {
    let path = format!("{}{}", file_copied, "/textures/");
    let png_textures = vec![format!("{}{}", path, "entity/blaze.png"),format!("{}{}", path, "entity/ghast/ghast_shooting.png"),format!("{}{}", path, "entity/sheep/sheep.png")];
    for i in 0..png_textures.len() {
        let tga_texture = png_textures[i].replace(".png", ".tga");
        let png: RgbaImage = image::open(&png_textures[i]).unwrap().to_rgba8();
        let mut tga = RgbaImage::new(png.width(), png.height());
        
        tga.copy_from(&png, 0, 0).expect("Could not copy image");
        tga.save(tga_texture).unwrap();
        std::fs::remove_file(&png_textures[i]).expect("Failed to remove a image.");
    }
}
