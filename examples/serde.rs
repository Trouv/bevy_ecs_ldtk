use bevy_ecs_ldtk::ldtk::LdtkJson;
use std::fs;

fn main() {
    let ldtk: LdtkJson = serde_json::from_reader(
        fs::File::open("assets/Typical_2D_platformer_example.ldtk").unwrap(),
    )
    .unwrap();

    println!("{}", serde_json::to_string_pretty(&ldtk).unwrap());
}
