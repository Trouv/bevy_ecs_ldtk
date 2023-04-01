use bevy::{ecs::system::ReadOnlySystem, prelude::*, utils::HashMap};

use crate::{ldtk::Level, EntityInstance, LayerMetadata};

#[derive(Default)]
pub struct LdtkEntityMetadata {
    pub entity_instance: EntityInstance,
    pub layer_metadata: LayerMetadata,
    pub level_metadata: Level,
}

pub trait LdtkEntityBundler<B, Marker>:
    IntoSystem<LdtkEntityMetadata, B, Marker, System = Self::ReadOnlySystem>
where
    B: Bundle,
{
    type ReadOnlySystem: ReadOnlySystem<In = LdtkEntityMetadata, Out = B>;
}

impl<B, Marker, F> LdtkEntityBundler<B, Marker> for F
where
    B: Bundle,
    F: IntoSystem<LdtkEntityMetadata, B, Marker>,
    F::System: ReadOnlySystem,
{
    type ReadOnlySystem = F::System;
}

pub trait IntoBundle {
    type Bundle: Bundle;

    fn into_bundle(self) -> Self::Bundle;
}

pub type BoxedBundler<B> = Box<dyn ReadOnlySystem<In = LdtkEntityMetadata, Out = B>>;

#[derive(Resource)]
struct LdtkEntityBundlerRegistry<B> {
    map: HashMap<String, BoxedBundler<B>>,
}

#[derive(Default)]
struct LayerTree {
    layer_metadata: LayerMetadata,
    entities: Vec<EntityInstance>,
}

#[derive(Default)]
struct MetadataTree {
    level_metadata: Level,
    layers: Vec<LayerTree>,
}

fn ldtk_entity_bundler_pipe_wrapper<B: Bundle>(
    In(metadata_tree): In<MetadataTree>,
    mut commands: Commands,
    world: &World,
    mut registry: ResMut<LdtkEntityBundlerRegistry<B>>,
) -> MetadataTree {
    let mut bundles: Vec<B> = Vec::new();
    for layer in metadata_tree.layers.iter() {
        for entity in layer.entities.iter() {
            if let Some(boxed_bundler) = registry.map.get_mut(&entity.identifier) {
                let metadata = LdtkEntityMetadata {
                    level_metadata: metadata_tree.level_metadata.clone(),
                    layer_metadata: layer.layer_metadata.clone(),
                    entity_instance: entity.clone(),
                };

                unsafe {
                    bundles.push(boxed_bundler.run_unsafe(metadata, world));
                }
            }
        }
    }

    commands.spawn_batch(bundles);

    metadata_tree
}

fn consume_metadata(_: In<MetadataTree>) {}

fn default_metadata_tree() -> MetadataTree {
    MetadataTree::default()
}

mod test {
    use crate::LevelSelection;

    use super::*;

    fn new_bundler<B: Bundle, M>(condition: impl LdtkEntityBundler<B, M>) -> BoxedBundler<B> {
        let bundler_system = IntoSystem::into_system(condition);
        assert!(
            bundler_system.is_send(),
            "Condition `{}` accesses `NonSend` resources. This is not currently supported.",
            bundler_system.name()
        );

        Box::new(bundler_system)
    }

    fn test() {
        let mut app = App::new();

        app.add_system(
            default_metadata_tree
                .pipe(ldtk_entity_bundler_pipe_wrapper::<SpriteSheetBundle>)
                .pipe(consume_metadata),
        );

        //from_entity_instance::<EntityInstance>.run(&metadata, &mut app.world);
    }
}
