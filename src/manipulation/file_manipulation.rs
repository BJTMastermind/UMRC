use std::{path::Path, fs, fs::File, io::Write};
use image::{GenericImage, RgbaImage, imageops};
use json::{object, JsonValue};
use crate::jsons::*;
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

pub fn unstitch(path: &String, image: &str) {
    let names = textures::get_terrain()["textures"].clone();
    // if image == "terrain.png" {
    //     names = textures::get_terrain()["textures"];
    // } else {
    //     names = vec![format!("{}{}", path, "leather_helmet.png"),format!("{}{}", path, "chainmail_helmet.png"),format!("{}{}", path, "iron_helmet.png"),format!("{}{}", path, "diamond_helmet.png"),format!("{}{}", path, "gold_helmet.png"),format!("{}{}", path, "flint_and_steel.png"),format!("{}{}", path, "flint.png"),format!("{}{}", path, "coal.png"),format!("{}{}", path, "string.png"),format!("{}{}", path, "wheat_seeds.png"),format!("{}{}", path, "apple.png"),format!("{}{}", path, "golden_apple.png"),format!("{}{}", path, "egg.png"),format!("{}{}", path, "suger.png"),format!("{}{}", path, "snowball.png"),format!("{}{}", path, "empty_helmet_slot.png"),format!("{}{}", path, "gold_helmet.png")];
    // }
    let mut base = image::open(format!("{}{}", path, image)).unwrap();
    let mut i: usize = 0;

    for y in (0..256).step_by(16) {
        for x in (0..256).step_by(16) {
            if names[i].to_string() != "none".to_string() {
                let texture: RgbaImage = base.sub_image(x, y, 16, 16).to_image();
                texture.save(format!("{}{}", path, &names[i].to_string())).unwrap();
            }
            i += 1;
        }
    }
    std::fs::remove_file(format!("{}{}", path, image)).expect("Failed to remove texture atlus");
}

pub fn stitch(path: &String) {
    let names = textures::get_terrain()["textures"].clone();
    let mut terrain = RgbaImage::new(256,256);
    RgbaImage::new(16, 16).save("blank.png").unwrap();
    create_missing_texture();
    let mut i: usize = 0;

    for y in (0..256).step_by(16) {
        for x in (0..256).step_by(16) {
            let texture: RgbaImage;
            if names[i].to_string() != "none".to_string() {
                texture = image::open(format!("{}{}", path, names[i].to_string())).unwrap_or(image::open("missing_texture.png").unwrap()).to_rgba8();
            } else {
                texture = image::open("blank.png").unwrap().to_rgba8();
            }
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

pub fn convert_pack(from_type: &u8, to_type: &u8, file_copied: &str) {
    let mut description = "";
    let parsed: json::JsonValue;
    let data: String;
    if *to_type == 1 {
        if *from_type == 2 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.txt")).unwrap();
            description = &data.as_str();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.txt")).expect("Failed to remove temp image \"pack.txt\".");
        } else if *from_type == 3 {
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

    } else if *to_type == 2 {
        if *from_type == 1 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
            parsed = json::parse(&data.as_str()).unwrap();
            description = parsed["pack"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
        } else if *from_type == 3 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/manifest.json")).unwrap();
            parsed = json::parse(&data.as_str()).unwrap();
            description = parsed["header"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/manifest.json")).expect("Failed to remove temp image \"manifest.json\".");
        }

        let mut file = File::create(format!("{}{}", &file_copied, "/pack.txt")).unwrap();
        writeln!(&mut file, "{}", &description.trim()).unwrap();

    } else if *to_type == 3 {
        if *from_type == 1 {
            data = fs::read_to_string(format!("{}{}", &file_copied, "/pack.mcmeta")).unwrap();
            parsed = json::parse(data.as_str()).unwrap();
            description = parsed["pack"]["description"].as_str().unwrap();
            std::fs::remove_file(format!("{}{}", &file_copied, "/pack.mcmeta")).expect("Failed to remove temp image \"pack.mcmeta\".");
        } else if *from_type == 2 {
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

pub fn crop(from_type: &u8, file_copied: &str) {
    if *from_type == 1 {
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
    } else if *from_type == 3 {
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

fn convert_bed(from_type: &u8, file_copied: &str) {
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
    if *from_type == 1 {
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

    } else if *from_type == 3 {
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
