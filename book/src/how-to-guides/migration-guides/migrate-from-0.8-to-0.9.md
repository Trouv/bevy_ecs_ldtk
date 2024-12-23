# Migrate from 0.8 to 0.9

## Bevy upgrade
`bevy_ecs_ldtk` has upgraded to Bevy and `bevy_ecs_tilemap` version `0.12`.
A Bevy `0.12` migration guide is available on [Bevy's website](https://bevyengine.org/learn/migration-guides/0.11-0.12/).

## LDtk upgrade
`bevy_ecs_ldtk` now supports LDtk 1.5.3, and is dropping support for previous versions.
To update your game to LDtk 1.5.3, you should only need to install the new version of LDtk, open your project, and save it.

## `Default` behavior for `LdtkEntity` and `LdtkIntCell` derive macros
Fields on an `LdtkEntity`- or `LdtkIntCell`-derived bundle are no longer constructed from the field's `Default` implementation, but the bundle's.

You may observe different behavior in `0.9` if the value for a field in your bundle's `Default` implementation differs from the field type's own `Default` implementation:
```rust,ignore
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
#[derive(Component)]
struct MyComponent(usize);

impl Default for MyComponent {
    fn default() -> MyComponent {
        MyComponent(1)
    }
}

#[derive(Bundle, LdtkEntity)]
struct MyBundle {
    component: MyComponent,
}

impl Default for MyBundle {
    fn default() -> MyBundle {
        MyBundle {
            component: MyComponent(2),
        }
    }
}

// In bevy_ecs_ldtk 0.8, the plugin would spawn an entity w/ MyComponent(1)

// In bevy_ecs_ldtk 0.9, the plugin now spawns the entity w/ MyComponent(2)
```

You may also need to implement `Default` for `LdtkEntity` types that did not have that implementation before:
```rust,ignore
// 0.8
#[derive(Bundle, LdtkEntity)]
struct MyBundle {
    component: MyComponentThatImplementsDefault,
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
# #[derive(Default, Component)]
# struct MyComponentThatImplementsDefault;
// 0.9
#[derive(Default, Bundle, LdtkEntity)]
struct MyBundle {
    component: MyComponentThatImplementsDefault,
}
```

## Hierarchy of LDtk Entities
Layer entities (with a `LayerMetadata` component) are now spawned for LDtk Entity layers, just like any other layer.
By default, LDtk Entities are now spawned as children to these layer entities instead of as children of the level.
```rust,ignore
// 0.8
fn get_level_of_entity(
    entities: Query<Entity, With<EntityInstance>>,
    parent_query: Query<&Parent>,
) {
    for entity in &entities {
        println!(
            "the level that {:?} belongs to is {:?}",
            entity,
            parent_query.iter_ancestors(entity).nth(0)
        );
    }
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
// 0.9
fn get_level_of_entity(
    entities: Query<Entity, With<EntityInstance>>,
    parent_query: Query<&Parent>,
) {
    for entity in &entities {
        println!(
            "the level that {:?} belongs to is {:?}",
            entity,
            parent_query.iter_ancestors(entity).nth(1)
        );
    }
}
```

## Asset Type Rework
Most breaking changes in this release are related to the asset types, previously `LdtkAsset` and `LdtkLevel`.
These types have been heavily reworked to improve code quality, correctness, performance, and provide better APIs.

### `LdtkAsset` is now `LdtkProject`, and other changes
`LdtkAsset` has now been renamed to `LdtkProject`.
Any types and systems that depend on this type will need to be updated accordingly:
```rust,ignore
// 0.8
fn do_some_processing_with_ldtk_data(
    worlds: Query<&Handle<LdtkAsset>>,
    ldtk_assets: Res<Assets<LdtkAsset>>
) {
    // do something
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
// 0.9
fn do_some_processing_with_ldtk_data(
    worlds: Query<&Handle<LdtkProject>>,
    ldtk_assets: Res<Assets<LdtkProject>>
) {
    // do something
}
```

Furthermore, all of its fields have been privatized, and are now only available via immutable accessor methods.
Not all of these methods share the same name as their corresponding field in `0.8`:
```rust,ignore
// 0.8
let ldtk_json = ldtk_project.project;
let tileset_map = ldtk_project.tileset_map;
let int_grid_image_handle = ldtk_project.int_grid_image_handle;
let level_map = ldtk_project.level_map;
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# fn foo(ldtk_project: LdtkProject) {
// 0.9
let ldtk_json = ldtk_project.json_data();
let tileset_map = ldtk_project.tileset_map();
let int_grid_image_handle = ldtk_project.int_grid_image_handle();
// the level_map is no longer available in the same way
# }
```

### `LdtkAsset` and `LdtkJson` level accessor methods have been moved
Level accessing methods have been completely redefined.
Analogues to existing methods have been renamed and moved to traits:
```rust,ignore
// 0.8
ldtk_json.iter_levels();

ldtk_asset.iter_levels();

ldtk_asset.get_level(&LevelSelection::Uid(24));
```
```rust,ignore
# use bevy_ecs_ldtk::{prelude::*, ldtk::LdtkJson};
# fn foo(ldtk_json: LdtkJson, ldtk_project: LdtkProject) {
// 0.9
// in `RawLevelAccessor` trait:
ldtk_json.iter_raw_levels();

ldtk_project.iter_raw_levels();

// in `LevelMetadataAccessor` trait
ldtk_project.find_raw_level_by_level_selection(&LevelSelection::Uid(24));
# }
```

Many new methods have been provided as well.

### Internal-levels and external-levels support now behind separate features
There are two new cargo features, `internal_levels` and `external_levels`.
`internal_levels` is enabled by default and allows loading of internal-levels LDtk projects.
`external_levels` is not enabled by default and allows loading of external-levels LDtk projects.
Some APIs are unique to the two cases.

If you have an LDtk project with internal levels, but have disabled default features, you will need to enable `internal_levels`:
```toml
# 0.8
bevy_ecs_ldtk = { version = "0.8", default-features = false, features = ["render"] }

# 0.9
bevy_ecs_ldtk = { version = "0.9", default-features = false, features = ["render", "internal_levels"] }
```

If you have an LDtk project with external levels, you will need to enable `external_levels`:
```toml
# 0.8
bevy_ecs_ldtk = "0.8"

# 0.9
bevy_ecs_ldtk = { version = "0.9", features = ["external_levels"] }
```

These features are **not** mutually exclusive, but at least one of them must be enabled.

### Level Asset Changes
The level asset type has changed significantly.
Most importantly, it is no longer the primary mechanism for storing loaded level data.
In fact, it is only compiled and used within the `external_levels` feature (see previous section).

#### Level entities now have a `LevelIid` instead of a `Handle<LdtkLevel>`
The level asset it is no longer the main component marking level entities.
In both internal-levels and external-levels projects, level entities will no longer have a handle to the level asset, but instead will have a `LevelIid` component:
```rust,ignore
// 0.8
fn print_level_entity(levels: Query<Entity, With<Handle<LdtkLevel>>>) {
    for entity in &levels {
        println!("level entity {:?} is currently spawned", entity);
    }
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
// 0.9
fn print_level_entity(levels: Query<Entity, With<LevelIid>>) {
    for entity in &levels {
        println!("level entity {:?} is currently spawned", entity);
    }
}
```

#### Accessing level data from the level entity
Retrieving level data from the level entity can be done using the `LevelIid` component.
If the data you need *is not* inside the level's `layer_instances`, you can access it on the `LdtkProject` asset:
```rust,ignore
// 0.8
fn print_level_uid(levels: Query<Handle<LdtkLevel>>, level_assets: Res<Assets<LdtkLevel>>) {
    for level_handle in &levels {
        let level_uid = level_assets.get(level_handle).unwrap().uid;
        println!("level w/ uid {level_uid}, is currently spawned");
    }
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
// 0.9
fn print_level_uid(
    levels: Query<&LevelIid>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>
) {
    for level_iid in &levels {
        let only_project = project_assets.get(projects.single()).unwrap();

        let level_uid = only_project.get_raw_level_by_iid(level_iid.get()).unwrap().uid;
        println!("level w/ uid {level_uid}, is currently spawned");
    }
}
```

If the level data you need *is* inside the level's `layer_instances`, you may want to retrieve a `LoadedLevel`.
A `Level` might not have complete data - in the case that it's the "raw" level inside an external-levels project's `LdtkProject` asset.
This new `LoadedLevel` type provides type guarantees that the level has complete data.
For internal-levels (aka "standalone") projects, you can retrieve loaded level data with a `LevelIid` and `LdtkProject` alone:
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
// 0.9, w/ internal_levels enabled
fn print_level_uid(
    levels: Query<&LevelIid>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>
) {
    for level_iid in &levels {
        let only_project = project_assets.get(projects.single()).unwrap();

        let layer_count = only_project
            .as_standalone()
            .get_loaded_level_by_iid(level_iid.get())
            .unwrap()
            .layer_instances()
            .len();
        println!("level has {layer_count} layers");
    }
}
```

For external-levels (aka "parent") projects, you will need to additionally access the `LdtkExternalLevel` asset store:
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
// 0.9, w/ external_levels enabled
fn print_level_uid(
    levels: Query<&LevelIid>,
    projects: Query<&Handle<LdtkProject>>,
    project_assets: Res<Assets<LdtkProject>>
    level_assets: Res<Assets<LdtkExternalLevel>>,
) {
    for level_iid in &levels {
        let only_project = project_assets.get(projects.single()).unwrap();

        let layer_count = only_project
            .as_parent()
            .get_external_level_by_iid(&level_assets, level_iid.get())
            .unwrap()
            .layer_instances()
            .len();
        println!("level has {layer_count} layers");
    }
}
```

### Module restructure
Some types related to assets have been removed, or privatized, or moved.

Those that were removed/privatized were generally not intended to be used by users:
- `LevelMap`
- `TilesetMap`
- `LdtkLevelLoader`
- `LdtkLoader`

Those that were moved have been moved into the `assets` module, and are still exposed in the `prelude`:
- `LdtkProject`
- `LdtkExternalLevel`

## `LevelIid` everywhere
`LevelIid` is a new component on level entities that stores the level's iid as a string.
It has been reused throughout the API.

### In `LevelSet`
`LevelSet` uses it, but can still be constructed from strings using `from_iids`:
```rust,ignore
// 0.8
let level_set = LevelSet {
    iids: [
        "e5eb2d73-60bb-4779-8b33-38a63da8d1db".to_string(),
        "855fab73-2854-419f-a3c6-4ed8466592f6".to_string(),
    ].into_iter().collect(),
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# fn f() {
// 0.9
let level_set = LevelSet::from_iids(
    [
        "e5eb2d73-60bb-4779-8b33-38a63da8d1db",
        "855fab73-2854-419f-a3c6-4ed8466592f6",
    ]
);
# }
```

### In `LevelEvent`
```rust,ignore
use std::any::{Any, TypeId};
// 0.8
fn assert_level_event_type(mut level_events: EventReader<LevelEvent>) {
    for level_event in level_events.iter() {
        use LevelEvent::*;
        let level_iid = match level_event {
            SpawnTriggered(level_iid) | Spawned(level_iid) | Transformed(level_iid) | Despawned(level_iid) => level_iid,
        };

        assert_eq!(level_iid.type_id(), TypeId::of::<String>());
    }
}
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# use bevy::prelude::*;
use std::any::{Any, TypeId};
// 0.9
fn assert_level_event_type(mut level_events: EventReader<LevelEvent>) {
    for level_event in level_events.read() {
        use LevelEvent::*;
        let level_iid = match level_event {
            SpawnTriggered(level_iid) | Spawned(level_iid) | Transformed(level_iid) | Despawned(level_iid) => level_iid,
        };

        assert_eq!(level_iid.type_id(), TypeId::of::<LevelIid>());
    }
}
```

### In `LevelSelection::Iid`
`LevelSelection` uses it, but can still be constructed with a string via the `iid` method:
```rust,ignore
// 0.8
let level_selection = LevelSelection::Iid("e5eb2d73-60bb-4779-8b33-38a63da8d1db".to_string());
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# fn f() {
// 0.9
let level_selection = LevelSelection::iid("e5eb2d73-60bb-4779-8b33-38a63da8d1db");
# }
```

## `LevelSelection` index variant now stores a world index
The `LevelSelection::Index` variant has been replaced by `LevelSelection::Indices`.
Internally, this contains a new `LevelIndices` type, which stores an optional world index in addition to the level index.
However, you can still construct a `LevelSelection` from a single level index using the `index` method:
```rust,ignore
// 0.8
let level_selection = LevelSelection::Index(2);
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# fn f() {
// 0.9
let level_selection = LevelSelection::index(2);
# }
```

## `LevelSet::from_iid` replaced with `LevelSet::from_iids`
`LevelSet::from_iid` has been replaced by `LevelSet::from_iids`.
This new method can accept any iterator of strings rather than just one:
```rust,ignore
// 0.8
let level_set = LevelSet::from_iid("e5eb2d73-60bb-4779-8b33-38a63da8d1db");
```
```rust,ignore
# use bevy_ecs_ldtk::prelude::*;
# fn f() {
// 0.9
let level_set = LevelSet::from_iids(["e5eb2d73-60bb-4779-8b33-38a63da8d1db"]);

// or many..
let level_set = LevelSet::from_iids(
    [
        "e5eb2d73-60bb-4779-8b33-38a63da8d1db",
        "855fab73-2854-419f-a3c6-4ed8466592f6",
    ]
);
# }
```
