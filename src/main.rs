mod display;
mod manipulation;
mod jsons;

use display::args;
use manipulation::*;


fn main() {
    let (mut from_type, mut to_type) = (0u8, 0u8);
    let (mut from_pack, mut to_pack, mut path) = (display::ConverterTypes::None, display::ConverterTypes::None, String::from(""));
    let (mut file, mut file_copied) = ("", "");

    let (i , o, p) = args();
    let from_pack = i.unwrap();
    let to_pack = o.unwrap();
    let file = p.unwrap();
    let file_copied: String;
    if file.ends_with(".zip") || file.ends_with(".mcpack") {
        file_copied = file.replace(".zip", "-Converted").replace(".mcpack", "-Converted");
    } else {
        file_copied = format!("{}{}", file, "-Converted");
    }
    let path = format!("{}{}", &file_copied, "/");
    match from_pack {
        display::ConverterTypes::Texture => from_type = 2,
        display::ConverterTypes::Resource => from_type = 1,
        display::ConverterTypes::Bedrock => from_type = 3,
        _ => {}
    }
    match to_pack {
        display::ConverterTypes::Texture => to_type = 2,
        display::ConverterTypes::Resource => to_type = 1,
        display::ConverterTypes::Bedrock => to_type = 3,
        _ => {}
    }
    //println!("file {} file copied {}", file, file_copied);
    match (from_type, to_type) {
        (1, 2) => manipulation::resource_to_texture(&file.as_str(), &file_copied.as_str(), &from_type, &to_type, &from_pack, &to_pack, &path),
        (1, 3) => manipulation::resource_to_bedrock(&file.as_str(), &file_copied.as_str(), &from_type, &to_type, &from_pack, &to_pack, &path),
        (2, 1) => manipulation::texture_to_resource(&file.as_str(), &file_copied.as_str(), &from_type, &to_type, &from_pack, &to_pack, &path),
        (2, 3) => manipulation::texture_to_bedrock(&file.as_str(), &file_copied.as_str(), &from_type, &to_type, &from_pack, &to_pack, &path),
        (3, 1) => manipulation::bedrock_to_resource(&file.as_str(), &file_copied.as_str(), &from_type, &to_type, &from_pack, &to_pack, &path),
        (3, 2) => manipulation::bedrock_to_texture(&file.as_str(), &file_copied.as_str(), &from_type, &to_type, &from_pack, &to_pack, &path),
        _ => {}
    }
    //println!("input: {:?} output: {:?} path: {:?}", from_pack.to_string(), to_pack.to_string(), file);
}
