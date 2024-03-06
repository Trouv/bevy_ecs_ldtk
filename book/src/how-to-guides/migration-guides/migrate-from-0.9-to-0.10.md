# Migrate from 0.9 to 0.10

## Entity translations now respect pivot
Entities now spawn with a translation matching their actual location, rather than their "visual center".

![Diagram showing an entity with a dot representing its location in three cases: in the LDtk editor; loaded in version 0.9; and loaded in 0.10. In LDtk, the dot is at the upper-left corner. Loaded in 0.10, the dot is at the same place, but in 0.9, the dot is instead at the entity's center.](images/pivot-0-9-to-0-10.png)

For `LdtkEntity` bundles with a `#[sprite_sheet_bundle(...)]`, the macro calculates the sprite's `Anchor` from the pivot, so they should appear the same,
but gameplay logic will need to be rewritten to account for the differences,
as will systems that add sprite bundles manually. In the latter case,
`utils::ldtk_pivot_to_anchor` can be used to find the correct `Anchor`.

If the entity's center point is still wanted, it can be found using `utils::ldtk_entity_visual_center`:
```rust,ignore
//0.9
fn shoot_laser_from_center (
	mut query: Query<(&mut Transform, &FiringLaser), With<EntityInstance>>
) {
	for (mut transform, laser) in &mut query {
		let center = transform.translation();

		//do laser shooting stuff
	}
}
```
```rust,ignore
//0.10
fn shoot_laser_from_center (
	mut query: Query<(&EntityInstance, &mut Transform, &FiringLaser)>
) {
	for (instance, mut transform, laser) in &mut query {
		let object_center = bevy_ecs_ldtk::utils::ldtk_entity_visual_center(
			transform.translation(),
			IVec2::new(entity.width, entity.height),
			entity.pivot
		);

		//do laser shooting stuff
	}
}
```