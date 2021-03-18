use json;

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
        