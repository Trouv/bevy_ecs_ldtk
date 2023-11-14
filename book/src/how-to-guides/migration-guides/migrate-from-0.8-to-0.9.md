# Migrate from 0.8 to 0.9

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

