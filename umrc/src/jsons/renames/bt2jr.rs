use json;
use json::JsonValue;

pub fn bt2jr() -> JsonValue {
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
    get_json
}