use json;
use json::JsonValue;

pub fn bt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "move": [
        {
            "from": "textures",
            "to": "assets/minecraft/textures"
        }
    ]
}
    "#).unwrap();
    get_json
}