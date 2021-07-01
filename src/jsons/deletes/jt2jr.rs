use json;
use json::JsonValue;

pub fn jt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "delete": [
        "font",
        "achievement",
        "item",
        "terrain",
        "assets/minecraft/textures/gui/particles.png"
    ]
}
    "#).unwrap();
    get_json
}