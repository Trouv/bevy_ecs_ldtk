use std::{any::TypeId, ops::Range};

use crate::ldtk::{LayerInstance, Level, World};
use fake::{faker::lorem::en::Words, uuid::UUIDv4, Dummy, Fake, Faker};
use rand::Rng;

use super::LdtkJson;

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LayerFaker {
    pub identifier: String,
}

impl Dummy<Faker> for LayerFaker {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let identifier = Words(2..5).fake_with_rng::<Vec<String>, R>(rng).join("_");
        LayerFaker { identifier }
    }
}

impl Dummy<LayerFaker> for LayerInstance {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &LayerFaker, rng: &mut R) -> Self {
        LayerInstance {
            iid: UUIDv4.fake_with_rng(rng),
            identifier: config.identifier.clone(),
            ..Default::default()
        }
    }
}

impl Dummy<Faker> for LayerInstance {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let layer_faker: LayerFaker = Faker.fake_with_rng(rng);
        layer_faker.fake_with_rng(rng)
    }
}

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq)]
pub struct UnloadedLevelFaker;

impl Dummy<UnloadedLevelFaker> for Level {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UnloadedLevelFaker, rng: &mut R) -> Level {
        let faker = Faker;
        Level {
            iid: UUIDv4.fake_with_rng(rng),
            uid: faker.fake_with_rng(rng),
            identifier: Words(2..5).fake_with_rng::<Vec<String>, R>(rng).join("_"),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct UnloadedLevelsFaker(pub Range<usize>);

impl Dummy<UnloadedLevelsFaker> for Vec<Level> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &UnloadedLevelsFaker, rng: &mut R) -> Vec<Level> {
        Fake::fake_with_rng(&(UnloadedLevelFaker, config.0.clone()), rng)
    }
}

#[derive(Clone, Default, Debug, PartialEq)]
pub struct LoadedLevelFaker(pub Vec<LayerInstance>);

impl Dummy<LoadedLevelFaker> for Level {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &LoadedLevelFaker, rng: &mut R) -> Level {
        Level {
            layer_instances: Some(config.0.clone()),
            ..UnloadedLevelFaker.fake_with_rng(rng)
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct LoadedLevelsFaker(pub Range<usize>);

impl Dummy<LoadedLevelsFaker> for Vec<Level> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &LoadedLevelsFaker, rng: &mut R) -> Vec<Level> {
        let layers = Fake::fake_with_rng(&(Faker, 2..5), rng);
        Fake::fake_with_rng(&(LoadedLevelFaker(layers), config.0.clone()), rng)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct WorldFaker<L>(pub L)
where
    Vec<Level>: Dummy<L>;

impl<L> Dummy<WorldFaker<L>> for World
where
    Vec<Level>: Dummy<L>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &WorldFaker<L>, rng: &mut R) -> Self {
        World {
            iid: UUIDv4.fake_with_rng(rng),
            identifier: Words(2..5).fake_with_rng::<Vec<String>, R>(rng).join("_"),
            levels: config.0.fake_with_rng(rng),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RootLevelsLdtkJsonFaker<L>(pub L)
where
    Vec<Level>: Dummy<L>;

impl<L> Dummy<RootLevelsLdtkJsonFaker<L>> for LdtkJson
where
    Vec<Level>: Dummy<L>,
    L: 'static,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &RootLevelsLdtkJsonFaker<L>, rng: &mut R) -> Self {
        LdtkJson {
            iid: UUIDv4.fake_with_rng(rng),
            levels: config.0.fake_with_rng(rng),
            external_levels: TypeId::of::<L>() == TypeId::of::<UnloadedLevelsFaker>(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct WorldLevelsLdtkJsonFaker<L>(pub L, pub Range<usize>)
where
    Vec<Level>: Dummy<L>;

impl<L> Dummy<WorldLevelsLdtkJsonFaker<L>> for LdtkJson
where
    Vec<Level>: Dummy<L>,
    L: Clone + 'static,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &WorldLevelsLdtkJsonFaker<L>, rng: &mut R) -> Self {
        LdtkJson {
            iid: UUIDv4.fake_with_rng(rng),
            worlds: Fake::fake_with_rng(&(WorldFaker(config.0.clone()), config.1.clone()), rng),
            external_levels: TypeId::of::<L>() == TypeId::of::<UnloadedLevelsFaker>(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct MixedLevelsLdtkJsonFaker<L>(pub L, pub Range<usize>)
where
    Vec<Level>: Dummy<L>;

impl<L> Dummy<MixedLevelsLdtkJsonFaker<L>> for LdtkJson
where
    Vec<Level>: Dummy<L>,
    L: Clone + 'static,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &MixedLevelsLdtkJsonFaker<L>, rng: &mut R) -> Self {
        LdtkJson {
            iid: UUIDv4.fake_with_rng(rng),
            levels: config.0.fake_with_rng(rng),
            worlds: Fake::fake_with_rng(&(WorldFaker(config.0.clone()), config.1.clone()), rng),
            external_levels: TypeId::of::<L>() == TypeId::of::<UnloadedLevelsFaker>(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct RootLevelsLdtkJsonWithExternalLevelsFaker(
    pub RootLevelsLdtkJsonFaker<LoadedLevelsFaker>,
);

impl Dummy<RootLevelsLdtkJsonWithExternalLevelsFaker> for (LdtkJson, Vec<Level>) {
    fn dummy_with_rng<R: Rng + ?Sized>(
        config: &RootLevelsLdtkJsonWithExternalLevelsFaker,
        rng: &mut R,
    ) -> Self {
        let mut ldtk_json: LdtkJson = config.0.fake_with_rng(rng);
        let levels = ldtk_json.levels.clone();

        ldtk_json
            .levels
            .iter_mut()
            .for_each(|level| level.layer_instances = None);

        ldtk_json.external_levels = true;

        (ldtk_json, levels)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub struct WorldLevelsLdtkJsonWithExternalLevelsFaker(
    pub WorldLevelsLdtkJsonFaker<LoadedLevelsFaker>,
);

impl Dummy<WorldLevelsLdtkJsonWithExternalLevelsFaker> for (LdtkJson, Vec<Level>) {
    fn dummy_with_rng<R: Rng + ?Sized>(
        config: &WorldLevelsLdtkJsonWithExternalLevelsFaker,
        rng: &mut R,
    ) -> Self {
        let mut ldtk_json: LdtkJson = config.0.fake_with_rng(rng);
        let levels = ldtk_json
            .worlds
            .iter()
            .map(|world| world.levels.iter().cloned())
            .flatten()
            .collect();

        ldtk_json.worlds.iter_mut().for_each(|world| {
            world
                .levels
                .iter_mut()
                .for_each(|level| level.layer_instances = None)
        });

        ldtk_json.external_levels = true;

        (ldtk_json, levels)
    }
}
