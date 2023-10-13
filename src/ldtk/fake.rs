use std::{any::TypeId, ops::Range};

use crate::ldtk::{LayerInstance, Level, World};
use derive_more::Constructor;
use fake::{faker::lorem::en::Words, uuid::UUIDv4, Dummy, Fake, Faker};
use rand::Rng;

use super::LdtkJson;

pub struct LdtkIdentifierFaker {
    pub num_words: Range<usize>,
}

impl Dummy<LdtkIdentifierFaker> for String {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &LdtkIdentifierFaker, rng: &mut R) -> Self {
        let words: Vec<String> = Words(config.num_words.clone()).fake_with_rng(rng);
        words.join("_")
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct LayerFaker {
    pub identifier: String,
}

impl Dummy<Faker> for LayerFaker {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let identifier = LdtkIdentifierFaker { num_words: 2..5 }.fake_with_rng(rng);
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

#[derive(Copy, Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct UnloadedLevelFaker;

impl Dummy<UnloadedLevelFaker> for Level {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &UnloadedLevelFaker, rng: &mut R) -> Level {
        let faker = Faker;
        Level {
            iid: UUIDv4.fake_with_rng(rng),
            uid: faker.fake_with_rng(rng),
            identifier: LdtkIdentifierFaker { num_words: 2..5 }.fake_with_rng(rng),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct UnloadedLevelsFaker {
    pub num_levels: Range<usize>,
}

impl Dummy<UnloadedLevelsFaker> for Vec<Level> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &UnloadedLevelsFaker, rng: &mut R) -> Vec<Level> {
        Fake::fake_with_rng(&(UnloadedLevelFaker, config.num_levels.clone()), rng)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Constructor)]
pub struct LoadedLevelFaker {
    pub layer_fakers: Vec<LayerFaker>,
}

impl Dummy<LoadedLevelFaker> for Level {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &LoadedLevelFaker, rng: &mut R) -> Level {
        Level {
            layer_instances: Some(
                config
                    .layer_fakers
                    .iter()
                    .map(|layer_faker| layer_faker.fake_with_rng(rng))
                    .collect(),
            ),
            ..UnloadedLevelFaker.fake_with_rng(rng)
        }
    }
}

impl Dummy<Faker> for Level {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        let layer_fakers: Vec<LayerFaker> = (Faker, 2..5).fake_with_rng(rng);
        LoadedLevelFaker { layer_fakers }.fake_with_rng(rng)
    }
}

// Config for faking a collection of loaded levels
//
// Just like a real LDtk file, each level should have the same set of layers.
// Corresponding layers between levels should have some values equal, others different.
//
// We cannot implement `Dummy<Faker>` for `Vec<Level>` since these are foreign types.
// So, the fields here are optional.
// In their absence, they will be faked or use a sensible default, like a Dummy<Faker> impl would.
#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct LoadedLevelsFaker {
    // If None, a default range of 4..8 is used
    pub num_levels: Option<Range<usize>>,
    // If None, 2-5 layer fakers are faked here
    pub layer_fakers: Option<Vec<LayerFaker>>,
}

impl Dummy<LoadedLevelsFaker> for Vec<Level> {
    fn dummy_with_rng<R: Rng + ?Sized>(config: &LoadedLevelsFaker, rng: &mut R) -> Vec<Level> {
        (
            LoadedLevelFaker {
                layer_fakers: config
                    .layer_fakers
                    .clone()
                    .unwrap_or((Faker, 2..5).fake_with_rng(rng)),
            },
            config.num_levels.clone().unwrap_or(4..8),
        )
            .fake_with_rng(rng)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct WorldFaker<L>
where
    Vec<Level>: Dummy<L>,
{
    pub levels_faker: L,
}

impl<L> Dummy<WorldFaker<L>> for World
where
    Vec<Level>: Dummy<L>,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &WorldFaker<L>, rng: &mut R) -> Self {
        World {
            iid: UUIDv4.fake_with_rng(rng),
            identifier: LdtkIdentifierFaker { num_words: 2..5 }.fake_with_rng(rng),
            levels: config.levels_faker.fake_with_rng(rng),
            ..Default::default()
        }
    }
}

impl Dummy<Faker> for World {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        WorldFaker {
            levels_faker: LoadedLevelsFaker::default(),
        }
        .fake_with_rng(rng)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct RootLevelsLdtkJsonFaker<L>
where
    Vec<Level>: Dummy<L>,
{
    pub levels_faker: L,
}

impl<L> Dummy<RootLevelsLdtkJsonFaker<L>> for LdtkJson
where
    Vec<Level>: Dummy<L>,
    L: 'static,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &RootLevelsLdtkJsonFaker<L>, rng: &mut R) -> Self {
        LdtkJson {
            iid: UUIDv4.fake_with_rng(rng),
            levels: config.levels_faker.fake_with_rng(rng),
            external_levels: TypeId::of::<L>() == TypeId::of::<UnloadedLevelsFaker>(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct WorldLevelsLdtkJsonFaker<L>
where
    Vec<Level>: Dummy<L>,
{
    pub levels_faker: L,
    pub num_worlds: Range<usize>,
}

impl<L> Dummy<WorldLevelsLdtkJsonFaker<L>> for LdtkJson
where
    Vec<Level>: Dummy<L>,
    L: Clone + 'static,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &WorldLevelsLdtkJsonFaker<L>, rng: &mut R) -> Self {
        LdtkJson {
            iid: UUIDv4.fake_with_rng(rng),
            worlds: (
                WorldFaker {
                    levels_faker: config.levels_faker.clone(),
                },
                config.num_worlds.clone(),
            )
                .fake_with_rng(rng),
            external_levels: TypeId::of::<L>() == TypeId::of::<UnloadedLevelsFaker>(),
            ..Default::default()
        }
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct MixedLevelsLdtkJsonFaker<L>
where
    Vec<Level>: Dummy<L>,
{
    pub levels_faker: L,
    pub num_worlds: Range<usize>,
}

impl<L> Dummy<MixedLevelsLdtkJsonFaker<L>> for LdtkJson
where
    Vec<Level>: Dummy<L>,
    L: Clone + 'static,
{
    fn dummy_with_rng<R: Rng + ?Sized>(config: &MixedLevelsLdtkJsonFaker<L>, rng: &mut R) -> Self {
        LdtkJson {
            iid: UUIDv4.fake_with_rng(rng),
            levels: config.levels_faker.fake_with_rng(rng),
            worlds: (
                WorldFaker {
                    levels_faker: config.levels_faker.clone(),
                },
                config.num_worlds.clone(),
            )
                .fake_with_rng(rng),
            external_levels: TypeId::of::<L>() == TypeId::of::<UnloadedLevelsFaker>(),
            ..Default::default()
        }
    }
}

impl Dummy<Faker> for LdtkJson {
    fn dummy_with_rng<R: Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
        RootLevelsLdtkJsonFaker {
            levels_faker: LoadedLevelsFaker::default(),
        }
        .fake_with_rng(rng)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct RootLevelsLdtkJsonWithExternalLevelsFaker {
    pub ldtk_json_faker: RootLevelsLdtkJsonFaker<LoadedLevelsFaker>,
}

impl Dummy<RootLevelsLdtkJsonWithExternalLevelsFaker> for (LdtkJson, Vec<Level>) {
    fn dummy_with_rng<R: Rng + ?Sized>(
        config: &RootLevelsLdtkJsonWithExternalLevelsFaker,
        rng: &mut R,
    ) -> Self {
        let mut ldtk_json: LdtkJson = config.ldtk_json_faker.fake_with_rng(rng);
        let levels = ldtk_json.levels.clone();

        ldtk_json
            .levels
            .iter_mut()
            .for_each(|level| level.layer_instances = None);

        ldtk_json.external_levels = true;

        (ldtk_json, levels)
    }
}

#[derive(Clone, Default, Debug, PartialEq, Eq, Constructor)]
pub struct WorldLevelsLdtkJsonWithExternalLevelsFaker {
    pub ldtk_json_faker: WorldLevelsLdtkJsonFaker<LoadedLevelsFaker>,
}

impl Dummy<WorldLevelsLdtkJsonWithExternalLevelsFaker> for (LdtkJson, Vec<Level>) {
    fn dummy_with_rng<R: Rng + ?Sized>(
        config: &WorldLevelsLdtkJsonWithExternalLevelsFaker,
        rng: &mut R,
    ) -> Self {
        let mut ldtk_json: LdtkJson = config.ldtk_json_faker.fake_with_rng(rng);
        let levels = ldtk_json
            .worlds
            .iter()
            .flat_map(|world| world.levels.iter().cloned())
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
