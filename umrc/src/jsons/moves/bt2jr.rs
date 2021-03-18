use json;

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
        