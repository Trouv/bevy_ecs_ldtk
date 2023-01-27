use bevy::{
    ecs::system::{EntityCommands, SystemParam, SystemParamFetch},
    prelude::{AssetServer, Assets, Handle, Image, NonSend},
    sprite::TextureAtlas,
};

use crate::{
    prelude::{LayerInstance, TilesetDefinition},
    utils::ldtk_map_get_or_default,
    EntityInstance, EntityInstanceBundle, IntGridCell, IntGridCellBundle,
};

use super::{
    LdtkEntityMap, LdtkIntCellMap, PhantomLdtkEntity, PhantomLdtkEntityTrait, PhantomLdtkIntCell,
    PhantomLdtkIntCellTrait,
};

pub struct EntityInput<'a> {
    pub entity_instance: &'a EntityInstance,
    pub tileset: Option<&'a Handle<Image>>,
    pub tileset_definition: Option<&'a TilesetDefinition>,
    pub context: SpawnContext<'a>,
}

pub struct IntCellInput<'a> {
    pub int_grid_cell: IntGridCell,
    pub context: SpawnContext<'a>,
}

pub struct SpawnContext<'a> {
    pub layer_instance: &'a LayerInstance,
    pub asset_server: &'a AssetServer,
    pub texture_atlases: &'a mut Assets<TextureAtlas>,
}

pub trait SpawnHook: Send + Sync + 'static {
    type Param<'w, 's>: SystemParam;

    fn spawn_entity(
        &mut self,
        commands: &mut EntityCommands,
        input: EntityInput,
        param_value: &mut <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    );

    fn spawn_int_cell(
        &mut self,
        commands: &mut EntityCommands,
        input: IntCellInput,
        param_value: &mut <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    );
}

pub struct DefaultSpawnHook;

impl SpawnHook for DefaultSpawnHook {
    type Param<'w, 's> = (NonSend<'w, LdtkEntityMap>, NonSend<'w, LdtkIntCellMap>);

    fn spawn_entity(
        &mut self,
        commands: &mut EntityCommands,
        input: EntityInput,
        (ldtk_entity_map, _): &mut (NonSend<LdtkEntityMap>, NonSend<LdtkIntCellMap>),
    ) {
        let default_ldtk_entity: Box<dyn PhantomLdtkEntityTrait> =
            Box::new(PhantomLdtkEntity::<EntityInstanceBundle>::new());

        ldtk_map_get_or_default(
            input.context.layer_instance.identifier.clone(),
            input.entity_instance.identifier.clone(),
            &default_ldtk_entity,
            ldtk_entity_map,
        )
        .evaluate(
            commands,
            input.entity_instance,
            input.context.layer_instance,
            input.tileset,
            input.tileset_definition,
            input.context.asset_server,
            input.context.texture_atlases,
        );
    }

    fn spawn_int_cell(
        &mut self,
        commands: &mut EntityCommands,
        input: IntCellInput,
        (_, ldtk_int_cell_map): &mut (NonSend<LdtkEntityMap>, NonSend<LdtkIntCellMap>),
    ) {
        let default_ldtk_int_cell: Box<dyn PhantomLdtkIntCellTrait> =
            Box::new(PhantomLdtkIntCell::<IntGridCellBundle>::new());

        ldtk_map_get_or_default(
            input.context.layer_instance.identifier.clone(),
            input.int_grid_cell.value,
            &default_ldtk_int_cell,
            ldtk_int_cell_map,
        )
        .evaluate(commands, input.int_grid_cell, input.context.layer_instance);
    }
}
