use crate::{
    components::{IntGridCell, IntGridCellBundle},
    ldtk::LayerInstance,
};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

/// [LdtkIntCellAppExt]: super::LdtkIntCellAppExt
/// [Bundle]: bevy::prelude::Bundle
/// [App]: bevy::prelude::App
/// [Component]: bevy::prelude::Component
///
/// Provides a constructor which can be used for spawning additional components on IntGrid tiles.
///
/// After implementing this trait on a [Bundle], you can register it to spawn automatically for a
/// given int grid value via [LdtkIntCellAppExt] on your [App].
///
/// For common use cases, you'll want to use derive-macro `#[derive(LdtkIntCell)]`, but you can
/// also provide a custom implementation.
///
/// You can also implement this trait on non-[Bundle] types, but only [Bundle]s can be registered.
///
/// If there is an IntGrid tile in the LDtk file whose value is NOT registered, an entity will be
/// spawned with an [IntGridCell] component, allowing you to flesh it out in your own system.
///
/// *Derive macro requires the "derive" feature, which is enabled by default*
///
/// ## Derive macro usage
/// Using `#[derive(LdtkIntCell)]` on a [Bundle] struct will allow the type to be registered to the
/// [App] via [LdtkIntCellAppExt] functions:
/// ```no_run
/// use bevy::prelude::*;
/// use bevy_ecs_ldtk::prelude::*;
///
/// fn main() {
///     App::empty()
///         .add_plugins(LdtkPlugin)
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
/// #[derive(Bundle, LdtkIntCell, Default)]
/// pub struct MyBundle {
///     a: ComponentA,
///     b: ComponentB,
///     c: ComponentC,
/// }
/// ```
/// Now, when loading your ldtk file, any IntGrid tiles with the value `1` will be spawned with as
/// tiles with `MyBundle` inserted.
///
/// By default, each component or nested bundle in the bundle will be consumed from bundle's
/// [Default] implementation, which means that deriving (or implementing manually) [Default]
/// is required (unless all fields are overriden, see below).
/// However, this behavior can be overriden with some field attribute macros...
///
/// ### `#[ldtk_int_cell]`
/// Indicates that a component or bundle that implements [LdtkIntCell] should be created with
/// [LdtkIntCell::bundle_int_cell], allowing for nested [LdtkIntCell]s.
///
/// Note: the [LdtkIntCell] field decorated with this attribute doesn't have to be a [Bundle].
/// This can be useful if a [Component]'s construction requires the additional access to the world
/// provided by [LdtkIntCell::bundle_int_cell].
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct RigidBody;
/// # #[derive(Component, Default)]
/// # struct Damage;
/// #[derive(Bundle, LdtkIntCell, Default)]
/// pub struct Wall {
///     rigid_body: RigidBody,
/// }
///
/// #[derive(Bundle, LdtkIntCell, Default)]
/// pub struct DestructibleWall {
///     #[ldtk_int_cell]
///     wall: Wall,
///     damage: Damage,
/// }
/// ```
///
/// ### `#[from_int_grid_cell]`
/// Indicates that a component or bundle that implements [`From<IntGridCell>`] should be created
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
/// #[derive(Bundle, LdtkIntCell, Default)]
/// pub struct Lava {
///     #[from_int_grid_cell]
///     fluid: Fluid,
///     #[from_int_grid_cell]
///     int_grid_cell: IntGridCell,
///     damage: Damage,
/// }
/// ```
///
/// ### `#[with(...)]`
///
/// Indicates that this component or bundle should be initialized with the given
/// function.
///
/// Note: The given function should have signature `fn (int_grid_cell: IntGridCell) -> T`
/// where `T` is the field type. The function should also be accessible in the scope.
///
/// ```
/// # use bevy::prelude::*;
/// # use bevy_ecs_ldtk::prelude::*;
/// # #[derive(Component, Default)]
/// # struct Fluid { viscosity: i32 }
/// # #[derive(Component, Default)]
/// # struct Damage;
/// fn initial_fluid(_: IntGridCell) -> Fluid {
///     Fluid {
///         viscosity: 20
///     }
/// }
///
/// #[derive(Bundle, LdtkIntCell, Default)]
/// pub struct Lava {
///     #[with(initial_fluid)]
///     fluid: Fluid,
///     damage: Damage,
/// }
/// ```
pub trait LdtkIntCell {
    /// The constructor used by the plugin when spawning additional components on IntGrid tiles.
    /// If you need access to more of the [World](bevy::prelude::World), you can create a system that queries for
    /// `Added<IntGridCell>`, and flesh out the entity from there, instead of implementing this
    /// trait.
    /// This is because the plugin spawns a tile with an [IntGridCell] component if the tile's
    /// value is not registered to the app.
    ///
    /// Note: whether or not the entity is registered to the app, the plugin will insert a
    /// [SpatialBundle](bevy::prelude::SpatialBundle) to the entity **after** this bundle is
    /// inserted.
    /// So, any custom implementations of these components within this trait will be overwritten.
    /// Furthermore, a [bevy_ecs_tilemap::tiles::TileBundle] will be inserted **before** this bundle, so
    /// be careful not to overwrite the components provided by that bundle.
    fn bundle_int_cell(int_grid_cell: IntGridCell, layer_instance: &LayerInstance) -> Self;
}

impl LdtkIntCell for IntGridCellBundle {
    fn bundle_int_cell(int_grid_cell: IntGridCell, _: &LayerInstance) -> Self {
        IntGridCellBundle { int_grid_cell }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct PhantomLdtkIntCell<B: LdtkIntCell + Bundle> {
    ldtk_int_cell: PhantomData<B>,
}

impl<B: LdtkIntCell + Bundle> PhantomLdtkIntCell<B> {
    pub fn new() -> Self {
        PhantomLdtkIntCell::<B> {
            ldtk_int_cell: PhantomData,
        }
    }
}

pub trait PhantomLdtkIntCellTrait {
    fn evaluate<'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'a>,
        int_grid_cell: IntGridCell,
        layer_instance: &LayerInstance,
    ) -> &'b mut EntityCommands<'a>;
}

impl<B: LdtkIntCell + Bundle> PhantomLdtkIntCellTrait for PhantomLdtkIntCell<B> {
    fn evaluate<'a, 'b>(
        &self,
        entity_commands: &'b mut EntityCommands<'a>,
        int_grid_cell: IntGridCell,
        layer_instance: &LayerInstance,
    ) -> &'b mut EntityCommands<'a> {
        entity_commands.insert(B::bundle_int_cell(int_grid_cell, layer_instance))
    }
}

/// Used by [LdtkIntCellAppExt](super::LdtkIntCellAppExt) to associate Ldtk IntGrid values with [LdtkIntCell]s.
pub type LdtkIntCellMap = HashMap<(Option<String>, Option<i32>), Box<dyn PhantomLdtkIntCellTrait>>;
