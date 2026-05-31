# Limitations

## Spacing and Padding

Due to a difference in the handling of spacing and padding between `bevy_ecs_tilemap` and LDtk spacing is not perfectly supported. This can be resolved by having the value of padding and spacing be equal, for example both 0, or both 1 and so on. Previous versions of `bevy_ecs_tilemap` require the `atlas` feature flag enabled for WASM support and also for tile spacing to work with Tile and AutoTile layers.
