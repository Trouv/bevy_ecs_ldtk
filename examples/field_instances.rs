use bevy::{prelude::*, render::texture::DEFAULT_IMAGE_HANDLE};
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(LdtkPlugin)
        .insert_resource(LevelSelection::default())
        .add_startup_system(setup)
        .register_ldtk_entity::<EntityWithFieldsBundle>("EntityWithFields")
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let ldtk_handle = asset_server.load("field_instances.ldtk");
    let map_entity = commands.spawn_empty().id();

    commands.entity(map_entity).insert(LdtkWorldBundle {
        ldtk_handle,
        ..Default::default()
    });
}

#[derive(Clone, Default, Bundle)]
struct EntityWithFieldsBundle {
    #[bundle]
    sprite_bundle: SpriteBundle,
}

impl LdtkEntity for EntityWithFieldsBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> EntityWithFieldsBundle {
        println!("EntityWithFields added, here are some facts:");
        for field_instance in &entity_instance.field_instances {
            println!(
                "    its {} {}",
                field_instance.identifier,
                explain_field(&field_instance.value)
            );
        }

        let mut sprite = Sprite {
            custom_size: Some(Vec2::splat(16.)),
            ..Default::default()
        };
        if let Some(color_field) = entity_instance
            .field_instances
            .iter()
            .find(|f| f.identifier == *"Color")
        {
            if let FieldValue::Color(color) = color_field.value {
                sprite.color = color;
            }
        }

        EntityWithFieldsBundle {
            sprite_bundle: SpriteBundle {
                sprite,
                texture: DEFAULT_IMAGE_HANDLE.typed(),
                ..Default::default()
            },
        }
    }
}

fn explain_field(value: &FieldValue) -> String {
    match value {
        FieldValue::Int(Some(i)) => format!("has an integer of {}", i),
        FieldValue::Float(Some(f)) => format!("has a float of {}", f),
        FieldValue::Bool(b) => format!("is {}", b),
        FieldValue::String(Some(s)) => format!("says {}", s),
        FieldValue::Color(c) => format!("has the color {:?}", c),
        FieldValue::Enum(Some(e)) => format!("is the variant {}", e),
        FieldValue::FilePath(Some(f)) => format!("references {}", f),
        FieldValue::Point(Some(p)) => format!("is at ({}, {})", p.x, p.y),
        a => format!("is hard to explain: {:?}", a),
    }
}
