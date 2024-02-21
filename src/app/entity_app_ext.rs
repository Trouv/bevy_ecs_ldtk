//! Provides [LdtkEntityAppExt] for registering bundles to spawn for given LDtk Entity identifiers.
use crate::app::ldtk_entity::*;
use bevy::prelude::*;

/// [Bundle]: bevy::prelude::Bundle
/// [App]: bevy::prelude::App
///
/// Provides functions to register [Bundle]s to bevy's [App] for particular LDtk layer and entity
/// identifiers.
///
/// After being registered, entities will be spawned with these [Bundle]s when some entity in LDtk
/// meets the criteria you specify.
///
/// Not intended for custom implementations on your own types.
pub trait LdtkEntityAppExt {
    /// Used internally by all the other LDtk entity registration functions.
    ///
    /// Similar to [LdtkEntityAppExt::register_ldtk_entity_for_layer], except it provides
    /// defaulting functionality:
    /// - Setting `layer_identifier` to [None] will make the registration apply to any Entity layer.
    /// - Setting `entity_identifier` to [None] will make the registration apply to any LDtk entity.
    ///
    /// This defaulting functionality means that a particular instance of an LDtk entity may match
    /// multiple registrations.
    /// In these cases, registrations are prioritized in order of most to least specific:
    /// 1. `layer_identifier` and `entity_identifier` are specified
    /// 2. Just `entity_identifier` is specified
    /// 3. Just `layer_identifier` is specified
    /// 4. Neither `entity_identifier` nor `layer_identifier` are specified
    fn register_ldtk_entity_for_layer_optional<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
    ) -> &mut Self;

    /// Registers [LdtkEntity] types to be spawned for a given Entity identifier and layer
    /// identifier in an LDtk file.
    ///
    /// This example lets the plugin know that it should spawn a MyBundle when it encounters a
    /// "my_entity_identifier" entity on a "MyLayerIdentifier" layer.
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// fn main() {
    ///     App::empty()
    ///         .add_plugins(LdtkPlugin)
    ///         .register_ldtk_entity_for_layer::<MyBundle>("MyLayerIdentifier", "my_entity_identifier")
    ///         // add other systems, plugins, resources...
    ///         .run();
    /// }
    ///
    /// # #[derive(Component, Default)]
    /// # struct ComponentA;
    /// # #[derive(Component, Default)]
    /// # struct ComponentB;
    /// # #[derive(Component, Default)]
    /// # struct ComponentC;
    /// #[derive(Bundle, LdtkEntity, Default)]
    /// pub struct MyBundle {
    ///     a: ComponentA,
    ///     b: ComponentB,
    ///     c: ComponentC,
    /// }
    /// ```
    ///
    /// You can find more details on the `#[derive(LdtkEntity)]` macro at [LdtkEntity].
    fn register_ldtk_entity_for_layer<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: &str,
        entity_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            Some(entity_identifier.to_string()),
        )
    }

    /// Similar to [LdtkEntityAppExt::register_ldtk_entity_for_layer], except it applies the
    /// registration to all layers.
    fn register_ldtk_entity<B: LdtkEntity + Bundle>(
        &mut self,
        entity_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(None, Some(entity_identifier.to_string()))
    }

    /// Similar to [LdtkEntityAppExt::register_ldtk_entity_for_layer], except it applies the
    /// registration to all entities on the given layer.
    fn register_default_ldtk_entity_for_layer<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(Some(layer_identifier.to_string()), None)
    }

    /// Similar to [LdtkEntityAppExt::register_ldtk_entity_for_layer], except it applies the
    /// registration to any entity and any layer.
    fn register_default_ldtk_entity<B: LdtkEntity + Bundle>(&mut self) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(None, None)
    }
}

impl LdtkEntityAppExt for App {
    fn register_ldtk_entity_for_layer_optional<B: LdtkEntity + Bundle>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
    ) -> &mut Self {
        let new_entry = Box::new(PhantomLdtkEntity::<B>::new());
        match self.world.get_non_send_resource_mut::<LdtkEntityMap>() {
            Some(mut entries) => {
                entries.insert((layer_identifier, entity_identifier), new_entry);
            }
            None => {
                let mut bundle_map = LdtkEntityMap::new();
                bundle_map.insert((layer_identifier, entity_identifier), new_entry);
                self.world
                    .insert_non_send_resource::<LdtkEntityMap>(bundle_map);
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        components::EntityInstance,
        ldtk::{LayerInstance, TilesetDefinition},
    };

    #[derive(Default, Component, Debug)]
    struct ComponentA;

    #[derive(Default, Component, Debug)]
    struct ComponentB;

    #[derive(Default, Bundle, Debug)]
    struct LdtkEntityBundle {
        a: ComponentA,
        b: ComponentB,
    }

    impl LdtkEntity for LdtkEntityBundle {
        fn bundle_entity(
            _: &EntityInstance,
            _: &LayerInstance,
            _: Option<&Handle<Image>>,
            _: Option<&TilesetDefinition>,
            _: &AssetServer,
            _: &mut Assets<TextureAtlasLayout>,
        ) -> LdtkEntityBundle {
            LdtkEntityBundle::default()
        }
    }

    #[test]
    fn test_ldtk_entity_registrations() {
        let mut app = App::new();
        app.register_ldtk_entity_for_layer::<LdtkEntityBundle>("layer", "entity_for_layer")
            .register_ldtk_entity::<LdtkEntityBundle>("entity")
            .register_default_ldtk_entity_for_layer::<LdtkEntityBundle>("default_entity_for_layer")
            .register_default_ldtk_entity::<LdtkEntityBundle>();

        let ldtk_entity_map = app.world.get_non_send_resource::<LdtkEntityMap>().unwrap();

        assert!(ldtk_entity_map.contains_key(&(
            Some("layer".to_string()),
            Some("entity_for_layer".to_string())
        )));

        assert!(ldtk_entity_map.contains_key(&(None, Some("entity".to_string()))));

        assert!(ldtk_entity_map.contains_key(&(Some("default_entity_for_layer".to_string()), None)));

        assert!(ldtk_entity_map.contains_key(&(None, None)));
    }
}
