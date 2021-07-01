use crate::manipulation::move_files::*;
use crate::jsons::*;

use json::JsonValue;

pub fn json(source: &str, file: &str, file_copied: &str, from_type: u8, to_type: u8) {
    let root_folder = format!("{}{}", file_copied, "/");

    match source {
        "renames" => {
            let rename: JsonValue = get_json_file(source, file).unwrap()["rename"].clone();
            for i in 0..rename.len() {
                let from = format!("{}{}{}", root_folder, "/", rename[i]["from"].to_string());
                let to = format!("{}{}{}", root_folder, "/", rename[i]["to"].to_string());
                std::fs::rename(from, to).unwrap();
            }
        },
        "moves" => {
            let moves: JsonValue = get_json_file(source, file).unwrap()["move"].clone();
            for i in 0..moves.len() {
                let from = format!("{}{}{}", root_folder, "/", moves[i]["from"].to_string());
                let to = format!("{}{}{}", root_folder, "/", moves[i]["to"].to_string());
                move_dir(from.as_str(), to.as_str()).unwrap();
            }
        },
        "deletes" => {
            let delete: JsonValue = get_json_file(source, file).unwrap()["delete"].clone();
            for i in 0..delete.len() {
                let remove = format!("{}{}{}", root_folder, "/", delete[i].to_string());
                std::fs::remove_dir_all(remove).unwrap_or({});
            }
        },
        "creates" => {
            let create: JsonValue = get_json_file(source, file).unwrap()["create"].clone();
            for i in 0..create.len() {
                let make = create[i].to_string();
                std::fs::create_dir(format!("{}{}{}", root_folder, "/", make)).unwrap();
            }
            if from_type == 2 && to_type == 1 {
                std::fs::rename(format!("{}{}", root_folder, ".zip"), root_folder).unwrap();
            }
        },
        _ => println!("Error: Could not find json file selected.")
    }
}

fn get_json_file(source: &str, file: &str) -> Option<JsonValue> {
    match (source, file) {
        ("creates", "bt2jr") => Some(creates::bt2jr()),
        ("creates", "bt2jt") => Some(creates::bt2jt()),
        ("creates", "jr2bt") => Some(creates::jr2bt()),
        ("creates", "jr2jt") => Some(creates::jr2jt()),
        ("creates", "jt2bt") => Some(creates::jt2bt()),
        ("creates", "jt2jr") => Some(creates::jt2jr()),
        ("deletes", "bt2jr") => Some(deletes::bt2jr()),
        ("deletes", "bt2jt") => Some(deletes::bt2jt()),
        ("deletes", "jr2bt") => Some(deletes::jr2bt()),
        ("deletes", "jr2jt") => Some(deletes::jr2jt()),
        ("deletes", "jt2bt") => Some(deletes::jt2bt()),
        ("deletes", "jt2jr") => Some(deletes::jt2jr()),
        ("moves", "bt2jr") => Some(moves::bt2jr()),
        ("moves", "bt2jt") => Some(moves::bt2jt()),
        ("moves", "jr2bt") => Some(moves::jr2bt()),
        ("moves", "jr2jt") => Some(moves::jr2jt()),
        ("moves", "jt2bt") => Some(moves::jt2bt()),
        ("moves", "jt2jr") => Some(moves::jt2jr()),
        ("renames", "bt2jr") => Some(renames::bt2jr()),
        ("renames", "bt2jt") => Some(renames::bt2jt()),
        ("renames", "jr2bt") => Some(renames::jr2bt()),
        ("renames", "jr2jt") => Some(renames::jr2jt()),
        ("renames", "jt2bt") => Some(renames::jt2bt()),
        ("renames", "jt2jr") => Some(renames::jt2jr()),
        _ => None
    }
}