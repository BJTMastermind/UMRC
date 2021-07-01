use json;
use json::JsonValue;

pub fn jt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "delete": [
        "terrain.png",
        "pack.txt",
        "item",
        "terrain"
    ]
}
    "#).unwrap();
    get_json
}