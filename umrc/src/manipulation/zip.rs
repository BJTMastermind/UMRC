use std::{fs::File, path::PathBuf};
use zip::ZipWriter;
use zip_extensions::{write::ZipWriterExtensions, zip_extract};

pub fn zip(file_copied: &str, to_type: u8) {
    let file: File;
    if to_type == 3 {
        file = File::create(format!("{}{}", file_copied, ".mcpack")).unwrap();
    } else {
        file = File::create(format!("{}{}", file_copied, ".zip")).unwrap();
    }
    let mut zip = ZipWriter::new(file);
    zip.create_from_directory(&PathBuf::from(&file_copied)).unwrap();
}

pub fn unzip(file_copied: &str) {
    if file_copied.ends_with(".zip") || file_copied.ends_with(".mcpack") {
        let archive = PathBuf::from(&file_copied);
        let file = PathBuf::from(&file_copied.replace(".zip", "").replace(".mcpack", ""));
        zip_extract(&archive, &file).unwrap();
    } else {
        println!("File given is a directory not a .zip or .mcpack!");
    }
}