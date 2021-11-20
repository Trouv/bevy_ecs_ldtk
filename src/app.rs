use crate::{assets::TilesetMap, ldtk::EntityInstance};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

/// Provides a constructor to a bevy `Bundle` which can be used for spawning entities from an LDtk
/// file.
/// After implementing this trait on a bundle, you can register it to spawn automatically for a
/// given identifier via `app.register_ldtk_entity()`.
///
/// For common use cases, you'll want to use derive-macro `#[derive(LdtkEntity)]`, but you can also
/// provide a custom implementation.
///
/// *Requires the "app" feature, which is enabled by default*
///
/// ## Derive macro usage
/// Using `#[derive(LdtkEntity)]` on a `Bundle` struct will allow the type to be registered to the
/// app via `app.register_ldtk_entity`:
/// ```
/// use bevy::prelude::*;
/// use bevy_ecs_ldtk::prelude::*;
///
/// fn main() {
///     App::new()
///         .add_plugin(LdtkPlugin)
///         .register_ldtk_entity::<MyBundle>("my_entity_identifier")
///         // add other systems, plugins, resources...
///         .run()
/// }
///
/// #[derive(Bundle, LdtkEntity)]
/// pub struct MyBundle {
///     a: ComponentA,
///     b: ComponentB,
///     c: ComponentC,
/// }
/// ```
/// Now, when loading your ldtk file, any entities with the entity identifier
/// "my_entity_identifier" will be spawned as `MyBundle`s.
///
/// By default, each component or nested bundle in the bundle will be created using their `Default`
/// implementations.
/// However, this behavior can be overriden with some field attribute macros...
///
/// `#[sprite_bundle...]` indicates that a `SpriteBundle` field should be created with an actual
/// material/image.
/// There are two forms for this attribute:
/// - `#[sprite_bundle("path/to/asset.png")]` will create the field using the image at the provided
/// path in the assets folder.
/// - `#[sprite_bundle]` will create the field using its Editor Visual image in LDtk, if it has one.
/// ```
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Gem {
///     #[sprite_bundle("textures/gem.png")]
///     #[bundle]
///     sprite_bundle: SpriteBundle,
///     other: OtherComponent,
/// }
///
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Player {
///     player: PlayerComponent,
///     #[sprite_bundle] // Uses the Editor Visual sprite in LDtk
///     #[bundle]
///     sprite_bundle: SpriteBundle,
/// }
/// ```
pub trait LdtkEntity: Bundle {
    /// The constructor used by the plugin when spawning entities from an LDtk file.
    /// Has access to resources/assets most commonly used for spawning 2d objects.
    /// If you need access to more of the World, you can create a system that queries for
    /// `Added<EntityInstance>`, and flesh out the entity from there, instead of implementing this
    /// trait.
    /// This is because the plugin spawns an entity with an `EntityInstance` component if it's not
    /// registered to the app.
    ///
    /// Note: whether or not the entity is registered to the app, the plugin will insert `Transform`,
    /// `GlobalTransform`, and `Parent` components to the entity *after* the entity is spawned.
    /// So, any custom implementations of these components within this trait will be overwritten.
    fn from_instance(
        entity_instance: &EntityInstance,
        tileset_map: &TilesetMap,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self;
}

impl LdtkEntity for SpriteBundle {
    fn from_instance(
        entity_instance: &EntityInstance,
        tileset_map: &TilesetMap,
        _: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        _: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self {
        let tile = match entity_instance.tile.as_ref() {
            Some(tile) => tile,
            None => {
                warn!("#[sprite_bundle] attribute expected the EntityInstance to have a tile defined.");
                return SpriteBundle::default();
            }
        };

        let tileset = match tileset_map.get(&tile.tileset_uid) {
            Some(tileset) => tileset.clone(),
            None => {
                warn!("EntityInstance's tileset should be in the TilesetMap");
                return SpriteBundle::default();
            }
        };

        let material = materials.add(tileset.into());
        SpriteBundle {
            material,
            ..Default::default()
        }
    }
}

pub trait RegisterLdtkObjects {
    fn register_ldtk_entity<B: LdtkEntity>(&mut self, identifier: &str) -> &mut App;
}

impl RegisterLdtkObjects for App {
    fn register_ldtk_entity<B: LdtkEntity>(&mut self, identifier: &str) -> &mut App {
        let new_entry = Box::new(PhantomLdtkEntity::<B> {
            ldtk_entity: PhantomData,
        });
        match self.world.get_non_send_resource_mut::<LdtkEntityMap>() {
            Some(mut entries) => {
                entries.insert(identifier.to_string(), new_entry);
            }
            None => {
                let mut bundle_map = LdtkEntityMap::new();
                bundle_map.insert(identifier.to_string(), new_entry);
                self.world.insert_non_send::<LdtkEntityMap>(bundle_map);
            }
        }
        self
    }
}

pub type LdtkEntityMap = HashMap<String, Box<dyn PhantomLdtkEntityTrait>>;

pub struct PhantomLdtkEntity<B: LdtkEntity> {
    ldtk_entity: PhantomData<B>,
}

pub trait PhantomLdtkEntityTrait {
    fn evaluate<'w, 's, 'a>(
        &self,
        commands: &'a mut Commands<'w, 's>,
        entity_instance: &EntityInstance,
        tileset_map: &TilesetMap,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> EntityCommands<'w, 's, 'a>;
}

impl<B: LdtkEntity> PhantomLdtkEntityTrait for PhantomLdtkEntity<B> {
    fn evaluate<'w, 's, 'a>(
        &self,
        commands: &'a mut Commands<'w, 's>,
        entity_instance: &EntityInstance,
        tileset_map: &TilesetMap,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> EntityCommands<'w, 's, 'a> {
        commands.spawn_bundle(B::from_instance(
            entity_instance,
            tileset_map,
            asset_server,
            materials,
            texture_atlases,
        ))
    }
}
