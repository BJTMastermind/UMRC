use json;

let get_json = json::parse(r#"
{
    "rename": [
        {}
    ]
}
"#).unwrap();
        