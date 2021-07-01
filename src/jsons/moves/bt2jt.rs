use json;
use json::JsonValue;

pub fn bt2jt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "move": [
        {
            "to": "terrain",
            "from": "environment/moon.png"
        },
        {
            "to": "terrain",
            "from": "environment/sun.png"
        },
        {
            "to": "item",
            "from": "mob/arrows.png"
        },
        {
            "to": "item",
            "from": "mob/cart.png"
        },
        {
            "to": "item",
            "from": "mob/sign.png"
        },
        {
            "to": "misc",
            "from": "colormap/foliagecolor.png"
        },
        {
            "to": "misc",
            "from": "colormap/grasscolor.png"
        },
        {
            "to": "mob",
            "from": "mob/cow/cow.png"
        },
        {
            "to": "mob",
            "from": "mob/creeper/creeper.png"
        },
        {
            "to": "mob",
            "from": "mob/ghast/ghast.png"
        },
        {
            "to": "mob",
            "from": "mob/ghast/ghast_fire.tga"
        },
        {
            "to": "mob",
            "from": "mob/pig/pig.png"
        },
        {
            "to": "mob",
            "from": "mob/sheep/sheep_fur.png"
        },
        {
            "to": "mob",
            "from": "mob/skeleton/skeleton.png"
        },
        {
            "to": "mob",
            "from": "mob/slime/slime.png"
        },
        {
            "to": "mob",
            "from": "mob/spider/spider.png"
        },
        {
            "to": "mob",
            "from": "mob/zombie/zombie.png"
        },
        {
            "from": "models/armor",
            "to": "/"
        },
        {
            "from": "art",
            "to": "/"
        },
        {
            "from": "environment",
            "to": "/"
        },
        {
            "from": "gui",
            "to": "/"
        },
        {
            "from": "misc",
            "to": "/"
        },
        {
            "from": "mob",
            "to": "/"
        },
        {
            "from": "particle/particles.png",
            "to": "/"
        }
    ]
}
    "#).unwrap();
    get_json
}