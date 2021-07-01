use json;
use json::JsonValue;

pub fn jt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "move": [
        {
            "to": "assets/minecraft/textures/models/armor",
            "from": "armor"
        },
        {
            "to": "assets/minecraft/textures/entity",
            "from": "entity"
        },
        {
            "to": "assets/minecraft/textures/environment",
            "from": "environment"
        },
        {
            "to": "assets/minecraft/textures/gui",
            "from": "gui"
        },
        {
            "to": "assets/minecraft/lang",
            "from": "lang"
        },
        {
            "to": "assets/minecraft/textures/misc",
            "from": "misc"
        },
        {
            "to": "assets/minecraft/textures/painting",
            "from": "painting"
        },
        {
            "to": "assets/minecraft/textures/gui/title",
            "from": "title"
        },
        {
            "to": "assets/minecraft/textures/entity/projectiles/arrow.png",
            "from": "item/arrow.png"
        },
        {
            "to": "assets/minecraft/textures/entity/minecart.png",
            "from": "item/minecart.png"
        },
        {
            "to": "assets/minecraft/textures/entity/sign.png",
            "from": "item/sign.png"
        },
        {
            "to": "assets/minecraft/textures/environment/sun.png",
            "from": "terrain/sun.png"
        },
        {
            "to": "assets/minecraft/textures/particle/particles.png",
            "from": "particles.png"
        },
        {
            "to": "assets/minecraft/textures/colormap/foliage.png",
            "from": "assets/minecraft/textures/misc/foliage.png"
        },
        {
            "to": "assets/minecraft/textures/colormap/grass.png",
            "from": "assets/minecraft/textures/misc/grass.png"
        },
        {
            "to": "assets/minecraft/textures/entity/cow/cow.png",
            "from": "assets/minecraft/textures/entity/cow.png"
        },
        {
            "to": "assets/minecraft/textures/entity/creeper/creeper.png",
            "from": "assets/minecraft/textures/entity/creeper.png"
        },
        {
            "to": "assets/minecraft/textures/entity/ghast/ghast.png",
            "from": "assets/minecraft/textures/entity/ghast.png"
        },
        {
            "to": "assets/minecraft/textures/entity/ghast/ghast_shooting.png",
            "from": "assets/minecraft/textures/entity/ghast_shooting.png"
        },
        {
            "to": "assets/minecraft/textures/entity/pig/pig.png",
            "from": "assets/minecraft/textures/entity/pig.png"
        },
        {
            "to": "assets/minecraft/textures/entity/pig/saddle.png",
            "from": "assets/minecraft/textures/entity/saddle.png"
        },
        {
            "to": "assets/minecraft/textures/entity/sheep/sheep.png",
            "from": "assets/minecraft/textures/entity/sheep.png"
        },
        {
            "to": "assets/minecraft/textures/entity/sheep/sheep_fur.png",
            "from": "assets/minecraft/textures/entity/sheep_fur.png"
        },
        {
            "to": "assets/minecraft/textures/entity/skeleton/skeleton.png",
            "from": "assets/minecraft/textures/entity/skeleton.png"
        },
        {
            "to": "assets/minecraft/textures/entity/slime/slime.png",
            "from": "assets/minecraft/textures/entity/slime.png"
        },
        {
            "to": "assets/minecraft/textures/entity/spider/spider.png",
            "from": "assets/minecraft/textures/entity/spider.png"
        },
        {
            "to": "assets/minecraft/textures/entity/zombie/zombie.png",
            "from": "assets/minecraft/textures/entity/zombie.png"
        },
        {
            "to": "assets/minecraft/textures/gui/container/crafting_table.png",
            "from": "assets/minecraft/textures/gui/crafting_table.png"
        },
        {
            "to": "assets/minecraft/textures/gui/container/dispenser.png",
            "from": "assets/minecraft/textures/gui/dispenser.png"
        },
        {
            "to": "assets/minecraft/textures/gui/container/furnace.png",
            "from": "assets/minecraft/textures/gui/furnace.png"
        },
        {
            "to": "assets/minecraft/textures/gui/container/generic_54.png",
            "from": "assets/minecraft/textures/gui/generic_54.png"
        },
        {
            "to": "assets/minecraft/textures/gui/container/inventory.png",
            "from": "assets/minecraft/textures/gui/inventory.png"
        },
        {
            "to": "assets/minecraft/textures/misc/unknown_pack.png",
            "from": "assets/minecraft/textures/gui/unknown_pack.png"
        },
        {
            "to": "assets/minecraft/textures/blocks/terrain.png",
            "from": "terrain.png"
        },
        {
            "to": "assets/minecraft/textures/items/items.png",
            "from": "assets/minecraft/textures/gui/items.png"
        }
    ]
}
    "#).unwrap();
    get_json
}