//! Types and traits for hooking into the ldtk loading process via bevy's [App].
//!
//! *Requires the "app" feature, which is enabled by default*
use crate::{
    components::{EntityInstanceBundle, IntGridCell, IntGridCellBundle},
    ldtk::{EntityInstance, TilesetDefinition},
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

/// Provides a constructor to a bevy [Bundle] which can be used for spawning entities from an LDtk
/// file.
/// After implementing this trait on a bundle, you can register it to spawn automatically for a
/// given identifier via [RegisterLdtkObjects] functions on your [App].
///
/// For common use cases, you'll want to use derive-macro `#[derive(LdtkEntity)]`, but you can also
/// provide a custom implementation.
///
/// If there is an entity in the LDtk file that is NOT registered, an entity will be spawned with
/// an [EntityInstance] component, allowing you to flesh it out in your own system.
///
/// *Requires the "app" feature, which is enabled by default*
///
/// *Derive macro requires the "derive" feature, which is also enabled by default*
///
/// ## Derive macro usage
/// Using `#[derive(LdtkEntity)]` on a [Bundle] struct will allow the type to be registered to the
/// [App] via [RegisterLdtkObjects] functions:
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_ecs_ldtk::prelude::*;
///
/// fn main() {
///     App::empty()
///         .add_plugin(LdtkPlugin)
///         .register_ldtk_entity::<MyBundle>("my_entity_identifier")
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
/// By default, each component or nested bundle in the bundle will be created using their [Default]
/// implementations.
/// However, this behavior can be overriden with some field attribute macros...
///
/// ### `#[sprite_bundle...]`
/// Indicates that a [SpriteBundle] field should be created with an actual material/image.
/// There are two forms for this attribute:
/// - `#[sprite_bundle("path/to/asset.png")]` will create the field using the image at the provided
/// path in the assets folder.
/// - `#[sprite_bundle]` will create the field using its Editor Visual image in LDtk, if it has one.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Sellable;
/// # #[derive(Component, Default)]
/// # struct PlayerComponent;
/// # #[derive(Component, Default)]
/// # struct Health;
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Gem {
///     #[sprite_bundle("textures/gem.png")]
///     #[bundle]
///     sprite_bundle: SpriteBundle,
///     sellable: Sellable,
/// }
///
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Player {
///     player: PlayerComponent,
///     health: Health,
///     #[sprite_bundle] // Uses the Editor Visual sprite in LDtk
///     #[bundle]
///     sprite_bundle: SpriteBundle,
/// }
/// ```
///
/// ### `#[sprite_sheet_bundle...]`
/// Similar to `#[sprite_bundle...]`, indicates that a [SpriteSheetBundle] field should be created
/// with an actual material/image.
/// There are two forms for this attribute:
/// - `#[sprite_sheet_bundle("path/to/asset.png", tile_width, tile_height, columns, rows, padding,
/// index)]` will create the field using all of the information provided.
/// Similar to using [TextureAtlas::from_grid()].
/// - `#[sprite_sheet_bundle]` will create the field using information from the LDtk Editor visual,
/// if it has one.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Damage;
/// # #[derive(Component, Default)]
/// # struct BleedDamage;
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Sword {
///     #[bundle]
///     #[sprite_sheet_bundle("weapons.png", 32.0, 32.0, 4, 5, 5.0, 17)]
///     sprite_sheet: SpriteSheetBundle,
///     damage: Damage,
/// }
///
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Dagger {
///     damage: Damage,
///     bleed_damage: BleedDamage,
///     #[bundle]
///     #[sprite_sheet_bundle]
///     sprite_sheet: SpriteSheetBundle,
/// }
/// ```
///
/// ### `#[ldtk_entity]`
/// Indicates that a nested bundle that implements [LdtkEntity] should be created with
/// [LdtkEntity::bundle_entity], allowing for nested [LdtkEntity]s.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Damage;
/// # #[derive(Component, Default)]
/// # struct BleedDamage;
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Weapon {
///     damage: Damage,
///     #[sprite_bundle]
///     #[bundle]
///     sprite: SpriteBundle,
/// }
///
/// #[derive(Bundle, LdtkEntity)]
/// pub struct Dagger {
///     #[ldtk_entity]
///     #[bundle]
///     weapon_bundle: Weapon,
///     bleed_damage: BleedDamage,
/// }
/// ```
///
/// ### `#[from_entity_instance]`
/// Indicates that a component or bundle that implements [From<EntityInstance>] should be created
/// using that conversion.
/// This allows for more modular and custom component construction, and for different structs that
/// contain the same component to have different constructions of that component, without having to
/// `impl LdtkEntity` for both of them.
/// It also allows you to have an [EntityInstance] field, since all types `T` implement `From<T>`.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Sellable { value: i32 }
/// impl From<EntityInstance> for Sellable {
///     fn from(entity_instance: EntityInstance) -> Sellable {
///         let sell_value = match entity_instance.identifier.as_str() {
///             "gem" => 1000,
///             "nickel" => 5,
///             _ => 10,
///         };
///
///         Sellable {
///             value: sell_value,
///         }
///     }
/// }
///
/// #[derive(Bundle, LdtkEntity)]
/// pub struct NickelBundle {
///     #[sprite_bundle]
///     #[bundle]
///     sprite: SpriteBundle,
///     #[from_entity_instance]
///     sellable: Sellable,
///     #[from_entity_instance]
///     entity_instance: EntityInstance,
/// }
/// ```
pub trait LdtkEntity: Bundle {
    /// The constructor used by the plugin when spawning entities from an LDtk file.
    /// Has access to resources/assets most commonly used for spawning 2d objects.
    /// If you need access to more of the [World], you can create a system that queries for
    /// `Added<EntityInstance>`, and flesh out the entity from there, instead of implementing this
    /// trait.
    /// This is because the plugin spawns an entity with an [EntityInstance] component if it's not
    /// registered to the app.
    ///
    /// Note: whether or not the entity is registered to the app, the plugin will insert [Transform],
    /// [GlobalTransform], and [Parent] components to the entity **after** this bundle is inserted.
    /// So, any custom implementations of these components within this trait will be overwritten.
    fn bundle_entity(
        entity_instance: &EntityInstance,
        tileset: Option<&Handle<Texture>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self;
}

impl LdtkEntity for EntityInstanceBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: Option<&Handle<Texture>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<ColorMaterial>,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        EntityInstanceBundle {
            entity_instance: entity_instance.clone(),
        }
    }
}

impl LdtkEntity for SpriteBundle {
    fn bundle_entity(
        _: &EntityInstance,
        tileset: Option<&Handle<Texture>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        let tileset = match tileset {
            Some(tileset) => tileset.clone(),
            None => {
                warn!("EntityInstance needs a tileset to be bundled as a SpriteBundle");
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

impl LdtkEntity for SpriteSheetBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        tileset: Option<&Handle<Texture>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<ColorMaterial>,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        match (tileset, &entity_instance.tile, tileset_definition) {
            (Some(tileset), Some(tile), Some(tileset_definition)) => SpriteSheetBundle {
                texture_atlas: texture_atlases.add(TextureAtlas::from_grid_with_padding(
                    tileset.clone(),
                    Vec2::new(tile.src_rect[2] as f32, tile.src_rect[3] as f32),
                    tileset_definition.c_wid as usize,
                    tileset_definition.c_hei as usize,
                    Vec2::splat(tileset_definition.spacing as f32),
                )),
                sprite: TextureAtlasSprite {
                    index: (tile.src_rect[1] / tile.src_rect[3]) as u32
                        * tileset_definition.c_hei as u32
                        + (tile.src_rect[0] / tile.src_rect[2]) as u32,
                    ..Default::default()
                },
                ..Default::default()
            },
            _ => {
                warn!("EntityInstance needs a tile, an associated tileset, and an associated tileset definition to be bundled as a SpriteSheetBundle");
                SpriteSheetBundle::default()
            }
        }
    }
}

pub struct PhantomLdtkEntity<B: LdtkEntity> {
    ldtk_entity: PhantomData<B>,
}

impl<B: LdtkEntity> PhantomLdtkEntity<B> {
    pub fn new() -> Self {
        PhantomLdtkEntity::<B> {
            ldtk_entity: PhantomData,
        }
    }
}

pub trait PhantomLdtkEntityTrait {
    #[allow(clippy::too_many_arguments)]
    fn evaluate<'w, 's, 'a, 'b>(
        &self,
        commands: &'b mut EntityCommands<'w, 's, 'a>,
        entity_instance: &EntityInstance,
        tileset: Option<&Handle<Texture>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> &'b mut EntityCommands<'w, 's, 'a>;
}

impl<B: LdtkEntity> PhantomLdtkEntityTrait for PhantomLdtkEntity<B> {
    fn evaluate<'w, 's, 'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'w, 's, 'a>,
        entity_instance: &EntityInstance,
        tileset: Option<&Handle<Texture>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        materials: &mut Assets<ColorMaterial>,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        entity_commands.insert_bundle(B::bundle_entity(
            entity_instance,
            tileset,
            tileset_definition,
            asset_server,
            materials,
            texture_atlases,
        ))
    }
}

/// Used by [RegisterLdtkObjects] to associate Ldtk entity identifiers with [LdtkEntity]s.
pub type LdtkEntityMap = HashMap<(Option<String>, Option<String>), Box<dyn PhantomLdtkEntityTrait>>;

/// Provides a constructor to a bevy [Bundle] which can be used for spawning additional components
/// on IntGrid tiles.
/// After implementing this trait on a bundle, you can register it to spawn automatically for a
/// given int grid value via
/// [RegisterLdtkObjects] on your [App].
///
/// For common use cases, you'll want to use derive-macro `#[derive(LdtkIntCell)]`, but you can
/// also provide a custom implementation.
///
/// If there is an IntGrid tile in the LDtk file whose value is NOT registered, an entity will be
/// spawned with an [IntGridCell] component, allowing you to flesh it out in your own system.
///
/// *Requires the "app" feature, which is enabled by default*
///
/// *Derive macro requires the "derive" feature, which is also enabled by default*
///
/// ## Derive macro usage
/// Using `#[derive(LdtkIntCell)]` on a [Bundle] struct will allow the type to be registered to the
/// [App] via [RegisterLdtkObjects] functions:
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_ecs_ldtk::prelude::*;
///
/// fn main() {
///     App::empty()
///         .add_plugin(LdtkPlugin)
///         .register_ldtk_int_cell::<MyBundle>(1)
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
/// #[derive(Bundle, LdtkIntCell)]
/// pub struct MyBundle {
///     a: ComponentA,
///     b: ComponentB,
///     c: ComponentC,
/// }
/// ```
/// Now, when loading your ldtk file, any IntGrid tiles with the value `1` will be spawned with as
/// tiles with `MyBundle` inserted.
///
/// By default, each component or nested bundle in the bundle will be created using their [Default]
/// implementations.
/// However, this behavior can be overriden with some field attribute macros...
///
/// ### `#[ldtk_int_cell]`
/// Indicates that a nested bundle that implements [LdtkIntCell] should be created with
/// [LdtkIntCell::bundle_int_cell], allowing for nested [LdtkIntCell]s.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct RigidBody;
/// # #[derive(Component, Default)]
/// # struct Damage;
/// #[derive(Bundle, LdtkIntCell)]
/// pub struct Wall {
///     rigid_body: RigidBody,
/// }
///
/// #[derive(Bundle, LdtkIntCell)]
/// pub struct DestructibleWall {
///     #[ldtk_int_cell]
///     #[bundle]
///     wall: Wall,
///     damage: Damage,
/// }
/// ```
///
/// ### `#[from_int_grid_cell]`
/// Indicates that a component or bundle that implements [From<IntGridCell>] should be created
/// using that conversion.
/// This allows for more modular and custom component construction, and for different structs that
/// contain the same component to have different constructions of that component, without having to
/// `impl LdtkIntCell` for both of them.
/// It also allows you to have an [IntGridCell] field, since all types `T` implement `From<T>`.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Fluid { viscosity: i32 }
/// # #[derive(Component, Default)]
/// # struct Damage;
/// impl From<IntGridCell> for Fluid {
///     fn from(int_grid_cell: IntGridCell) -> Fluid {
///         let viscosity = match int_grid_cell.value {
///             1 => 5,
///             2 => 20,
///             _ => 0,
///         };
///
///         Fluid {
///             viscosity,
///         }
///     }
/// }
///
/// #[derive(Bundle, LdtkIntCell)]
/// pub struct Lava {
///     #[from_int_grid_cell]
///     fluid: Fluid,
///     #[from_int_grid_cell]
///     int_grid_cell: IntGridCell,
///     damage: Damage,
/// }
/// ```
pub trait LdtkIntCell: Bundle {
    /// The constructor used by the plugin when spawning additional components on IntGrid tiles.
    /// If you need access to more of the [World], you can create a system that queries for
    /// `Added<IntGridCell>`, and flesh out the entity from there, instead of implementing this
    /// trait.
    /// This is because the plugin spawns a tile with an [IntGridCell] component if the tile's
    /// value is not registered to the app.
    ///
    /// Note: whether or not the entity is registered to the app, the plugin will insert [Transform],
    /// [GlobalTransform], and [Parent] components to the entity **after** this bundle is inserted.
    /// So, any custom implementations of these components within this trait will be overwritten.
    /// Furthermore, a [bevy_ecs_tilemap::TileBundle] will be inserted **before** this bundle, so
    /// be careful not to overwrite the components provided by that bundle.
    fn bundle_int_cell(int_grid_cell: IntGridCell) -> Self;
}

impl LdtkIntCell for IntGridCellBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell) -> Self {
        IntGridCellBundle { int_grid_cell }
    }
}

pub struct PhantomLdtkIntCell<B: LdtkIntCell> {
    ldtk_int_cell: PhantomData<B>,
}

impl<B: LdtkIntCell> PhantomLdtkIntCell<B> {
    pub fn new() -> Self {
        PhantomLdtkIntCell::<B> {
            ldtk_int_cell: PhantomData,
        }
    }
}

pub trait PhantomLdtkIntCellTrait {
    fn evaluate<'w, 's, 'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'w, 's, 'a>,
        int_grid_cell: IntGridCell,
    ) -> &'b mut EntityCommands<'w, 's, 'a>;
}

impl<B: LdtkIntCell> PhantomLdtkIntCellTrait for PhantomLdtkIntCell<B> {
    fn evaluate<'w, 's, 'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'w, 's, 'a>,
        int_grid_cell: IntGridCell,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        entity_commands.insert_bundle(B::bundle_int_cell(int_grid_cell))
    }
}

pub type LdtkIntCellMap = HashMap<(Option<String>, Option<i32>), Box<dyn PhantomLdtkIntCellTrait>>;

/// Provides functions to register [Bundle]s to bevy's [App] for particular LDtk layer identifiers,
/// entity identifiers, and IntGrid values.
/// After being registered, [Entity]s will be spawned with these bundles when some IntGrid tile or
/// entity meets the criteria you specify.
///
/// Not necessarily intended for custom implementations on your own types.
///
/// *Requires the "app" feature, which is enabled by default*
pub trait RegisterLdtkObjects {
    /// Used internally by all the other LDtk entity registration functions.
    ///
    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it provides
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
    fn register_ldtk_entity_for_layer_optional<B: LdtkEntity>(
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
    ///         .add_plugin(LdtkPlugin)
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
    /// #[derive(Bundle, LdtkEntity)]
    /// pub struct MyBundle {
    ///     a: ComponentA,
    ///     b: ComponentB,
    ///     c: ComponentC,
    /// }
    /// ```
    ///
    /// You can find more details on the `#[derive(LdtkEntity)]` macro at [LdtkEntity].
    fn register_ldtk_entity_for_layer<B: LdtkEntity>(
        &mut self,
        layer_identifier: &str,
        entity_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            Some(entity_identifier.to_string()),
        )
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it applies the
    /// registration to all layers.
    fn register_ldtk_entity<B: LdtkEntity>(&mut self, entity_identifier: &str) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(None, Some(entity_identifier.to_string()))
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it applies the
    /// registration to all entities on the given layer.
    fn register_default_ldtk_entity_for_layer<B: LdtkEntity>(
        &mut self,
        layer_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(Some(layer_identifier.to_string()), None)
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_entity_for_layer], except it applies the
    /// registration to any entity and any layer.
    fn register_default_ldtk_entity<B: LdtkEntity>(&mut self) -> &mut Self {
        self.register_ldtk_entity_for_layer_optional::<B>(None, None)
    }

    /// Used internally by all the other LDtk int cell registration functions.
    ///
    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it provides
    /// defaulting functionality:
    /// - Setting `layer_identifier` to [None] will make the registration apply to any IntGrid layer.
    /// - Setting `value` to [None] will make the registration apply to any IntGrid tile.
    ///
    /// This defaulting functionality means that a particular LDtk IntGrid tile may match multiple
    /// registrations.
    /// In these cases, registrations are prioritized in order of most to least specific:
    /// 1. `layer_identifier` and `value` are specified
    /// 2. Just `value` is specified
    /// 3. Just `layer_identifier` is specified
    /// 4. Neither `value` nor `layer_identifier` are specified
    fn register_ldtk_int_cell_for_layer_optional<B: LdtkIntCell>(
        &mut self,
        layer_identifier: Option<String>,
        value: Option<i32>,
    ) -> &mut Self;

    /// Registers [LdtkIntCell] types to be inserted for a given IntGrid value and layer identifier
    /// in an LDtk file.
    ///
    /// This example lets the plugin know that it should spawn a MyBundle when it encounters an
    /// IntGrid tile whose value is `1` on a "MyLayerIdentifier" layer.
    /// ```no_run
    /// use bevy::prelude::*;
    /// use bevy_ecs_ldtk::prelude::*;
    ///
    /// fn main() {
    ///     App::empty()
    ///         .add_plugin(LdtkPlugin)
    ///         .register_ldtk_int_cell_for_layer::<MyBundle>("MyLayerIdentifier", 1)
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
    /// #[derive(Bundle, LdtkIntCell)]
    /// pub struct MyBundle {
    ///     a: ComponentA,
    ///     b: ComponentB,
    ///     c: ComponentC,
    /// }
    /// ```
    ///
    /// You can find more details on the `#[derive(LdtkIntCell)]` macro at [LdtkIntCell].
    fn register_ldtk_int_cell_for_layer<B: LdtkIntCell>(
        &mut self,
        layer_identifier: &str,
        value: i32,
    ) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            Some(value),
        )
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it applies the
    /// registration to all layers.
    fn register_ldtk_int_cell<B: LdtkIntCell>(&mut self, value: i32) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(None, Some(value))
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it applies the
    /// registration to all tiles on the given layer.
    fn register_default_ldtk_int_cell_for_layer<B: LdtkIntCell>(
        &mut self,
        layer_identifier: &str,
    ) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(
            Some(layer_identifier.to_string()),
            None,
        )
    }

    /// Similar to [RegisterLdtkObjects::register_ldtk_int_cell_for_layer], except it applies the
    /// registration to any tile and any layer.
    fn register_default_ldtk_int_cell<B: LdtkIntCell>(&mut self) -> &mut Self {
        self.register_ldtk_int_cell_for_layer_optional::<B>(None, None)
    }
}

impl RegisterLdtkObjects for App {
    fn register_ldtk_entity_for_layer_optional<B: LdtkEntity>(
        &mut self,
        layer_identifier: Option<String>,
        entity_identifier: Option<String>,
    ) -> &mut Self {
        let new_entry = Box::new(PhantomLdtkEntity::<B> {
            ldtk_entity: PhantomData,
        });
        match self.world.get_non_send_resource_mut::<LdtkEntityMap>() {
            Some(mut entries) => {
                entries.insert((layer_identifier, entity_identifier), new_entry);
            }
            None => {
                let mut bundle_map = LdtkEntityMap::new();
                bundle_map.insert((layer_identifier, entity_identifier), new_entry);
                self.world.insert_non_send::<LdtkEntityMap>(bundle_map);
            }
        }
        self
    }

    fn register_ldtk_int_cell_for_layer_optional<B: LdtkIntCell>(
        &mut self,
        layer_identifier: Option<String>,
        value: Option<i32>,
    ) -> &mut Self {
        let new_entry = Box::new(PhantomLdtkIntCell::<B> {
            ldtk_int_cell: PhantomData,
        });
        match self.world.get_non_send_resource_mut::<LdtkIntCellMap>() {
            Some(mut entries) => {
                entries.insert((layer_identifier, value), new_entry);
            }
            None => {
                let mut bundle_map = LdtkIntCellMap::new();
                bundle_map.insert((layer_identifier, value), new_entry);
                self.world.insert_non_send::<LdtkIntCellMap>(bundle_map);
            }
        }
        self
    }
}
