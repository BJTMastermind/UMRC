use json;
use json::JsonValue;

pub fn jr2jt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "create": [
        "pack.mcmeta",
        "terrain",
        "item",
        "title"
    ]
}
    "#).unwrap();
    get_json
}