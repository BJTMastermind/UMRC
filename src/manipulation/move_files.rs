use std::{path, fs};

pub fn move_file(from: &String, to: &String) -> std::io::Result<()> {
    fs::copy(from, to).expect("Failed to copy file to new location.");
    fs::remove_file(from).expect("Failed to delete original file.");
    Ok(())
}

pub fn move_dir(from: &String, to: &String) -> std::io::Result<()> {
    copy_dir(from, to).unwrap_or({});
    fs::remove_dir_all(from).expect("Failed to delete original directory.");
    Ok(())
}

fn copy_dir(from: impl AsRef<path::Path>, to: impl AsRef<path::Path>) -> std::io::Result<()> {
    fs::create_dir_all(&to)?;
    for entry in fs::read_dir(from)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), to.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), to.as_ref().join(entry.file_name()))?;
            //move_file(&entry.file_name().into_string().unwrap(), &to.as_ref().to_str().unwrap().to_string()).unwrap();
        }
    }
    Ok(())
}