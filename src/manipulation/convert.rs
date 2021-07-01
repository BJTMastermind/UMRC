use crate::manipulation::*;

pub fn resource_to_texture(file: &str, file_copied: &str, from_type: u8, to_type: u8, from_pack: &str, to_pack: &str, path: &str) {
    copy(&file, &file_copied);
    tga_to_png(&file_copied);
    /*crop(from_type, file_copied);
    stitch(path);
    convert_pack(from_type, to_type, &file_copied);
    json
    json
    json
    json
    zip(&file_copied, to_type);*/
}

pub fn resource_to_bedrock() {
    
}

pub fn texture_to_resource() {
    
}

pub fn texture_to_bedrock() {
    
}

pub fn bedrock_to_resource() {
    
}

pub fn bedrock_to_texture() {
    
}