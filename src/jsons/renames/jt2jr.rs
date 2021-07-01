use json;
use json::JsonValue;

pub fn jt2jr() -> JsonValue {
    let get_json = json::parse(r#"
{
    "rename": [
        {
            "from": "armor/chain_1.png",
            "to": "armor/chainmail_layer_1.png"
        },
        {
            "from": "armor/chain_2.png",
            "to": "armor/chainmail_layer_2.png"
        },
        {
            "from": "armor/cloth_1.png",
            "to": "armor/leather_layer_1.png"
        },
        {
            "from": "armor/cloth_2.png",
            "to": "armor/leather_layer_2.png"
        },
        {
            "from": "armor/cloth_1_b.png",
            "to": "armor/leather_layer_1_overlay.png"
        },
        {
            "from": "armor/cloth_2_b.png",
            "to": "armor/leather_layer_2_overlay.png"
        },
        {
            "from": "armor/diamond_1.png",
            "to": "armor/diamond_layer_1.png"
        },
        {
            "from": "armor/diamond_2.png",
            "to": "armor/diamond_layer_2.png"
        },
        {
            "from": "armor/gold_1.png",
            "to": "armor/gold_layer_1.png"
        },
        {
            "from": "armor/gold_2.png",
            "to": "armor/gold_layer_2.png"
        },
        {
            "from": "armor/iron_1.png",
            "to": "armor/iron_layer_1.png"
        },
        {
            "from": "armor/iron_2.png",
            "to": "armor/iron_layer_2.png"
        },
        {
            "from": "art/kz.png",
            "to": "art/paintings_kristoffer_zetterstrand.png"
        },
        {
            "from": "art",
            "to": "painting"
        },
        {
            "from": "gui/background.png",
            "to": "gui/options_background.png"
        },
        {
            "from": "gui/container.png",
            "to": "gui/generic_54.png"
        },
        {
            "from": "gui/crafting.png",
            "to": "gui/crafting_table.png"
        },
        {
            "from": "gui/gui.png",
            "to": "gui/widgets.png"
        },
        {
            "from": "gui/trap.png",
            "to": "gui/dispenser.png"
        },
        {
            "from": "item/cart.png",
            "to": "item/minecart.png"
        },
        {
            "from": "lang/en_US.lang",
            "to": "lang/en_us.lang"
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
            "from": "item/arrows.png",
            "to": "item/arrow.png"
        },
        {
            "from": "mob/pigzombie.png",
            "to": "mob/zombie_pigman.png"
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