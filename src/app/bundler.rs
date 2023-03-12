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

pub type BoxedIntoBundle = Box<dyn IntoBundle<Bundle = Box<dyn Bundle>>>;

pub type BoxedBundler = Box<dyn ReadOnlySystem<In = LdtkEntityMetadata, Out = BoxedIntoBundle>>;

struct LdtkEntityBundlerRegistry {
    map: HashMap<String, BoxedBundler>,
}

mod test {
    use crate::LevelSelection;

    use super::*;

    fn from_entity_instance(
        In(metadata): In<LdtkEntityMetadata>,
        _level_selection: Res<LevelSelection>,
    ) -> EntityInstance {
        metadata.entity_instance.clone()
    }

    fn implements_ldtk_entity_bundler<T, B, Marker>()
    where
        T: LdtkEntityBundler<B, Marker>,
        B: Bundle,
    {
    }

    fn new_bundler<B: Bundle, M>(condition: impl LdtkEntityBundler<B, M>) -> BoxedBundler {
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
        let metadata = LdtkEntityMetadata::default();

        let mut boxed = new_bundler(from_entity_instance);

        let entity_instance = boxed.run(metadata, &mut app.world);

        //from_entity_instance::<EntityInstance>.run(&metadata, &mut app.world);
    }
}
