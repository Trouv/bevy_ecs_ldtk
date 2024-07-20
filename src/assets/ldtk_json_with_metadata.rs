use crate::{
    assets::{level_locale::LevelLocale, LevelIndices, LevelMetadata, LevelMetadataAccessor},
    ldtk::{
        loaded_level::LoadedLevel, raw_level_accessor::RawLevelAccessor, LdtkJson, Level, World,
    },
    resources::LevelSelection,
};
use bevy::reflect::{Reflect, TypePath};
use derive_getters::Getters;
use std::collections::HashMap;

#[cfg(feature = "internal_levels")]
use crate::assets::InternalLevels;

#[cfg(feature = "external_levels")]
use crate::assets::{ExternalLevels, LdtkExternalLevel};
#[cfg(feature = "external_levels")]
use bevy::prelude::*;

#[cfg(feature = "internal_levels")]
fn expect_level_loaded(level: &Level) -> LoadedLevel {
    LoadedLevel::try_from(level)
        .expect("LdtkProject construction should guarantee that internal levels are loaded")
}

/// LDtk json data and level metadata produced when loading an [`LdtkProject`] asset.
///
/// Generic over a level-locale marker type, `L`.
/// This helps differentiate between internal- and external-level projects.
/// `L` will can only be either [`InternalLevels`] or [`ExternalLevels`].
/// This provides some abstraction over the two cases, but they are ultimately different types.
/// Some methods are exclusive to each case, especially for obtaining [`LoadedLevel`]s.
/// See the [`LoadedLevel`]-accessing methods in the following impls:
/// - [internal-levels](LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<InternalLevels>)
/// - [external-levels](LdtkJsonWithMetadata#impl-LdtkJsonWithMetadata<ExternalLevels>)
///
/// [`LdtkProject`]: crate::assets::LdtkProject
#[derive(Clone, Debug, PartialEq, Getters, Reflect)]
pub struct LdtkJsonWithMetadata<L>
where
    L: LevelLocale,
    L::Metadata: TypePath,
{
    /// Raw ldtk json data.
    json_data: LdtkJson,
    /// Map from level iids to level metadata.
    level_map: HashMap<String, L::Metadata>,
}

impl<L> LdtkJsonWithMetadata<L>
where
    L: LevelLocale,
    L::Metadata: TypePath,
{
    /// Construct a new [`LdtkJsonWithMetadata`].
    ///
    /// Only public to the crate to preserve type guarantees about loaded levels.
    pub(crate) fn new(
        json_data: LdtkJson,
        level_map: HashMap<String, L::Metadata>,
    ) -> LdtkJsonWithMetadata<L> {
        LdtkJsonWithMetadata {
            json_data,
            level_map,
        }
    }
}

impl<L> RawLevelAccessor for LdtkJsonWithMetadata<L>
where
    L: LevelLocale,
    L::Metadata: TypePath,
{
    fn root_levels(&self) -> &[Level] {
        self.json_data.root_levels()
    }

    fn worlds(&self) -> &[World] {
        self.json_data.worlds()
    }
}

#[cfg(feature = "internal_levels")]
impl LevelMetadataAccessor for LdtkJsonWithMetadata<InternalLevels> {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        self.level_map.get(iid)
    }
}

#[cfg(feature = "internal_levels")]
impl LdtkJsonWithMetadata<InternalLevels> {
    /// Iterate through this project's loaded levels.
    ///
    /// This first iterates through [root levels, then world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn iter_loaded_levels(&self) -> impl Iterator<Item = LoadedLevel> {
        self.iter_raw_levels().map(expect_level_loaded)
    }

    /// Immutable access to a loaded level at the given [`LevelIndices`].
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn get_loaded_level_at_indices(&self, indices: &LevelIndices) -> Option<LoadedLevel> {
        self.get_raw_level_at_indices(indices)
            .map(expect_level_loaded)
    }

    /// Returns a reference to the loaded level metadata corresponding to the given level iid.
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn get_loaded_level_by_iid(&self, iid: &String) -> Option<LoadedLevel> {
        self.get_raw_level_by_iid(iid).map(expect_level_loaded)
    }

    /// Find the loaded level matching the given [`LevelSelection`].
    ///
    /// This lookup is constant for [`LevelSelection::Iid`] and [`LevelSelection::Indices`] variants.
    /// The other variants require iterating through the levels to find the match.
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn find_loaded_level_by_level_selection(
        &self,
        level_selection: &LevelSelection,
    ) -> Option<LoadedLevel> {
        self.find_raw_level_by_level_selection(level_selection)
            .map(expect_level_loaded)
    }
}

#[cfg(feature = "external_levels")]
impl LevelMetadataAccessor for LdtkJsonWithMetadata<ExternalLevels> {
    fn get_level_metadata_by_iid(&self, iid: &String) -> Option<&LevelMetadata> {
        Some(self.level_map.get(iid)?.metadata())
    }
}

#[cfg(feature = "external_levels")]
impl LdtkJsonWithMetadata<ExternalLevels> {
    /// Iterate through this project's external levels.
    ///
    /// This first iterates through [root levels, then world levels](RawLevelAccessor#root-vs-world-levels).
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn iter_external_levels<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
    ) -> impl Iterator<Item = LoadedLevel<'a>> {
        self.iter_raw_levels()
            .filter_map(|level| self.level_map.get(&level.iid))
            .filter_map(|metadata| external_level_assets.get(metadata.external_handle()))
            .map(LdtkExternalLevel::data)
    }

    /// Immutable access to an external level at the given [`LevelIndices`].
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn get_external_level_at_indices<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        indices: &LevelIndices,
    ) -> Option<LoadedLevel<'a>> {
        self.get_external_level_by_iid(
            external_level_assets,
            &self.get_raw_level_at_indices(indices)?.iid,
        )
    }

    /// Returns a reference to the external level metadata corresponding to the given level iid.
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn get_external_level_by_iid<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        iid: &String,
    ) -> Option<LoadedLevel<'a>> {
        self.level_map()
            .get(iid)
            .and_then(|metadata| external_level_assets.get(metadata.external_handle()))
            .map(LdtkExternalLevel::data)
    }

    /// Find the external level matching the given [`LevelSelection`].
    ///
    /// This lookup is constant for [`LevelSelection::Iid`] and [`LevelSelection::Indices`] variants.
    /// The other variants require iterating through the levels to find the match.
    ///
    /// These levels are [loaded], meaning that they are type-guaranteed to have complete data.
    ///
    /// [loaded]: crate::assets::LdtkProject#raw-vs-loaded-levels
    pub fn find_external_level_by_level_selection<'a>(
        &'a self,
        external_level_assets: &'a Assets<LdtkExternalLevel>,
        level_selection: &LevelSelection,
    ) -> Option<LoadedLevel<'a>> {
        match level_selection {
            LevelSelection::Iid(iid) => {
                self.get_external_level_by_iid(external_level_assets, iid.get())
            }
            LevelSelection::Indices(indices) => {
                self.get_external_level_at_indices(external_level_assets, indices)
            }
            _ => self.get_external_level_by_iid(
                external_level_assets,
                &self.find_raw_level_by_level_selection(level_selection)?.iid,
            ),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;
    use derive_more::Constructor;
    use fake::Dummy;

    #[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Constructor)]
    pub struct LdtkJsonWithMetadataFaker<F>
    where
        LdtkJson: Dummy<F>,
    {
        pub ldtk_json_faker: F,
    }

    #[cfg(feature = "internal_levels")]
    mod internal_levels {
        use fake::{Fake, Faker};

        use crate::{
            assets::level_metadata_accessor::tests::BasicLevelMetadataAccessor,
            ldtk::fake::{LoadedLevelsFaker, MixedLevelsLdtkJsonFaker},
            LevelIid,
        };

        use super::*;

        impl<F> Dummy<LdtkJsonWithMetadataFaker<F>> for LdtkJsonWithMetadata<InternalLevels>
        where
            LdtkJson: Dummy<F>,
        {
            fn dummy_with_rng<R: rand::Rng + ?Sized>(
                config: &LdtkJsonWithMetadataFaker<F>,
                rng: &mut R,
            ) -> Self {
                let json_data: LdtkJson = config.ldtk_json_faker.fake_with_rng(rng);
                let level_map = json_data
                    .levels
                    .iter()
                    .enumerate()
                    .map(|(i, level)| {
                        (
                            level.iid.clone(),
                            LevelMetadata::new(None, LevelIndices::in_root(i)),
                        )
                    })
                    .collect();

                LdtkJsonWithMetadata {
                    json_data,
                    level_map,
                }
            }
        }

        impl Dummy<Faker> for LdtkJsonWithMetadata<InternalLevels> {
            fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
                LdtkJsonWithMetadataFaker::new(Faker).fake_with_rng(rng)
            }
        }

        #[test]
        fn raw_level_accessor_implementation_is_transparent() {
            let project: LdtkJsonWithMetadata<InternalLevels> = LdtkJsonWithMetadataFaker::new(
                MixedLevelsLdtkJsonFaker::new(LoadedLevelsFaker::default(), 4..8),
            )
            .fake();

            assert_eq!(project.root_levels(), project.json_data().root_levels());
            assert_eq!(project.worlds(), project.json_data().worlds());
        }

        #[test]
        fn level_metadata_accessor_implementation_is_transparent() {
            let basic = BasicLevelMetadataAccessor::sample_with_root_levels();

            let ldtk_json_with_metadata = LdtkJsonWithMetadata::<InternalLevels> {
                json_data: basic.data.clone(),
                level_map: basic.level_metadata.clone(),
            };

            for level in &basic.data.levels {
                assert_eq!(
                    ldtk_json_with_metadata.get_level_metadata_by_iid(&level.iid),
                    basic.get_level_metadata_by_iid(&level.iid),
                );
            }

            assert_eq!(
                ldtk_json_with_metadata
                    .get_level_metadata_by_iid(&"This_level_doesnt_exist".to_string()),
                None
            );
        }

        #[test]
        fn loaded_level_iteration() {
            let project: LdtkJsonWithMetadata<InternalLevels> = Faker.fake();

            assert_eq!(
                project.iter_loaded_levels().count(),
                project.json_data.levels.len()
            );

            for (loaded_level, expected_level) in project
                .iter_loaded_levels()
                .zip(project.json_data.levels.iter())
            {
                assert_eq!(loaded_level.raw(), expected_level)
            }
        }

        #[test]
        fn indices_lookup_returns_expected_loaded_levels() {
            let project: LdtkJsonWithMetadata<InternalLevels> = Faker.fake();

            for (i, expected_level) in project.json_data.levels.iter().enumerate() {
                assert_eq!(
                    project
                        .get_loaded_level_at_indices(&LevelIndices::in_root(i))
                        .unwrap()
                        .raw(),
                    expected_level
                );
            }

            assert_eq!(
                project.get_loaded_level_at_indices(&LevelIndices::in_root(10)),
                None
            );
            assert_eq!(
                project.get_loaded_level_at_indices(&LevelIndices::in_world(0, 0)),
                None
            );
        }

        #[test]
        fn iid_lookup_returns_expected_loaded_levels() {
            let project: LdtkJsonWithMetadata<InternalLevels> = Faker.fake();

            for expected_level in &project.json_data.levels {
                assert_eq!(
                    project
                        .get_loaded_level_by_iid(&expected_level.iid)
                        .unwrap()
                        .raw(),
                    expected_level
                );
            }

            assert_eq!(
                project
                    .get_loaded_level_by_iid(&"cd51071d-5224-4628-ae0d-abbe28090521".to_string()),
                None
            )
        }

        #[test]
        fn find_by_level_selection_returns_expected_loaded_levels() {
            let project: LdtkJsonWithMetadata<InternalLevels> = Faker.fake();

            for (i, expected_level) in project.json_data.levels.iter().enumerate() {
                assert_eq!(
                    project
                        .find_loaded_level_by_level_selection(&LevelSelection::index(i))
                        .unwrap()
                        .raw(),
                    expected_level
                );
                assert_eq!(
                    project
                        .find_loaded_level_by_level_selection(&LevelSelection::Identifier(
                            expected_level.identifier.clone()
                        ))
                        .unwrap()
                        .raw(),
                    expected_level
                );
                assert_eq!(
                    project
                        .find_loaded_level_by_level_selection(&LevelSelection::Iid(LevelIid::new(
                            expected_level.iid.clone()
                        )))
                        .unwrap()
                        .raw(),
                    expected_level
                );
                assert_eq!(
                    project
                        .find_loaded_level_by_level_selection(&LevelSelection::Uid(
                            expected_level.uid
                        ))
                        .unwrap()
                        .raw(),
                    expected_level
                );
            }

            assert_eq!(
                project.find_loaded_level_by_level_selection(&LevelSelection::index(10)),
                None
            );
            assert_eq!(
                project.find_loaded_level_by_level_selection(&LevelSelection::Identifier(
                    "Back_Rooms".to_string()
                )),
                None
            );
            assert_eq!(
                project.find_loaded_level_by_level_selection(&LevelSelection::Iid(LevelIid::new(
                    "cd51071d-5224-4628-ae0d-abbe28090521".to_string()
                ))),
                None
            );
            assert_eq!(
                project.find_loaded_level_by_level_selection(&LevelSelection::Uid(2023)),
                None,
            );
        }
    }

    #[cfg(feature = "external_levels")]
    mod external_levels {
        use super::*;
        use crate::{
            assets::{
                level_metadata_accessor::tests::BasicLevelMetadataAccessor, ExternalLevelMetadata,
            },
            ldtk::fake::{
                LoadedLevelsFaker, MixedLevelsLdtkJsonFaker, RootLevelsLdtkJsonFaker,
                RootLevelsLdtkJsonWithExternalLevelsFaker, UnloadedLevelsFaker,
            },
            LevelIid,
        };
        use fake::{Fake, Faker};

        impl<F> Dummy<LdtkJsonWithMetadataFaker<F>> for LdtkJsonWithMetadata<ExternalLevels>
        where
            LdtkJson: Dummy<F>,
        {
            fn dummy_with_rng<R: rand::Rng + ?Sized>(
                config: &LdtkJsonWithMetadataFaker<F>,
                rng: &mut R,
            ) -> Self {
                let json_data: LdtkJson = config.ldtk_json_faker.fake_with_rng(rng);
                let level_map = json_data
                    .levels
                    .iter()
                    .enumerate()
                    .map(|(i, level)| {
                        (
                            level.iid.clone(),
                            ExternalLevelMetadata::new(
                                LevelMetadata::new(None, LevelIndices::in_root(i)),
                                Handle::weak_from_u128(Faker.fake()),
                            ),
                        )
                    })
                    .collect();

                LdtkJsonWithMetadata {
                    json_data,
                    level_map,
                }
            }
        }

        impl Dummy<Faker> for LdtkJsonWithMetadata<ExternalLevels> {
            fn dummy_with_rng<R: rand::Rng + ?Sized>(_: &Faker, rng: &mut R) -> Self {
                LdtkJsonWithMetadataFaker::new(RootLevelsLdtkJsonFaker::new(
                    UnloadedLevelsFaker::new(4..8),
                ))
                .fake_with_rng(rng)
            }
        }

        fn app_setup() -> App {
            let mut app = App::new();
            app.add_plugins(AssetPlugin::default())
                .init_asset::<LdtkExternalLevel>();

            app
        }

        fn fake_and_load_ldtk_json_with_metadata(
            app: &mut App,
        ) -> LdtkJsonWithMetadata<ExternalLevels> {
            let (json_data, levels): (LdtkJson, Vec<Level>) =
                RootLevelsLdtkJsonWithExternalLevelsFaker::new(RootLevelsLdtkJsonFaker::new(
                    LoadedLevelsFaker::default(),
                ))
                .fake();

            let mut assets = app
                .world_mut()
                .get_resource_mut::<Assets<LdtkExternalLevel>>()
                .unwrap();

            let level_map = levels
                .iter()
                .enumerate()
                .map(|(i, level)| {
                    (
                        level.iid.clone(),
                        ExternalLevelMetadata::new(
                            LevelMetadata::new(None, LevelIndices::in_root(i)),
                            assets.add(LdtkExternalLevel::new(level.clone())),
                        ),
                    )
                })
                .collect();

            LdtkJsonWithMetadata {
                json_data,
                level_map,
            }
        }

        #[test]
        fn raw_level_accessor_implementation_is_transparent() {
            let project: LdtkJsonWithMetadata<ExternalLevels> = LdtkJsonWithMetadataFaker::new(
                MixedLevelsLdtkJsonFaker::new(LoadedLevelsFaker::default(), 4..8),
            )
            .fake();

            assert_eq!(project.root_levels(), project.json_data().root_levels());
            assert_eq!(project.worlds(), project.json_data().worlds());
        }

        #[test]
        fn external_level_metadata_accessor_is_transparent() {
            let basic = BasicLevelMetadataAccessor::sample_with_root_levels();

            let ldtk_json_with_metadata = LdtkJsonWithMetadata::<ExternalLevels> {
                json_data: basic.data.clone(),
                level_map: basic
                    .level_metadata
                    .clone()
                    .into_iter()
                    .map(|(iid, level_metadata)| {
                        (
                            iid,
                            ExternalLevelMetadata::new(level_metadata, Handle::default()),
                        )
                    })
                    .collect(),
            };

            for level in &basic.data.levels {
                assert_eq!(
                    ldtk_json_with_metadata.get_level_metadata_by_iid(&level.iid),
                    basic.get_level_metadata_by_iid(&level.iid),
                );
            }

            assert_eq!(
                ldtk_json_with_metadata
                    .get_level_metadata_by_iid(&"This_level_doesnt_exist".to_string()),
                None
            );
        }

        #[test]
        fn external_level_iteration() {
            let mut app = app_setup();
            let project = fake_and_load_ldtk_json_with_metadata(&mut app);

            assert_eq!(
                project
                    .iter_external_levels(
                        app.world()
                            .get_resource::<Assets<LdtkExternalLevel>>()
                            .unwrap()
                    )
                    .count(),
                project.json_data.levels.len()
            );

            for (external_level, expected_level) in project
                .iter_external_levels(
                    app.world()
                        .get_resource::<Assets<LdtkExternalLevel>>()
                        .unwrap(),
                )
                .zip(project.json_data.levels.iter())
            {
                assert_eq!(external_level.iid(), &expected_level.iid)
            }
        }

        #[test]
        fn indices_lookup_returns_expected_external_levels() {
            let mut app = app_setup();
            let project = fake_and_load_ldtk_json_with_metadata(&mut app);

            let assets = app
                .world()
                .get_resource::<Assets<LdtkExternalLevel>>()
                .unwrap();

            for (i, expected_level) in project.json_data.levels.iter().enumerate() {
                assert_eq!(
                    project
                        .get_external_level_at_indices(assets, &LevelIndices::in_root(i))
                        .unwrap()
                        .iid(),
                    &expected_level.iid
                );
            }

            assert_eq!(
                project.get_external_level_at_indices(assets, &LevelIndices::in_root(10)),
                None
            );
            assert_eq!(
                project.get_external_level_at_indices(assets, &LevelIndices::in_world(0, 0)),
                None
            );
        }

        #[test]
        fn iid_lookup_returns_expected_external_levels() {
            let mut app = app_setup();
            let project = fake_and_load_ldtk_json_with_metadata(&mut app);

            let assets = app
                .world()
                .get_resource::<Assets<LdtkExternalLevel>>()
                .unwrap();

            for expected_level in &project.json_data.levels {
                assert_eq!(
                    project
                        .get_external_level_by_iid(assets, &expected_level.iid)
                        .unwrap()
                        .iid(),
                    &expected_level.iid
                );
            }

            assert_eq!(
                project.get_external_level_by_iid(
                    assets,
                    &"cd51071d-5224-4628-ae0d-abbe28090521".to_string()
                ),
                None
            )
        }

        #[test]
        fn find_by_level_selection_returns_expected_external_levels() {
            let mut app = app_setup();
            let project = fake_and_load_ldtk_json_with_metadata(&mut app);

            let assets = app
                .world()
                .get_resource::<Assets<LdtkExternalLevel>>()
                .unwrap();

            for (i, expected_level) in project.json_data.levels.iter().enumerate() {
                assert_eq!(
                    project
                        .find_external_level_by_level_selection(assets, &LevelSelection::index(i))
                        .unwrap()
                        .iid(),
                    &expected_level.iid
                );
                assert_eq!(
                    project
                        .find_external_level_by_level_selection(
                            assets,
                            &LevelSelection::Identifier(expected_level.identifier.clone())
                        )
                        .unwrap()
                        .iid(),
                    &expected_level.iid
                );
                assert_eq!(
                    project
                        .find_external_level_by_level_selection(
                            assets,
                            &LevelSelection::Iid(LevelIid::new(expected_level.iid.clone()))
                        )
                        .unwrap()
                        .iid(),
                    &expected_level.iid
                );
                assert_eq!(
                    project
                        .find_external_level_by_level_selection(
                            assets,
                            &LevelSelection::Uid(expected_level.uid)
                        )
                        .unwrap()
                        .iid(),
                    &expected_level.iid
                );
            }

            assert_eq!(
                project.find_external_level_by_level_selection(assets, &LevelSelection::index(10)),
                None
            );
            assert_eq!(
                project.find_external_level_by_level_selection(
                    assets,
                    &LevelSelection::Identifier("Back_Rooms".to_string())
                ),
                None
            );
            assert_eq!(
                project.find_external_level_by_level_selection(
                    assets,
                    &LevelSelection::Iid(LevelIid::new(
                        "cd51071d-5224-4628-ae0d-abbe28090521".to_string()
                    ))
                ),
                None
            );
            assert_eq!(
                project.find_external_level_by_level_selection(assets, &LevelSelection::Uid(2023)),
                None,
            );
        }
    }
}
