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
        .register_ldtk_entity::<MyBundle>("My Entity Identifier")
        .run();
}

#[derive(Default, Component)]
struct MyComponent;

#[derive(Default, Bundle, LdtkEntity)]
struct MyBundle {
    my_component: MyComponent,
    #[sprite_sheet_bundle]
    sprite_sheet_bundle: SpriteSheetBundle,
}
```

How does `LdtkEntity`/`LdtkIntCell` construct the bundle?


## Post-processing plugin-added entities

## A combined approach - the blueprint pattern

[`LdtkEntity`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/app/trait.LdtkEntity.html) <!-- x-release-please-version -->
[`LdtkIntCell`](https://docs.rs/bevy_ecs_ldtk/0.8.0/bevy_ecs_ldtk/app/trait.LdtkIntCell.html) <!-- x-release-please-version -->

