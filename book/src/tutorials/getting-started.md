# Getting Started

The goal of this plugin is to make it as easy as possible to use LDtk with Bevy
for common use cases, while providing solutions to handle more difficult cases.
You only need a few things to get started:
1. Add the `LdtkPlugin` to the `App`
2. Insert the `LevelSelection` resource into the `App` to pick your level
3. Spawn an `LdtkWorldBundle`
4. Optionally, use `#[derive(LdtkEntity)]` and `#[derive(LdtkIntCell)]` on
   bundles and register them to the `App` to automatically spawn those bundles
   on Entity and IntGrid layers.

```rust,no_run
{{ #include ../../../examples/basic.rs }}
```

There are other attributes available to `#[derive(LdtkEntity)]` and `#[derive(LdtkIntCell)]`, see the documentation for more details.

By default, LDtk Entities and IntGrid tiles get spawned with `EntityInstance`
and `IntGridCell` components respectfully.
So, you can flesh out these entities in a system that queries for
`Added<EntityInstance>` or `Added<IntGridCell>` if you need more access to the
world, or if you just don't want to use the `LdtkEntity` and `LdtkIntCell`
traits.

To load a new level, you can just update the `LevelSelection` resource.
Be sure to check out the `LdtkSettings` resource and the `LevelSet` component
for additional level-loading options.
