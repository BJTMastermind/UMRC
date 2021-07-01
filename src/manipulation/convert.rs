use crate::manipulation::*;
use crate::display::*;

pub fn resource_to_texture(file: &str, file_copied: &str, from_type: &u8, to_type: &u8, from_pack: &ConverterTypes, to_pack: &ConverterTypes, path: &str) {
    copy(&file, &file_copied);
    tga_to_png(&file_copied);
    /*crop(from_type, file_copied);
    stitch(path);
    convert_pack(from_type, to_type, &file_copied);
    json("creates", "jr2bt", &path, from_type, to_type);
    json("renames", "jr2bt", &path, from_type, to_type);
    json("moves", "jr2bt", &path, from_type, to_type);
    json("deletes", "jr2bt", &path, from_type, to_type);
    zip(&file_copied, to_type);
    println!("Done!");*/
}

pub fn resource_to_bedrock(file: &str, file_copied: &str, from_type: &u8, to_type: &u8, from_pack: &ConverterTypes, to_pack: &ConverterTypes, path: &String) {
    
}

pub fn texture_to_resource(file: &str, file_copied: &str, from_type: &u8, to_type: &u8, from_pack: &ConverterTypes, to_pack: &ConverterTypes, path: &String) {
    unzip(&file);
    convert_pack(&from_type, &to_type, &file_copied);
    json("creates", "jt2jr", &path, &from_type, &to_type);
    json("renames", "jt2jr", &path, &from_type, &to_type);
    json("moves", "jt2jr", &path, &from_type, &to_type);
    json("deletes", "jt2jr", &path, &from_type, &to_type);
    unstitch(&format!("{}{}", &path, "assets/minecraft/textures/blocks/"), "terrain.png");
    //unstitch(&format!("{}{}", &path, "assets/minecraft/textures/items/"), "items.png");
    println!("Done!");
}

pub fn texture_to_bedrock(file: &str, file_copied: &str, from_type: &u8, to_type: &u8, from_pack: &ConverterTypes, to_pack: &ConverterTypes, path: &String) {
    
}

pub fn bedrock_to_resource(file: &str, file_copied: &str, from_type: &u8, to_type: &u8, from_pack: &ConverterTypes, to_pack: &ConverterTypes, path: &String) {
    
}

pub fn bedrock_to_texture(file: &str, file_copied: &str, from_type: &u8, to_type: &u8, from_pack: &ConverterTypes, to_pack: &ConverterTypes, path: &String) {
    
}