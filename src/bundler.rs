use crate::ldtk::EntityInstance;
use bevy::{ecs::system::EntityCommands, prelude::*};
use std::{collections::HashMap, marker::PhantomData};

pub trait LdtkEntity: Bundle {
    fn from_instance(
        entity_instance: &EntityInstance,
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> Self;
}

pub trait AddLdtkObjects {
    fn add_ldtk_entity<B: LdtkEntity>(&mut self, identifier: &str) -> &mut App;
}

impl AddLdtkObjects for App {
    fn add_ldtk_entity<B: LdtkEntity>(&mut self, identifier: &str) -> &mut App {
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
        asset_server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    ) -> EntityCommands<'w, 's, 'a> {
        commands.spawn_bundle(B::from_instance(
            entity_instance,
            asset_server,
            materials,
            texture_atlases,
        ))
    }
}
