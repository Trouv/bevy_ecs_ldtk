use crate::components::{IntGridCell, IntGridCellBundle};
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

#[allow(unused_imports)]
use crate::app::register_ldtk_objects::RegisterLdtkObjects;

/// Provides a constructor to a bevy [Bundle] which can be used for spawning additional components
/// on IntGrid tiles.
///
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
/// *Derive macro requires the "derive" feature, which is enabled by default*
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

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash)]
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

/// Used by [RegisterLdtkObjects] to associate Ldtk IntGrid values with [LdtkIntCell]s.
pub type LdtkIntCellMap = HashMap<(Option<String>, Option<i32>), Box<dyn PhantomLdtkIntCellTrait>>;