# Game Logic Integration
Loading LDtk levels into Bevy doesn't get you very far if you cannot play them.

Aside from rendering tilemaps, LDtk has features for placing gameplay objects on Entity layers.
Even within tilemaps, IntGrid layers imply a categorization of tiles, and perhaps a game designerly meaning.
It is fundamental to associate the LDtk entities and IntGrid tiles with Bevy entities/components.
`bevy_ecs_ldtk` is designed around a couple core strategies for doing so, which will be discussed here.

## `LdtkEntity` and `LdtkIntCell` registration
The `LdtkEntity`/`LdtkIntCell` registration API allows you to hook custom bevy `Bundle`s into the level spawning process.
You define what components you want on the entity with a bundle, define how they should be constructed with the `LdtkEntity` or `LdtkIntCell` derive, and register the bundle to the `App` for a given LDtk entity identifier, or IntGrid value.

```rust,no_run
use bevy::prelude::*;
use bevy_ecs_ldtk::prelude::*;

fn main() {
    App::new()
        // other App builders
        .register_ldtk_entity::<PlayerBundle>("Player")
        .run();
}

#[derive(Default, Component)]
struct Player;

#[derive(Default, Bundle, LdtkEntity)]
struct PlayerBundle {
    player: Player,
    #[sprite_bundle]
    sprite_bundle: SpriteBundle,
}
```

How does `LdtkEntity`/`LdtkIntCell` construct the bundle when derived?
Without any intervention, the bundle's fields are constructed using the bundle's `Default` implementation.
However, various attributes are available to override this behavior, like `#[sprite_bundle]` in the above example.
This attribute gives the entity a sprite using the tileset in its LDtk editor visual.
For documentation about all the available attributes, check out the API reference for these traits:
- [`LdtkEntity`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/app/trait.LdtkEntity.html) <!-- x-release-please-version -->
- [`LdtkIntCell`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/app/trait.LdtkIntCell.html) <!-- x-release-please-version -->

This approach is suitable for many common, simple use cases.
There's also room for more granular, component-level customization within some of the attributes, like `#[with(...)]` or `#[from_entity_instance]`.
Of course, the traits can also be manually implemented for the even-more custom cases.

## Post-processing plugin-spawned entities
There are still many cases where `LdtkEntity`/`LdtkIntCell` registration is insufficient.
Perhaps you need to spawn children of the entity, or need access to more resources in the `World`.
For these more demanding cases, post-processing plugin-spawned entities in a custom system is always an option.

If an LDtk entity does not have a matching `LdtkEntity` registration, it will be spawned with an `EntityInstance` component by default.
This component contains the raw LDtk data for that entity.
Querying for newly-spawned `EntityInstance` entities can be a good starting point for implementing your own custom spawning logic.
Intgrid tiles have similar behavior, except their default component is `IntGridCell`, which simply contains the IntGrid value for that tile.

```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
#[derive(Default, Component)]
struct PlayerChild;

#[derive(Default, Component)]
struct Player;

fn process_player(
    mut commands: Commands,
    new_entity_instances: Query<(Entity, &EntityInstance, &Transform), Added<EntityInstance>>,
    assets: Res<AssetServer>,
)
{
    for (entity, entity_instance, transform) in new_entity_instances.iter() {
        if entity_instance.identifier == "Player".to_string() {
            commands
                .entity(entity)
                .insert(Player)
                .insert(SpriteBundle {
                    texture: assets.load("player.png"),
                    transform: *transform,
                    ..default()
                })
                .with_children(|commands| {
                    commands.spawn(PlayerChild);
                });
        }
    }
}
```

This approach makes spawning entities from LDtk just as powerful and customizable as a Bevy system, because that's all it is.
`LdtkEntity` and `LdtkIntCell` ultimately make some assumptions about what data from the LDtk asset and the Bevy world you will need to spawn your entity, which post-processing avoids.
However, there are some pretty obvious ergonomics issues to this strategy compared to using registration:
- You need to manually filter `EntityInstance`s for the desired LDtk entity identifier.
- You need to manually perform the iteration of the query.
- If you need the associated layer data, or tileset image, or tileset definition, you need to manually access these assets.
- You need to be careful not to overwrite the plugin-provided `Transform` component.

## A combined approach - the blueprint pattern
