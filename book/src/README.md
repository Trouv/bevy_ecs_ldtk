# Introduction

## `bevy_ecs_ldtk`

{{ #include blurb.md }}

## This book
This book is a work in progress, but aims to provide the following pieces of documentation:
- tutorials: lessons detailing the creation of simple games from start to finish
- explanation: clarification of concepts and strategies employed by `bevy_ecs_ldtk`, including details about how it works and why
- how-to guides: recommended solutions to common problems, as well as migration guides

This book is not an API reference.
For that, please refer to `bevy_ecs_ldtk`'s documentation on [docs.rs](https://docs.rs/bevy_ecs_ldtk/).

While this book aims to be comprehensive, it should also be easy to maintain and up-to-date.
This is why, in consort with the API reference, documentation for `bevy_ecs_ldtk` aims to satisfy [The Grand Unified Theory of Documentation](https://documentation.divio.com/).
Furthermore, code snippets in this book are automatically tested by `bevy_ecs_ldtk`'s CI wherever possible with the help of [mdBook-Keeper](https://github.com/tfpk/mdbook-keeper/).
This should help inform maintainers when changes to the plugin have made documentation out-of-date.
Deployment of this book to github pages is also performed by `bevy_ecs_ldtk`'s CI automatically on new releases.

Splitting the documentation up this way means that this book is not necessarily meant to be read in order.
Some chapters are intended to be read while working on your own project, while others are meant to be more like studying material.
The following chapters are good jumping-off points for beginners:
- [*Tile-based Game* tutorial](tutorials/tile-based-game/index.html)
- [*Level Selection* explanation](explanation/level-selection.md)
- [*Game Logic Integration* explanation](explanation/game-logic-integration.md)

## Other resources
This book is not suitable documentation for bevy or LDtk.
Some resources for learning Bevy include those listed on the [Bevy website](https://bevyengine.org/learn), as well as the unofficial [Bevy Cheat Book](https://bevy-cheatbook.github.io/).
LDtk also provides documentation on [its website](https://ldtk.io/docs/).

`bevy_ecs_ldtk`'s [source code](https://github.com/Trouv/bevy_ecs_ldtk) is available on github.
This repository also contains [cargo examples](https://github.com/Trouv/bevy_ecs_ldtk/tree/v0.9.0/examples), which can be run after cloning the repository using `$ cargo run --example example-name`. <!-- x-release-please-version -->
These examples may be difficult to follow on their own, and many of their strategies are described in this book.
When viewing these examples, be careful to checkout the correct git tag for the version of the plugin you are using.
Some changes may have been made to the plugin or to the examples on the `main` branch that are not released yet, and trying to apply these to the version of the plugin you are using can lead to errors.

## License
The pages of this book fall under the same license as the rest of the `bevy_ecs_ldtk` repository.
I.e., this book is dual-licensed under [MIT](http://opensource.org/licenses/MIT) and [Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0) at your option.
The plain text of this license is available in the `bevy_ecs_ldtk` repository's [LICENSE file](https://github.com/Trouv/bevy_ecs_ldtk/blob/main/LICENSE).
