use json;
use json::JsonValue;

pub fn jt2bt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "rename": [
        {
            "from": "pack.png",
            "to": "pack_icon.png"
        },
        {
            "from": "art",
            "to": "painting"
        },
        {
            "from": "item/cart.png",
            "to": "item/minecart.png"
        },
        {
            "from": "misc/foliagecolor.png",
            "to": "misc/foliage.png"
        },
        {
            "from": "misc/grasscolor.png",
            "to": "misc/grass.png"
        },
        {
            "from": "mob/char.png",
            "to": "mob/steve.png"
        },
        {
            "from": "mob/ghast_fire.png",
            "to": "mob/ghast_shooting.png"
        },
        {
            "from": "mob",
            "to": "entity"
        }
    ]
}
    "#).unwrap();
    get_json
}