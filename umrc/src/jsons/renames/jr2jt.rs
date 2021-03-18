use json;

let get_json = json::parse(r#"
{
    "rename": [
        {
            "from": "assets/minecraft/textures/painting",
            "to": "assets/minecraft/textures/art"
        },
        {
            "from": "assets/minecraft/textures/models/armor/chainmail_layer_1.png",
            "to": "assets/minecraft/textures/models/armor/chain_1.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/chainmail_layer_2.png",
            "to": "assets/minecraft/textures/models/armor/chain_2.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/diamond_layer_1.png",
            "to": "assets/minecraft/textures/models/armor/diamond_1.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/diamond_layer_2.png",
            "to": "assets/minecraft/textures/models/armor/diamond_2.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/gold_layer_1.png",
            "to": "assets/minecraft/textures/models/armor/gold_1.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/gold_layer_2.png",
            "to": "assets/minecraft/textures/models/armor/gold_2.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/iron_layer_1.png",
            "to": "assets/minecraft/textures/models/armor/iron_1.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/iron_layer_2.png",
            "to": "assets/minecraft/textures/models/armor/iron_2.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/leather_layer_1.png",
            "to": "assets/minecraft/textures/models/armor/cloth_1.png"
        },
        {
            "from": "assets/minecraft/textures/models/armor/leather_layer_2.png",
            "to": "assets/minecraft/textures/models/armor/cloth_2.png"
        },
        {
            "from": "assets/minecraft/textures/art/paintings_kristoffer_zetterstrand.png",
            "to": "assets/minecraft/textures/art/kz.png"
        },
        {
            "from": "assets/minecraft/textures/entity/arrow.png",
            "to": "assets/minecraft/textures/entity/arrows.png"
        },
        {
            "from": "assets/minecraft/textures/colormap/foliage.png",
            "to": "assets/minecraft/textures/colormap/foliagecolor.png"
        },
        {
            "from": "assets/minecraft/textures/colormap/grass.png",
            "to": "assets/minecraft/textures/colormap/grasscolor.png"
        },
        {
            "from": "assets/minecraft/textures/environment/moon_phases.png",
            "to": "assets/minecraft/textures/environment/moon.png"
        },
        {
            "from": "assets/minecraft/textures/lang/en_us.lang",
            "to": "assets/minecraft/textures/lang/en_US.lang"
        },
        {
            "from": "assets/minecraft/textures/entity/steve.png",
            "to": "assets/minecraft/textures/entity/char.png"
        },
        {
            "from": "assets/minecraft/textures/entity/zombie_pigman.png",
            "to": "assets/minecraft/textures/entity/pigzombie.png"
        },
        {
            "from": "assets/minecraft/textures/entity/minecart.png",
            "to": "assets/minecraft/textures/entity/cart.png"
        },
        {
            "from": "assets/minecraft/textures/entity/ghast/ghast_shooting.png",
            "to": "assets/minecraft/textures/entity/ghast/ghast_fire.png"
        },
        {
            "from": "assets/minecraft/textures/entity/pig/pig_saddle.png",
            "to": "assets/minecraft/textures/entity/pig/saddle.png"
        },
        {
            "from": "assets/minecraft/textures/entity",
            "to": "assets/minecraft/textures/mob"
        },
        {
            "from": "assets/minecraft/textures/gui/options_background.png",
            "to": "assets/minecraft/textures/gui/background.png"
        },
        {
            "from": "assets/minecraft/textures/gui/widgets.png",
            "to": "assets/minecraft/textures/gui/gui.png"
        },
        {
            "from": "assets/minecraft/textures/gui/container/crafting_table.png",
            "to": "assets/minecraft/textures/gui/container/crafting.png"
        },
        {
            "from": "assets/minecraft/textures/gui/container/dispenser.png",
            "to": "assets/minecraft/textures/gui/container/trap.png"
        },
        {
            "from": "assets/minecraft/textures/gui/container/generic_54.png",
            "to": "assets/minecraft/textures/gui/container/container.png"
        }
    ]
}
"#).unwrap();
        