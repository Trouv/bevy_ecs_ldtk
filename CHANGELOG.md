# Changelog

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
