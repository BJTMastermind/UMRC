use json;

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
        