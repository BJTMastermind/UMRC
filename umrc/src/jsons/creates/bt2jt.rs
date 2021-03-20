use json;
use json::JsonValue;

pub fn bt2jt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "create": [
        "terrain",
        "item",
        "title"
    ]
}
    "#).unwrap();
    get_json
}