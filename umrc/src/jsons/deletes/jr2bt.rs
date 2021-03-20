use json;
use json::JsonValue;

pub fn jr2bt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "delete": []
}
    "#).unwrap();
    get_json
}