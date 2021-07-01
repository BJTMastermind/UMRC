use clap::{Arg, App};
use ConverterTypes::*;

#[derive(Debug, PartialEq)]
#[derive(Copy, Clone)]
pub enum ConverterTypes {
    Texture,
    Resource,
    Bedrock,
    None,
}
impl std::fmt::Display for ConverterTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string_display = match self {
            Self::Texture => "Java Texture Pack",
            Self::Resource => "Java Resource Pack",
            Self::Bedrock => "Bedrock Resource Pack",
            Self::None => "None",
        };
        write!(f, "{}", string_display)
    }
}

pub fn args() -> (Option<ConverterTypes>, Option<ConverterTypes>, Option<String>) {
    let matches = App::new("Universal Minecraft Resource Converter")
        .version("0.1.0")
        .author("BJTMastermind")
        .about("Converts a Minecraft resource pack from one version to another.")
        .arg(Arg::with_name("input")
            .short("i")
            .multiple(false)
            .takes_value(true)
            .value_name("texture | resource | bedrock")
            .help("Gives the program a input file."))
        .arg(Arg::with_name("output")
            .short("o")
            .multiple(false)
            .takes_value(true)
            .value_name("texture | resource | bedrock")
            .help("Gives the program a output directory."))
        .arg(Arg::with_name("file")
            .short("f")
            .multiple(false)
            .takes_value(true)
            .value_name("FILE")).get_matches();

    let in_type = match matches.value_of("input").unwrap_or(" ") {
        "texture" => Some(Texture),
        "resource" => Some(Resource),
        "bedrock" => Some(Bedrock),
        _ => Some(None),
    };
    let out_type = match matches.value_of("output").unwrap_or(" ") {
        "texture" => Some(Texture),
        "resource" => Some(Resource),
        "bedrock" => Some(Bedrock),
        _ => Some(None),
    };
    let pack_path = matches.value_of("file");
    
    if matches.occurrences_of("input") == 1 || matches.occurrences_of("output") == 1 || matches.occurrences_of("file") == 1 {
        if matches.occurrences_of("input") < 1 || matches.occurrences_of("output") < 1 || matches.occurrences_of("file") < 1 {
            println!("You must enter all arguments or no arguments!");
            std::process::exit(0);
        }
    }
    if in_type == out_type {
        println!("Input and Ouput types can't be the same!");
        std::process::exit(0);
    }
    (in_type, out_type, pack_path.map(ToString::to_string))
}