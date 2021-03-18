use json;

let get_json = json::parse(r#"
{
    "create": [
        "assets",
        "assets/minecraft"
    ]
}
"#).unwrap();
        