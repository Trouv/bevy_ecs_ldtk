# Game Logic Integration
Loading LDtk levels into Bevy doesn't get you very far if you cannot play them.

Aside from rendering tilemaps, LDtk has features for placing gameplay objects on Entity layers.
Even within tilemaps, IntGrid layers imply a categorization of tiles, and perhaps a game designerly meaning.
It is fundamental to associate the LDtk entities and IntGrid tiles with Bevy entities/components.
`bevy_ecs_ldtk` is designed around a couple core strategies for doing so, which will be discussed here.

## `LdtkEntity` and `LdtkIntCell` registration

## Post-processing plugin-added entities

## A combined approach - the blueprint pattern

