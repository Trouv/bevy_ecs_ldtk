# Migrate from 0.8 to 0.9

## LDtk upgrade
`bevy_ecs_ldtk` now supports LDtk 1.4.1, and is dropping support for previous versions.
To update your game to LDtk 1.4.1, you should only need to install the new version of LDtk, open your project, and save it.

## `Default` behavior for `LdtkEntity` and `LdtkIntCell` derive macros
Fields on an `LdtkEntity`- or `LdtkIntCell`-derived bundle are no longer constructed from the field's `Default` implementation, but the bundle's.

You may observe different behavior in `0.9` if the value for a field in your bundle's `Default` implementation differs from the field type's own `Default` implementation:
```rust,ignore
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

// 0.9
#[derive(Default, Bundle, LdtkEntity)]
struct MyBundle {
    component: MyComponentThatImplementsDefault,
}
```

## Ancestors of LDtk Entities
Layer entities (with a `LayerMetadata` component) are now spawned for LDtk Entity layers.
By default, LDtk Entities are now spawned as children to these layer entities instead of as children of the level.
```rust,ignore
fn get_level_of_entity(
    entities: Query<Entity, With<EntityInstance>>,
    parent_query: Query<&Parent>,
) {
    for entity in &entities {
        // 0.8
        println!(
            "the level that {:?} belongs to is {:?}",
            entity,
            parent_query.iter_ancestors(entity).nth(0)
        );

        // 0.9
        println!(
            "the level that {:?} belongs to is {:?}",
            entity,
            parent_query.iter_ancestors(entity).nth(1)
        );
    }
}
