use crate::{
    components::{EntityInstanceBundle, GridCoords, Worldly},
    ldtk::{EntityInstance, LayerInstance, TilesetDefinition},
    utils,
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

#[allow(unused_imports)]
use crate::app::register_ldtk_objects::RegisterLdtkObjects;

/// Provides a constructor which can be used for spawning entities from an LDtk file.
///
/// After implementing this trait on a [Bundle], you can register it to spawn automatically for a
/// given identifier via [RegisterLdtkObjects] functions on your [App].
///
/// For common use cases, you'll want to use derive-macro `#[derive(LdtkEntity)]`, but you can also
/// provide a custom implementation.
///
/// You can also implement this trait on non-[Bundle] types, but only [Bundle]s can be registered.
///
/// If there is an entity in the LDtk file that is NOT registered, an entity will be spawned with
/// an [EntityInstance] component, allowing you to flesh it out in your own system.
///
/// *Derive macro requires the "derive" feature, which is enabled by default*
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
/// offset, index)]` will create the field using all of the information provided.
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
///     #[sprite_sheet_bundle("weapons.png", 32.0, 32.0, 4, 5, 5.0, 1.0, 17)]
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
/// ### `#[worldly]`
/// Indicates that a component is [Worldly].
///
/// [Worldly] entities don't despawn when their birth level despawns, and they don't respawn when
/// their birth level respawns.
/// This is useful for entities that travel across multiple levels, like a player.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Player;
/// # #[derive(Component, Default)]
/// # struct BleedDamage;
/// #[derive(Bundle, LdtkEntity)]
/// pub struct PlayerBundle {
///     player: Player,
///     #[sprite_sheet_bundle]
///     #[bundle]
///     sprite_sheet_bundle: SpriteSheetBundle,
///     #[worldly]
///     worldly: Worldly,
/// }
/// ```
///
/// ### `#[grid_coords]`
/// Indicates that a [GridCoords] component should be created with the entity's initial grid-based
/// position in LDtk.
///
/// See the [GridCoords] documentation for more details about this component.
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Block;
/// # #[derive(Component, Default)]
/// # struct Movable;
/// #[derive(Bundle, LdtkEntity)]
/// pub struct BlockBundle {
///     block: Block,
///     movable: Movable,
///     #[sprite_sheet_bundle]
///     #[bundle]
///     sprite_sheet_bundle: SpriteSheetBundle,
///     #[grid_coords]
///     grid_coords: GridCoords,
/// }
/// ```
///
/// ### `#[ldtk_entity]`
/// Indicates that a component or bundle that implements [LdtkEntity] should be created with
/// [LdtkEntity::bundle_entity], allowing for nested [LdtkEntity]s.
///
/// Note: the [LdtkEntity] field decorated with this attribute doesn't have to be a [Bundle].
/// This can be useful if a [Component]'s construction requires the additional access to the world
/// provided by [LdtkEntity::bundle_entity].
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
///
/// ### `#[with(...)]`
///
/// Indicates that this component or bundle should be initialized with the given
/// function.
///
/// Note: The given function should have signature `fn (entity: EntityInstance) -> T`
/// where `T` is the field type. The function should also be accessible in the scope.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # pub struct Player;
/// # #[derive(Component, Default, Clone)]
/// # pub struct Money(f32);
/// #[derive(Clone, Default, Bundle)]
/// pub struct InventoryBundle {
///     pub money: Money,
/// }
///
/// #[derive(Bundle, Default, LdtkEntity)]
/// pub struct PlayerBundle {
///     player: Player,
///     #[with(player_initial_inventory)]
///     #[bundle]
///     collider: InventoryBundle,
/// }
///
/// fn player_initial_inventory(_: EntityInstance) -> InventoryBundle {
///     InventoryBundle {
///         money: Money(4.0)
///     }
/// }
/// ```
pub trait LdtkEntity {
    /// The constructor used by the plugin when spawning entities from an LDtk file.
    /// Has access to resources/assets most commonly used for spawning 2d objects.
    /// If you need access to more of the [World], you can create a system that queries for
    /// `Added<EntityInstance>`, and flesh out the entity from there, instead of implementing this
    /// trait.
    /// This is because the plugin spawns an entity with an [EntityInstance] component if it's not
    /// registered to the app.
    ///
    /// Note: whether or not the entity is registered to the app, the plugin will insert [Transform],
    /// [GlobalTransform], [Name], and [Parent] components to the entity **after** this bundle is
    /// inserted.
    /// So, any custom implementations of these components within this trait will be overwritten.
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self;
}

impl LdtkEntity for EntityInstanceBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
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
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        utils::sprite_bundle_from_entity_info(tileset)
    }
}

impl LdtkEntity for SpriteSheetBundle {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        _: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> Self {
        utils::sprite_sheet_bundle_from_entity_info(
            entity_instance,
            tileset,
            tileset_definition,
            texture_atlases,
        )
    }
}

impl LdtkEntity for Worldly {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        _: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Worldly {
        Worldly::from_entity_info(entity_instance)
    }
}

impl LdtkEntity for GridCoords {
    fn bundle_entity(
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        _: Option<&Handle<Image>>,
        _: Option<&TilesetDefinition>,
        _: &AssetServer,
        _: &mut Assets<TextureAtlas>,
    ) -> Self {
        GridCoords::from_entity_info(entity_instance, layer_instance)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct PhantomLdtkEntity<B: LdtkEntity + Bundle> {
    ldtk_entity: PhantomData<B>,
}

impl<B: LdtkEntity + Bundle> PhantomLdtkEntity<B> {
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
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> &'b mut EntityCommands<'w, 's, 'a>;
}

impl<B: LdtkEntity + Bundle> PhantomLdtkEntityTrait for PhantomLdtkEntity<B> {
    fn evaluate<'w, 's, 'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'w, 's, 'a>,
        entity_instance: &EntityInstance,
        layer_instance: &LayerInstance,
        tileset: Option<&Handle<Image>>,
        tileset_definition: Option<&TilesetDefinition>,
        asset_server: &AssetServer,
        texture_atlases: &mut Assets<TextureAtlas>,
    ) -> &'b mut EntityCommands<'w, 's, 'a> {
        entity_commands.insert_bundle(B::bundle_entity(
            entity_instance,
            layer_instance,
            tileset,
            tileset_definition,
            asset_server,
            texture_atlases,
        ))
    }
}

/// Used by [RegisterLdtkObjects] to associate Ldtk entity identifiers with [LdtkEntity]s.
pub type LdtkEntityMap = HashMap<(Option<String>, Option<String>), Box<dyn PhantomLdtkEntityTrait>>;
