use json;
use json::JsonValue;

pub fn jt2bt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "delete": [
        "terrain.png",
        "pack.txt",
        "lang",
        "title",
        "entity/pigman.png",
        "entity/spider_eyes.png",
        "item",
        "terrain",
        "gui"
    ]
}
    "#).unwrap();
    get_json
}