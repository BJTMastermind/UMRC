use json;

let get_json = json::parse(r#"
{
    "rename": [
        {
            "from": "pack_icon.png",
            "to": "pack.png"
        }
    ]
}
"#).unwrap();
        