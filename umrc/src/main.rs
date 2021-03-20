mod display;
mod manipulation;
mod jsons;
mod move_files;

use std::fs::File;
use display::args;
use manipulation::*;
use json::*;


fn main() {
    let (mut from_type, mut to_type, mut return_val) = (0u8, 0u8, 0u8);
    let (mut from_pack, mut to_pack, mut path) = ("", "", "");
    let (mut file, mut file_copied) = ("", "");
    //convert_pack(1, 2, "/home/bjtmastermind/Desktop/texture_test");
    //let (i , o, p) = args();
    //println!("input: {:?} output: {:?} path: {:?}", i, o, p);
}
