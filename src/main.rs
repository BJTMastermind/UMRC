mod display;
mod manipulation;
mod jsons;

use std::fs::File;
use display::args;
use manipulation::*;


fn main() {
    let (mut from_type, mut to_type, mut return_val) = (0u8, 0u8, 0u8);
    let (mut from_pack, mut to_pack, mut path) = ("", "", "");
    let (mut file, mut file_copied) = ("", "");
    //unzip("/home/bjtmastermind/Desktop/texture_test.zip");
    //let (i , o, p) = args();
    //println!("input: {:?} output: {:?} path: {:?}", i, o, p);
}
