# Create Bevy Relations from LDtk Entity References
LDtk allows entities to point to other entities using a field.
This is analogous to a bevy "relation" - a component on one entity that stores the `Entity` identifier of another entity.

This chapter goes through one possible method for resolving LDtk entity references as such.
This code is used in the `field_instances` cargo example, and facilitates "enemy" entities pointing to another "enemy" entity as their "mother".

## Register unresolved reference
First, create a component representing an "unresolved" entity reference, storing the target entity's LDtk iid rather than a bevy `Entity`:
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{ #include ../../../examples/field_instances/mother.rs:10:11 }}
```

Create a method for constructing this component from an `&EntityInstance`.
This should retrieve the value of the entity reference field instance on the LDtk entity.
Most likely, you'll use a hard-coded field identifier ("mother" in this example) to find it:
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# {{ #include ../../../examples/field_instances/mother.rs:11 }}
{{ #include ../../../examples/field_instances/mother.rs:13:23 }}
```

Add this component to the `LdtkEntity` and configure it to be constructed using this method.
This guide assumes that you've already registered this bundle to the app.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# {{ #include ../../../examples/field_instances/mother.rs:10 }}
# {{ #include ../../../examples/field_instances/mother.rs:11 }}
# impl UnresolvedMotherRef { fn from_mother_field(_: &EntityInstance) -> UnresolvedMotherRef { todo!() } }
{{ #include ../../../examples/field_instances/enemy.rs:7:8}}
{{ #include ../../../examples/field_instances/enemy.rs:15:19}}
```

## Resolve reference in post-processing
Create a second relational component that stores the actual bevy `Entity` that this `Unresolved` reference should "resolve" to.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
{{ #include ../../../examples/field_instances/mother.rs:26:27 }}
```

Finally, create a ["post-processing"](../explanation/game-logic-integration.html#post-processing-plugin-spawned-entities) system that takes entities with the `Unresolved` component, and replaces it with the relational component.
```rust,no_run
# use bevy::prelude::*;
# use bevy_ecs_ldtk::prelude::*;
# {{ #include ../../../examples/field_instances/mother.rs:10 }}
# {{ #include ../../../examples/field_instances/mother.rs:11 }}
# {{ #include ../../../examples/field_instances/mother.rs:26 }}
# {{ #include ../../../examples/field_instances/mother.rs:27 }}
{{ #include ../../../examples/field_instances/mother.rs:29:51 }}
```
