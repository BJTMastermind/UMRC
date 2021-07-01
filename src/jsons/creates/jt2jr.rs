use json;
use json::JsonValue;

pub fn jt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "create": [
        "entity/cow",
        "entity/creeper",
        "entity/ghast",
        "entity/pig",
        "entity/projectiles",
        "entity/sheep",
        "entity/skeleton",
        "entity/slime",
        "entity/spider",
        "entity/zombie",
        "gui/container",
        "textures/models",
        "textures/colormap",
        "textures/particle"
    ]
}
    "#).unwrap();
    get_json
}