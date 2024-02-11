# Changelog

## [0.9.0](https://github.com/Trouv/bevy_ecs_ldtk/compare/v0.8.0...v0.9.0) (2024-02-11)


### ⚠ BREAKING CHANGES

* upgrade to bevy 0.12 ([#265](https://github.com/Trouv/bevy_ecs_ldtk/issues/265))
* upgrade to LDtk 1.5.3, dropping support for previous versions ([#295](https://github.com/Trouv/bevy_ecs_ldtk/issues/295))
* add `SpawnExclusions` to `LdtkSettings` for skipping layers by identifier ([#275](https://github.com/Trouv/bevy_ecs_ldtk/issues/275))
* add layer entity for Entity layers, changing the hierarchy ([#257](https://github.com/Trouv/bevy_ecs_ldtk/issues/257))
* upgrade to LDtk types and examples to 1.4.1 (drop support for <1.4.1) ([#256](https://github.com/Trouv/bevy_ecs_ldtk/issues/256))
* LdtkLevel renamed to LdtkExternalLevel and is no longer used as a component ([#244](https://github.com/Trouv/bevy_ecs_ldtk/issues/244))
* redesign LdtkProject with better level data accessors and correct modeling of internal/external levels ([#244](https://github.com/Trouv/bevy_ecs_ldtk/issues/244))
* use the bundle's `Default` implementation rather than the field's in `LdtkEntity` and `LdtkIntCell` derive macros ([#222](https://github.com/Trouv/bevy_ecs_ldtk/issues/222))
* add `RawLevelAccessor` trait for `LdtkJson` level borrowing/iteration, replacing existing methods ([#225](https://github.com/Trouv/bevy_ecs_ldtk/issues/225))
* add `LevelIndices` type defining a level's location in a project and use it in `LevelSelection::Indices` ([#221](https://github.com/Trouv/bevy_ecs_ldtk/issues/221))
* change `LevelEvent` inner types from `String` to `LevelIid` ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219))
* change `LevelSet` inner type from `String` to `LevelIid` ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219))
* change `LevelSelection::Iid` inner type from `String` to `LevelIid` ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219))
* replace `LevelSet::from_iid` with `LevelSet::from_iids`, which can convert from any collection of strings. ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219))
* use new LevelIid type in LevelEvent, LevelSet, and LevelSelection, plus other improvements ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219))
* `LdtkProject::project` and `LdtkLevel::level` fields have both been renamed to `data` ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206))
* All fields of `LdtkProject` and `LdtkLevel` are now privatized, and have immutable getter methods ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206))
* `LevelMap` and `TilesetMap` type aliases have been removed ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206))
* `LdtkAsset` and `LdtkProject` are now exported in new `assets` module instead of `lib.rs` ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206))
* asset `Loader` types are now private ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206))
* `LdtkAsset` renamed to `LdtkProject` ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206))

### Features

* add `LevelIndices` type defining a level's location in a project and use it in `LevelSelection::Indices` ([#221](https://github.com/Trouv/bevy_ecs_ldtk/issues/221)) ([59618fe](https://github.com/Trouv/bevy_ecs_ldtk/commit/59618fe2f406caddd433ec435cff0a2156775c5c))
* add `RawLevelAccessor` trait for `LdtkJson` level borrowing/iteration, replacing existing methods ([#225](https://github.com/Trouv/bevy_ecs_ldtk/issues/225)) ([d3de2d9](https://github.com/Trouv/bevy_ecs_ldtk/commit/d3de2d9d4079865d110af57016258f67ac3f3de8))
* add `SpawnExclusions` to `LdtkSettings` for skipping layers by identifier ([#275](https://github.com/Trouv/bevy_ecs_ldtk/issues/275)) ([282404d](https://github.com/Trouv/bevy_ecs_ldtk/commit/282404d1f472ce2d31fef52d2943525fe1e045b0)), closes [#272](https://github.com/Trouv/bevy_ecs_ldtk/issues/272)
* add layer entity for Entity layers, changing the hierarchy ([#257](https://github.com/Trouv/bevy_ecs_ldtk/issues/257)) ([ee20a53](https://github.com/Trouv/bevy_ecs_ldtk/commit/ee20a53d39aafc282008ed03fb1cf3355f62dd5a))
* add LdtkJsonWithMetadata type for representing internal- and external-level project data with generics ([#242](https://github.com/Trouv/bevy_ecs_ldtk/issues/242)) ([630434a](https://github.com/Trouv/bevy_ecs_ldtk/commit/630434a417eec89bed2dc1c5076a62e8ca46ca96))
* add LdtkProjectData for representing either internal- or external-level project data concretely ([#243](https://github.com/Trouv/bevy_ecs_ldtk/issues/243)) ([c530bc9](https://github.com/Trouv/bevy_ecs_ldtk/commit/c530bc975dc055eff3df0f799d12d50c132a9945))
* add level locale types and begin splitting internal_levels and external_levels features ([#237](https://github.com/Trouv/bevy_ecs_ldtk/issues/237)) ([8129e55](https://github.com/Trouv/bevy_ecs_ldtk/commit/8129e5564e52cbe971efe36e0d33fdb5a2b316fa))
* add LevelIid component and spawn it on every level ([#215](https://github.com/Trouv/bevy_ecs_ldtk/issues/215)) ([ad83455](https://github.com/Trouv/bevy_ecs_ldtk/commit/ad834552400ae5b21ff51ae2e4d9f4651e2c82c1))
* add LoadedLevel type that wraps around levels with complete data ([#214](https://github.com/Trouv/bevy_ecs_ldtk/issues/214)) ([3d40c15](https://github.com/Trouv/bevy_ecs_ldtk/commit/3d40c158584f68ea65dbfd36744b07fc5b656163))
* add types and traits around LevelMetadata ([#229](https://github.com/Trouv/bevy_ecs_ldtk/issues/229)) ([382dea2](https://github.com/Trouv/bevy_ecs_ldtk/commit/382dea23407b9ebeffd9eacbc76db6018076cd3a))
* change `LevelEvent` inner types from `String` to `LevelIid` ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219)) ([0039ed7](https://github.com/Trouv/bevy_ecs_ldtk/commit/0039ed757bf6a54c74d912bc43fa4165ada17bbb))
* change `LevelSelection::Iid` inner type from `String` to `LevelIid` ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219)) ([0039ed7](https://github.com/Trouv/bevy_ecs_ldtk/commit/0039ed757bf6a54c74d912bc43fa4165ada17bbb))
* change `LevelSet` inner type from `String` to `LevelIid` ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219)) ([0039ed7](https://github.com/Trouv/bevy_ecs_ldtk/commit/0039ed757bf6a54c74d912bc43fa4165ada17bbb))
* LdtkLevel renamed to LdtkExternalLevel and is no longer used as a component ([#244](https://github.com/Trouv/bevy_ecs_ldtk/issues/244)) ([670cd4e](https://github.com/Trouv/bevy_ecs_ldtk/commit/670cd4e6b704a4748ab41070742733004f1686f9))
* redesign LdtkProject with better level data accessors and correct modeling of internal/external levels ([#244](https://github.com/Trouv/bevy_ecs_ldtk/issues/244)) ([670cd4e](https://github.com/Trouv/bevy_ecs_ldtk/commit/670cd4e6b704a4748ab41070742733004f1686f9))
* replace `LevelSet::from_iid` with `LevelSet::from_iids`, which can convert from any collection of strings. ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219)) ([0039ed7](https://github.com/Trouv/bevy_ecs_ldtk/commit/0039ed757bf6a54c74d912bc43fa4165ada17bbb))
* upgrade to bevy 0.12 ([#265](https://github.com/Trouv/bevy_ecs_ldtk/issues/265)) ([194731e](https://github.com/Trouv/bevy_ecs_ldtk/commit/194731e681727ef8344e7973ade3809ad36d8e8b))
* upgrade to LDtk 1.5.3, dropping support for previous versions ([#295](https://github.com/Trouv/bevy_ecs_ldtk/issues/295)) ([4926a50](https://github.com/Trouv/bevy_ecs_ldtk/commit/4926a50ec0eb37ac3e2ab57a83a5aebcf59d3bf3))
* upgrade to LDtk types and examples to 1.4.1 (drop support for &lt;1.4.1) ([#256](https://github.com/Trouv/bevy_ecs_ldtk/issues/256)) ([ab21e2c](https://github.com/Trouv/bevy_ecs_ldtk/commit/ab21e2c35e0851d06e1881dc8027d30dd891992e))
* use new LevelIid type in LevelEvent, LevelSet, and LevelSelection, plus other improvements ([#219](https://github.com/Trouv/bevy_ecs_ldtk/issues/219)) ([0039ed7](https://github.com/Trouv/bevy_ecs_ldtk/commit/0039ed757bf6a54c74d912bc43fa4165ada17bbb))
* use the bundle's `Default` implementation rather than the field's in `LdtkEntity` and `LdtkIntCell` derive macros ([#222](https://github.com/Trouv/bevy_ecs_ldtk/issues/222)) ([f003127](https://github.com/Trouv/bevy_ecs_ldtk/commit/f003127901c9bb724e8c4f079e54861c1f667ff5))


### Bug Fixes

* don't apply level set until project and dependencies are completely loaded ([#296](https://github.com/Trouv/bevy_ecs_ldtk/issues/296)) ([dbfe1c6](https://github.com/Trouv/bevy_ecs_ldtk/commit/dbfe1c691035f5cc983bf189b44a53cbf6705389))
* normalize resolved asset paths using `path_clean` ([#255](https://github.com/Trouv/bevy_ecs_ldtk/issues/255)) ([33a8998](https://github.com/Trouv/bevy_ecs_ldtk/commit/33a89982545199342875c4f4e11fa53e497686b6)), closes [#240](https://github.com/Trouv/bevy_ecs_ldtk/issues/240)
* only spawn invisible tiles on first sub-layer of AutoTile+IntGrid layers ([#231](https://github.com/Trouv/bevy_ecs_ldtk/issues/231)) ([d2873e3](https://github.com/Trouv/bevy_ecs_ldtk/commit/d2873e35cce8e91a24c3800b84d57d2de0978874))
* recalculate layer offset to adjust for tileset sizes ([#254](https://github.com/Trouv/bevy_ecs_ldtk/issues/254)) ([c00085d](https://github.com/Trouv/bevy_ecs_ldtk/commit/c00085db89c524a6c77f1ee6525d9c6678406631))
* use entity definition tile size instead of entity instance tile size as basis when calculating ldtk entity scale ([#271](https://github.com/Trouv/bevy_ecs_ldtk/issues/271)) ([833af01](https://github.com/Trouv/bevy_ecs_ldtk/commit/833af011adb583ce379c3cd1479adabf2c9dfcce))


### Documentation Changes

* add 0.8 to 0.9 migration guide ([#266](https://github.com/Trouv/bevy_ecs_ldtk/issues/266)) ([bb91660](https://github.com/Trouv/bevy_ecs_ldtk/commit/bb9166036ca0e21d5afbdf0b7df64b014a77f514))
* add collectathon cargo example ([#288](https://github.com/Trouv/bevy_ecs_ldtk/issues/288)) ([32dfb85](https://github.com/Trouv/bevy_ecs_ldtk/commit/32dfb85e095fa16d450d96bab2af622738e0ea63))
* add mdbook with outline and introduction ([#261](https://github.com/Trouv/bevy_ecs_ldtk/issues/261)) ([810b25a](https://github.com/Trouv/bevy_ecs_ldtk/commit/810b25aa7b3782467adcbe25225fc9f33ec2936d))
* add tile-based game example w/ a tutorial in the book, replacing getting-started guide ([#269](https://github.com/Trouv/bevy_ecs_ldtk/issues/269)) ([2d43efa](https://github.com/Trouv/bevy_ecs_ldtk/commit/2d43efa28814cf25e012d7a4e5f9aea17008aaa5))
* document all-features in docs.rs ([#252](https://github.com/Trouv/bevy_ecs_ldtk/issues/252)) ([321bb07](https://github.com/Trouv/bevy_ecs_ldtk/commit/321bb07caeaba5cca1d98e81695eecd0292a9f7a))
* reference book in API ref and README.md, replacing redundant sections ([#282](https://github.com/Trouv/bevy_ecs_ldtk/issues/282)) ([e7afdad](https://github.com/Trouv/bevy_ecs_ldtk/commit/e7afdad79d4526b892fd457a596084ce805369c5))
* remove README.md caveat for hot reloading external levels ([#253](https://github.com/Trouv/bevy_ecs_ldtk/issues/253)) ([59eb6b3](https://github.com/Trouv/bevy_ecs_ldtk/commit/59eb6b3e4404060ce354a754b1392809742ba0e2))
* write *Anatomy of the World* chapter of book ([#285](https://github.com/Trouv/bevy_ecs_ldtk/issues/285)) ([29d5e33](https://github.com/Trouv/bevy_ecs_ldtk/commit/29d5e33e95c692f35b0413adafd0ce20d830bdc1))
* write *Create bevy relations from ldtk entity references* chapter of book ([#287](https://github.com/Trouv/bevy_ecs_ldtk/issues/287)) ([8080f24](https://github.com/Trouv/bevy_ecs_ldtk/commit/8080f24b401df200dccf4c7840905b36b84f10b8))
* write *Game Logic Integration* chapter of the book ([#279](https://github.com/Trouv/bevy_ecs_ldtk/issues/279)) ([a62a556](https://github.com/Trouv/bevy_ecs_ldtk/commit/a62a556c2f84d7eafe3ab541725347879b34ecdc))
* write *Level Selection* chapter of book ([#284](https://github.com/Trouv/bevy_ecs_ldtk/issues/284)) ([226c60c](https://github.com/Trouv/bevy_ecs_ldtk/commit/226c60c1e7e27fb32ea6cc9de6f68432b867f537))
* write *Make level selection follow player* chapter of book ([#293](https://github.com/Trouv/bevy_ecs_ldtk/issues/293)) ([201d908](https://github.com/Trouv/bevy_ecs_ldtk/commit/201d908ae3e4f3deeb40de228f234c414c6b3141))
* write *Respawn levels and worlds* chapter of book ([#289](https://github.com/Trouv/bevy_ecs_ldtk/issues/289)) ([55ed30f](https://github.com/Trouv/bevy_ecs_ldtk/commit/55ed30f203a1cffeccc562f54ae797e23b299c89))


### Code Refactors

* `LdtkAsset` and `LdtkProject` are now exported in new `assets` module instead of `lib.rs` ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206)) ([fe44774](https://github.com/Trouv/bevy_ecs_ldtk/commit/fe44774c69cc639ecdb710af593a748744a1810d))
* `LdtkAsset` renamed to `LdtkProject` ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206)) ([fe44774](https://github.com/Trouv/bevy_ecs_ldtk/commit/fe44774c69cc639ecdb710af593a748744a1810d))
* `LdtkProject::project` and `LdtkLevel::level` fields have both been renamed to `data` ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206)) ([fe44774](https://github.com/Trouv/bevy_ecs_ldtk/commit/fe44774c69cc639ecdb710af593a748744a1810d))
* `LevelMap` and `TilesetMap` type aliases have been removed ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206)) ([fe44774](https://github.com/Trouv/bevy_ecs_ldtk/commit/fe44774c69cc639ecdb710af593a748744a1810d))
* All fields of `LdtkProject` and `LdtkLevel` are now privatized, and have immutable getter methods ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206)) ([fe44774](https://github.com/Trouv/bevy_ecs_ldtk/commit/fe44774c69cc639ecdb710af593a748744a1810d))
* asset `Loader` types are now private ([#206](https://github.com/Trouv/bevy_ecs_ldtk/issues/206)) ([fe44774](https://github.com/Trouv/bevy_ecs_ldtk/commit/fe44774c69cc639ecdb710af593a748744a1810d))

## [0.8.0](https://github.com/Trouv/bevy_ecs_ldtk/compare/v0.7.0...v0.8.0) (2023-07-31)


### ⚠ BREAKING CHANGES

* upgrade to bevy and bevy_ecs_tilemap 0.11 ([#204](https://github.com/Trouv/bevy_ecs_ldtk/issues/204))
* `LdtkAsset::world_height` has been removed
* upgrade LDtk types to 1.3.3 (dropping support for <1.3.3 LDtk projects) ([#203](https://github.com/Trouv/bevy_ecs_ldtk/issues/203))

### Features

* add `EntityIid` component which is added to all entities by default ([#194](https://github.com/Trouv/bevy_ecs_ldtk/issues/194)) ([d99f1ae](https://github.com/Trouv/bevy_ecs_ldtk/commit/d99f1ae7eec28114d9277e5c5063418234fcc261))
* register and derive Reflect for LdtkLevel and dependent types ([#201](https://github.com/Trouv/bevy_ecs_ldtk/issues/201)) ([873ed17](https://github.com/Trouv/bevy_ecs_ldtk/commit/873ed179b8fe80b95100f1aabaf63754ad285f74))
* upgrade LDtk types to 1.3.3 (dropping support for &lt;1.3.3 LDtk projects) ([#203](https://github.com/Trouv/bevy_ecs_ldtk/issues/203)) ([e347780](https://github.com/Trouv/bevy_ecs_ldtk/commit/e3477804906f3a8f1ff5afc209734fe4891fc439))
* upgrade to bevy and bevy_ecs_tilemap 0.11 ([#204](https://github.com/Trouv/bevy_ecs_ldtk/issues/204)) ([ef1b075](https://github.com/Trouv/bevy_ecs_ldtk/commit/ef1b075a12b4793575a1f310421f5062a403494e))


### Bug Fixes

* remove `LdtkAsset::world_height` and correct `UseWorldTranslation` y-calculation ([#207](https://github.com/Trouv/bevy_ecs_ldtk/issues/207)) ([8923b4e](https://github.com/Trouv/bevy_ecs_ldtk/commit/8923b4e01b1d78c7516299fbf052a09bc37ea657))


### Documentation Changes

* fix code links in entity_iid module ([#210](https://github.com/Trouv/bevy_ecs_ldtk/issues/210)) ([53728b3](https://github.com/Trouv/bevy_ecs_ldtk/commit/53728b3dd0a969dfad08cfccecc1e5f4e1fee03d))

## [0.7.0](https://github.com/Trouv/bevy_ecs_ldtk/compare/v0.6.0...v0.7.0) (2023-04-29)


### ⚠ BREAKING CHANGES

* Most likely won't affect users - `LdtkAsset` has gained a `int_grid_image_handle` field, breaking any manual construction of it.

### Features

* add LdtkFields trait with convenience methods for accessing field instances ([#180](https://github.com/Trouv/bevy_ecs_ldtk/issues/180)) ([a8dba24](https://github.com/Trouv/bevy_ecs_ldtk/commit/a8dba247ffee79c1eae2a2669c7ca0c6e5d17dd9))


### Bug Fixes

* create IntGrid white-image on asset load and minimize its size ([#183](https://github.com/Trouv/bevy_ecs_ldtk/issues/183)) ([23fd924](https://github.com/Trouv/bevy_ecs_ldtk/commit/23fd9244484505c50e2f6232aaf7a7d6355e0452))
* insert Name component before evaluating `LdtkEntity` ([#186](https://github.com/Trouv/bevy_ecs_ldtk/issues/186)) ([a5c1579](https://github.com/Trouv/bevy_ecs_ldtk/commit/a5c157936bb25bf1a13b4796ce2eea880cfa6687))


### Documentation Changes

* rewrite field_instances example, demonstrating LdtkFields API ([#187](https://github.com/Trouv/bevy_ecs_ldtk/issues/187)) ([7be6635](https://github.com/Trouv/bevy_ecs_ldtk/commit/7be663592ff7a173491f1f8d1679445c882d8752))

## [0.6.0](https://github.com/Trouv/bevy_ecs_ldtk/compare/v0.5.0...v0.6.0) (2023-03-31)


### ⚠ BREAKING CHANGES

* In addition to updating to bevy 0.10, users may need define order between `LdtkSystemSet::ProcessApi` and other 3rd party system sets, like [rapier](https://github.com/Trouv/bevy_ecs_ldtk/blob/5b8f17cc51f91ff9aedbed8afca560e750b557c8/examples/platformer/main.rs#L17).
* change LdtkEntity's #[with] attribute to borrow EntityInstance ([#158](https://github.com/Trouv/bevy_ecs_ldtk/issues/158))
* split `RegisterLdtkObjects` into two new traits with a different naming convention ([#155](https://github.com/Trouv/bevy_ecs_ldtk/issues/155))
* change #[from_entity_instance] to use references ([#149](https://github.com/Trouv/bevy_ecs_ldtk/issues/149))

### Features

* add `#[sprite_sheet_bundle(no_grid)]` attribute for generating a single-texture `TextureAtlas` instead of a grid ([#161](https://github.com/Trouv/bevy_ecs_ldtk/issues/161)) ([d6d3c9c](https://github.com/Trouv/bevy_ecs_ldtk/commit/d6d3c9c31d4a89179c6f5a867f6e35e25438ea6a))
* add `with` attribute for LdtkIntCell derive macro ([#157](https://github.com/Trouv/bevy_ecs_ldtk/issues/157)) ([d3fbd3c](https://github.com/Trouv/bevy_ecs_ldtk/commit/d3fbd3c76e4425a11b6255b2e1a2334dcd36e847))
* add LevelSet::from_iid method ([#144](https://github.com/Trouv/bevy_ecs_ldtk/issues/144)) ([fb17ae1](https://github.com/Trouv/bevy_ecs_ldtk/commit/fb17ae1a2a329c249f01d4728fc585c5550a98c5))
* add render feature for headless mode (tilemaps only) ([#159](https://github.com/Trouv/bevy_ecs_ldtk/issues/159)) ([2f8000e](https://github.com/Trouv/bevy_ecs_ldtk/commit/2f8000e4a8566e7bb2a1bf579ca21487fb44153f))
* change #[from_entity_instance] to use references ([#149](https://github.com/Trouv/bevy_ecs_ldtk/issues/149)) ([246880f](https://github.com/Trouv/bevy_ecs_ldtk/commit/246880f64deeca22e5ab1b733d5afc72f571fc7e))
* change LdtkEntity's #[with] attribute to borrow EntityInstance ([#158](https://github.com/Trouv/bevy_ecs_ldtk/issues/158)) ([c052b31](https://github.com/Trouv/bevy_ecs_ldtk/commit/c052b313979f45a698ffeece4803dca74f638784))
* register TileMetadata and TileEnumTags types ([#153](https://github.com/Trouv/bevy_ecs_ldtk/issues/153)) ([26cae15](https://github.com/Trouv/bevy_ecs_ldtk/commit/26cae1597801ca1f13bece97760fe6172e3dbb42))
* register types `GridCoords` and `LayerMetadata` ([#146](https://github.com/Trouv/bevy_ecs_ldtk/issues/146)) ([ed4a0f9](https://github.com/Trouv/bevy_ecs_ldtk/commit/ed4a0f9ae89ed4f709343d097e6652ec905284e5))
* upgrade to bevy 0.10 ([#168](https://github.com/Trouv/bevy_ecs_ldtk/issues/168)) ([5b8f17c](https://github.com/Trouv/bevy_ecs_ldtk/commit/5b8f17cc51f91ff9aedbed8afca560e750b557c8))


### Bug Fixes

* use normal sprite for background color instead of tile ([#171](https://github.com/Trouv/bevy_ecs_ldtk/issues/171)) ([b22b11d](https://github.com/Trouv/bevy_ecs_ldtk/commit/b22b11dee6c1a7d74fef3912ca1f0154bc0bc6a2))


### Example Changes

* improve ground detection in platformer example ([#137](https://github.com/Trouv/bevy_ecs_ldtk/issues/137)) ([cafba57](https://github.com/Trouv/bevy_ecs_ldtk/commit/cafba57e0e0fcf35927497693efcc38985658374))
* use rect_builder buffer instead of row-specific current_rects in spawn_wall_collisions ([#147](https://github.com/Trouv/bevy_ecs_ldtk/issues/147)) ([45303f3](https://github.com/Trouv/bevy_ecs_ldtk/commit/45303f368e684e9b9898a1238fd9e3b19064538e))


### Code Refactors

* split `RegisterLdtkObjects` into two new traits with a different naming convention ([#155](https://github.com/Trouv/bevy_ecs_ldtk/issues/155)) ([156ae8c](https://github.com/Trouv/bevy_ecs_ldtk/commit/156ae8cb7c512a8458297d166891b7e2a1ec932f))


### Documentation Changes

* explain feature flags in crate-level documentation ([#164](https://github.com/Trouv/bevy_ecs_ldtk/issues/164)) ([a832da0](https://github.com/Trouv/bevy_ecs_ldtk/commit/a832da00a97be592d917e4e44c5ab1781d7b34ca))
* explain that sprite_bundle should not be used with tilemap editor visuals ([#142](https://github.com/Trouv/bevy_ecs_ldtk/issues/142)) ([1a7a8a1](https://github.com/Trouv/bevy_ecs_ldtk/commit/1a7a8a177f20b717fbaa08832a1c47d07527f67e))
* repair doc links to bevy in app module ([#154](https://github.com/Trouv/bevy_ecs_ldtk/issues/154)) ([0f928e8](https://github.com/Trouv/bevy_ecs_ldtk/commit/0f928e89b97102b14a2ae4b2191e47e2a716ece9))

## [0.5.0](https://github.com/Trouv/bevy_ecs_ldtk/compare/v0.4.0...v0.5.0) (2022-11-19)


### ⚠ BREAKING CHANGES

* upgrade to bevy 0.9 (#138)
* adjust tile transformations for bevy_ecs_tilemap 0.8 (#136)
* upgrade `bevy_ecs_tilemap` dependency to 0.8 (#134)

### Features

* add with attribute to LdtkEntity derive ([#128](https://github.com/Trouv/bevy_ecs_ldtk/issues/128)) ([18e84be](https://github.com/Trouv/bevy_ecs_ldtk/commit/18e84be31a134bae77f3cd1334a5e3b93ca21bc4))
* insert Name component on ldtk entities, layers, and levels ([33f2b73](https://github.com/Trouv/bevy_ecs_ldtk/commit/33f2b737bd6b7b767dda1ff1a3303adb0eb27ef0))
* upgrade `bevy_ecs_tilemap` dependency to 0.8 ([#134](https://github.com/Trouv/bevy_ecs_ldtk/issues/134)) ([7d4d1d0](https://github.com/Trouv/bevy_ecs_ldtk/commit/7d4d1d0b82692ef60987784019132c31a6f08cf5))
* upgrade to bevy 0.9 ([#138](https://github.com/Trouv/bevy_ecs_ldtk/issues/138)) ([048285c](https://github.com/Trouv/bevy_ecs_ldtk/commit/048285cff1024b5f319bfb276511f534629b80b3))


### Bug Fixes

* adjust tile transformations for bevy_ecs_tilemap 0.8 ([#136](https://github.com/Trouv/bevy_ecs_ldtk/issues/136)) ([aad0325](https://github.com/Trouv/bevy_ecs_ldtk/commit/aad03258f6ba4000676831eed765f792deb0126d))
* do not spawn empty ECS entity for omitted worldly entities ([#122](https://github.com/Trouv/bevy_ecs_ldtk/issues/122)) ([a9a0318](https://github.com/Trouv/bevy_ecs_ldtk/commit/a9a0318924448613a59203a85669555ef672e266))
* filter out out-of-bounds tiles ([#129](https://github.com/Trouv/bevy_ecs_ldtk/issues/129)) ([37dfed0](https://github.com/Trouv/bevy_ecs_ldtk/commit/37dfed084f57f35516f636ba5ed0b94042eac63b))
