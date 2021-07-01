use json;
use json::JsonValue;

pub fn jt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "create": [
        "mob/cow",
        "mob/creeper",
        "mob/ghast",
        "mob/pig",
        "mob/projectiles",
        "mob/sheep",
        "mob/skeleton",
        "mob/slime",
        "mob/spider",
        "mob/zombie",
        "gui/container",
        "assets",
        "assets/minecraft",
        "assets/minecraft/textures",
        "assets/minecraft/textures/blocks",
        "assets/minecraft/textures/items",
        "assets/minecraft/textures/models",
        "assets/minecraft/textures/colormap",
        "assets/minecraft/textures/particle"
    ]
}
    "#).unwrap();
    get_json
}