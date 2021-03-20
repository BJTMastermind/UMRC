use json;
use json::JsonValue;

pub fn bt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "create": [
        "assets",
        "assets/minecraft"
    ]
}
    "#).unwrap();
    get_json
}