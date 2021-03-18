use json;

let get_json = json::parse(r#"
{
    "rename": [
        {
            "from": "textures/painting",
            "to": "textures/art"
        },
        {
            "from": "textures/colormap/foliage.png",
            "to": "textures/colormap/foliagecolor.png"
        },
        {
            "from": "textures/colormap/grass.png",
            "to": "textures/colormap/grasscolor.png"
        },
        {
            "from": "textures/environment/moon_phases.png",
            "to": "textures/environment/moon.png"
        },
        {
            "from": "textures/entity/steve.png",
            "to": "textures/entity/char.png"
        },
        {
            "from": "textures/entity/minecart.png",
            "to": "textures/entity/cart.png"
        },
        {
            "from": "textures/entity/ghast/ghast_shooting.tga",
            "to": "textures/entity/ghast/ghast_fire.tga"
        },
        {
            "from": "textures/entity/pig/pig_saddle.png",
            "to": "textures/entity/pig/saddle.png"
        },
        {
            "from": "textures/entity/sheep/sheep.png",
            "to": "textures/entity/sheep_fur.png"
        },
        {
            "from": "textures/entity",
            "to": "textures/mob"
        },
        {
            "from": "pack_icon.png",
            "to": "pack.png"
        },
        {
            "from": "texts",
            "to": "lang"
        }
    ]
}
"#).unwrap();
        