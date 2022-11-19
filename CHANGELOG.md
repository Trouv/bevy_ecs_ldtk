# Changelog

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
