use json;

let get_json = json::parse(r#"
{
    "create": [
        "textures",
        "entity/cow",
        "entity/creeper",
        "entity/ghast",
        "entity/pig",
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
        