use json;
use json::JsonValue;

pub fn jr2jt() -> JsonValue {
    let get_json = json::parse(r#"
{
    "move": [
        {
            "from": "environment/moon.png",
            "to": "terrain"
        },
        {
            "from": "environment/sun.png",
            "to": "terrain"
        },
        {
            "from": "mob/arrows.png",
            "to": "item"
        },
        {
            "from": "mob/cart.png",
            "to": "item"
        },
        {
            "from": "mob/sign.png",
            "to": "item"
        },
        {
            "from": "textures/colormap/foliagecolor.png",
            "to": "misc"
        },
        {
            "from": "textures/colormap/grasscolor.png",
            "to": "misc"
        },
        {
            "from": "gui/container/crafting.png",
            "to": "gui"
        },
        {
            "from": "gui/container/furnace.png",
            "to": "gui"
        },
        {
            "from": "gui/container/container.png",
            "to": "gui"
        },
        {
            "from": "gui/container/inventory.png",
            "to": "gui"
        },
        {
            "from": "gui/container/trap.png",
            "to": "gui"
        },
        {
            "from": "gui/title/mojang.png",
            "to": "title"
        },
        {
            "from": "misc/unknown_pack.png",
            "to": "gui"
        },
        {
            "from": "mob/cow/cow.png",
            "to": "mob"
        },
        {
            "from": "mob/creeper/creeper.png",
            "to": "mob"
        },
        {
            "from": "mob/ghast/ghast.png",
            "to": "mob"
        },
        {
            "from": "mob/ghast/ghast_fire.png",
            "to": "mob"
        },
        {
            "from": "mob/pig/pig.png",
            "to": "mob"
        },
        {
            "from": "mob/pig/saddle.png",
            "to": "mob"
        },
        {
            "from": "mob/sheep/sheep.png",
            "to": "mob"
        },
        {
            "from": "mob/sheep/sheep_fur.png",
            "to": "mob"
        },
        {
            "from": "mob/skeleton/skeleton.png",
            "to": "mob"
        },
        {
            "from": "mob/slime/slime.png",
            "to": "mob"
        },
        {
            "from": "mob/spider/spider.png",
            "to": "mob"
        },
        {
            "from": "mob/zombie/zombie.png",
            "to": "mob"
        },
        {
            "from": "lang",
            "to": "/"
        },
        {
            "from": "textures/models/armor",
            "to": "/"
        },
        {
            "from": "textures/art",
            "to": "/"
        },
        {
            "from": "textures/environment",
            "to": "/"
        },
        {
            "from": "textures/gui",
            "to": "/"
        },
        {
            "from": "textures/misc",
            "to": "/"
        },
        {
            "from": "textures/mob",
            "to": "/"
        },
        {
            "from": "textures/particle/particles.png",
            "to": "/"
        }
    ]
}
    "#).unwrap();
    get_json
}