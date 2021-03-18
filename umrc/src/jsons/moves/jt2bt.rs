use json;

let get_json = json::parse(r#"
{
    "move": [
        {
            "to": "textures/models/armor",
            "from": "armor"
        },
        {
            "to": "textures/entity",
            "from": "entity"
        },
        {
            "to": "textures/environment",
            "from": "environment"
        },
        {
            "to": "textures/misc",
            "from": "misc"
        },
        {
            "to": "textures/painting",
            "from": "painting"
        },
        {
            "to": "textures/entity",
            "from": "item/arrows.png"
        },
        {
            "to": "textures/entity",
            "from": "item/minecart.png"
        },
        {
            "to": "textures/entity",
            "from": "item/sign.png"
        },
        {
            "to": "textures/environment",
            "from": "terrain/sun.png"
        },
        {
            "to": "textures/particle",
            "from": "particles.png"
        },
        {
            "to": "textures/colormap",
            "from": "textures/misc/foliage.png"
        },
        {
            "to": "textures/colormap",
            "from": "textures/misc/grass.png"
        },
        {
            "to": "textures/entity/cow",
            "from": "textures/entity/cow.png"
        },
        {
            "to": "textures/entity/creeper",
            "from": "textures/entity/creeper.png"
        },
        {
            "to": "textures/entity/ghast",
            "from": "textures/entity/ghast.png"
        },
        {
            "to": "textures/entity/ghast",
            "from": "textures/entity/ghast_shooting.png"
        },
        {
            "to": "textures/entity/pig",
            "from": "textures/entity/pig.png"
        },
        {
            "to": "textures/entity/pig",
            "from": "textures/entity/saddle.png"
        },
        {
            "to": "textures/entity/pig",
            "from": "textures/entity/pigzombie.png"
        },
        {
            "to": "textures/entity/sheep",
            "from": "textures/entity/sheep.png"
        },
        {
            "to": "textures/entity/sheep",
            "from": "textures/entity/sheep_fur.png"
        },
        {
            "to": "textures/entity/skeleton",
            "from": "textures/entity/skeleton.png"
        },
        {
            "to": "textures/entity/slime",
            "from": "textures/entity/slime.png"
        },
        {
            "to": "textures/entity/spider",
            "from": "textures/entity/spider.png"
        },
        {
            "to": "textures/entity/zombie",
            "from": "textures/entity/zombie.png"
        },
        {
            "to": "textures/misc",
            "from": "gui/unknown_pack.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_0.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_1.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_2.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_3.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_4.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_5.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_6.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_7.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_8.png"
        },
        {
            "to": "textures/environment",
            "from": "textures/blocks/destroy_stage_9.png"
        }
    ]
}
"#).unwrap();
        