//! Contains [`LoadedLevel`] and related types/implementaions.
use crate::ldtk::{
    ldtk_fields::LdtkFields, BgPos, FieldInstance, LayerInstance, Level, LevelBackgroundPosition,
    NeighbourLevel,
};
use bevy::prelude::Color;
use thiserror::Error;

/// Error that can occur when trying to coerce a [`Level`] into a [`LoadedLevel`].
#[derive(Debug, Error)]
#[error("loaded levels must have non-null layer instances")]
pub struct LevelNotLoaded;

/// Wrapper around a borrowed [`Level`] that guarantees the level has complete data.
///
/// In the LDtk json format, a level might not have complete data if external levels are enabled.
/// In particular, the main project file will have levels whose `layer_instances` field is null.
/// The complete data for these levels will exist in a separate file.
///
/// Can be constructed via [`LoadedLevel::try_from`], or accessed via the [asset types].
/// The construction verifies that the `layer_instances` are not null.
/// As a result, the [`LoadedLevel::layer_instances`] accessor expects the `Option` away.
///
/// [asset types]: crate::assets::LdtkProject#accessing-internal-and-external-loaded-levels
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct LoadedLevel<'a> {
    level: &'a Level,
}

impl<'a> TryFrom<&'a Level> for LoadedLevel<'a> {
    type Error = LevelNotLoaded;

    fn try_from(level: &'a Level) -> Result<Self, Self::Error> {
        if level.layer_instances.is_some() {
            Ok(LoadedLevel { level })
        } else {
            Err(LevelNotLoaded)
        }
    }
}

impl<'a> LoadedLevel<'a> {
    /// The raw level data borrowed by this instance.
    pub fn raw(&self) -> &Level {
        self.level
    }

    /// Background color of the level (same as `bgColor`, except the default value is
    /// automatically used here if its value is `null`)
    pub fn bg_color(&self) -> &Color {
        &self.level.bg_color
    }

    /// Position informations of the background image, if there is one.
    pub fn bg_pos(&self) -> &Option<LevelBackgroundPosition> {
        &self.level.bg_pos
    }

    /// An array listing all other levels touching this one on the world map.<br/>  Only relevant
    /// for world layouts where level spatial positioning is manual (ie. GridVania, Free). For
    /// Horizontal and Vertical layouts, this array is always empty.
    pub fn neighbours(&self) -> &Vec<NeighbourLevel> {
        &self.level.neighbours
    }

    /// The "guessed" color for this level in the editor, decided using either the background
    /// color or an existing custom field.
    pub fn smart_color(&self) -> &Color {
        &self.level.smart_color
    }

    /// Background color of the level. If `null`, the project `defaultLevelBgColor` should be
    /// used.
    pub fn level_bg_color(&self) -> &Option<Color> {
        &self.level.level_bg_color
    }

    /// Background image X pivot (0-1)
    pub fn bg_pivot_x(&self) -> &f32 {
        &self.level.bg_pivot_x
    }

    /// Background image Y pivot (0-1)
    pub fn bg_pivot_y(&self) -> &f32 {
        &self.level.bg_pivot_y
    }

    /// An enum defining the way the background image (if any) is positioned on the level. See
    /// `__bgPos` for resulting position info. Possible values: &lt;`null`&gt;, `Unscaled`,
    /// `Contain`, `Cover`, `CoverDirty`, `Repeat`
    pub fn level_bg_pos(&self) -> &Option<BgPos> {
        &self.level.level_bg_pos
    }

    /// The *optional* relative path to the level background image.
    pub fn bg_rel_path(&self) -> &Option<String> {
        &self.level.bg_rel_path
    }

    /// This value is not null if the project option "*Save levels separately*" is enabled. In
    /// this case, this **relative** path points to the level Json file.
    pub fn external_rel_path(&self) -> &Option<String> {
        &self.level.external_rel_path
    }

    /// An array containing this level custom field values.
    pub fn field_instances(&self) -> &Vec<FieldInstance> {
        &self.level.field_instances
    }

    /// User defined unique identifier
    pub fn identifier(&self) -> &String {
        &self.level.identifier
    }

    /// Unique instance identifier
    pub fn iid(&self) -> &String {
        &self.level.iid
    }

    /// An array containing all Layer instances.
    pub fn layer_instances(&self) -> &Vec<LayerInstance> {
        self.level
            .layer_instances
            .as_ref()
            .expect("LoadedLevel construction should guarantee the existence of layer instances")
    }

    /// Height of the level in pixels
    pub fn px_hei(&self) -> &i32 {
        &self.level.px_hei
    }

    /// Width of the level in pixels
    pub fn px_wid(&self) -> &i32 {
        &self.level.px_wid
    }

    /// Unique Int identifier
    pub fn uid(&self) -> &i32 {
        &self.level.uid
    }

    /// If TRUE, the level identifier will always automatically use the naming pattern as defined
    /// in `Project.levelNamePattern`. Becomes FALSE if the identifier is manually modified by
    /// user.
    pub fn use_auto_identifier(&self) -> &bool {
        &self.level.use_auto_identifier
    }

    /// Index that represents the "depth" of the level in the world. Default is 0, greater means
    /// "above", lower means "below".<br/>  This value is mostly used for display only and is
    /// intended to make stacking of levels easier to manage.
    pub fn world_depth(&self) -> &i32 {
        &self.level.world_depth
    }

    /// World X coordinate in pixels.<br/>  Only relevant for world layouts where level spatial
    /// positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the
    /// value is always -1 here.
    pub fn world_x(&self) -> &i32 {
        &self.level.world_x
    }

    /// World Y coordinate in pixels.<br/>  Only relevant for world layouts where level spatial
    /// positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the
    /// value is always -1 here.
    pub fn world_y(&self) -> &i32 {
        &self.level.world_y
    }
}

impl<'a> LdtkFields for LoadedLevel<'a> {
    fn field_instances(&self) -> &[FieldInstance] {
        self.level.field_instances()
    }
}

#[cfg(test)]
mod tests {
    use bevy::color::palettes::css;

    use super::*;

    fn valid_level() -> Level {
        Level {
            bg_color: css::AZURE.into(),
            bg_pos: Some(LevelBackgroundPosition::default()),
            neighbours: vec![NeighbourLevel::default()],
            smart_color: css::BEIGE.into(),
            level_bg_color: Some(css::AQUA.into()),
            bg_pivot_x: 0.,
            bg_pivot_y: 1.,
            level_bg_pos: Some(BgPos::Cover),
            bg_rel_path: Some("path/to/bg.png".to_string()),
            external_rel_path: Some("path/to/external.ldtkl".to_string()),
            field_instances: vec![],
            identifier: "level_identifier".to_string(),
            iid: "level_iid".to_string(),
            layer_instances: Some(vec![LayerInstance::default()]),
            px_hei: 2,
            px_wid: 3,
            uid: 4,
            use_auto_identifier: true,
            world_depth: 5,
            world_x: 6,
            world_y: 7,
        }
    }

    #[test]
    fn getter_methods_return_correct_values() {
        // This test mostly exists to easily warn us if the `Level` type has changed.
        let raw = valid_level();

        let loaded = LoadedLevel::try_from(&raw).unwrap();

        // layer instances isn't optional
        assert_eq!(*loaded.layer_instances(), vec![LayerInstance::default()]);

        // all other getters
        assert_eq!(*loaded.bg_color(), css::AZURE.into());
        assert_eq!(*loaded.bg_pos(), Some(LevelBackgroundPosition::default()));
        assert_eq!(*loaded.neighbours(), vec![NeighbourLevel::default()]);
        assert_eq!(*loaded.smart_color(), css::BEIGE.into());
        assert_eq!(*loaded.level_bg_color(), Some(css::AQUA.into()));
        assert_eq!(*loaded.bg_pivot_x(), 0.);
        assert_eq!(*loaded.bg_pivot_y(), 1.);
        assert_eq!(*loaded.level_bg_pos(), Some(BgPos::Cover));
        assert_eq!(*loaded.bg_rel_path(), Some("path/to/bg.png".to_string()));
        assert_eq!(
            *loaded.external_rel_path(),
            Some("path/to/external.ldtkl".to_string())
        );
        assert_eq!(*loaded.field_instances(), vec![]);
        assert_eq!(*loaded.identifier(), "level_identifier".to_string());
        assert_eq!(*loaded.iid(), "level_iid".to_string());
        assert_eq!(*loaded.px_hei(), 2);
        assert_eq!(*loaded.px_wid(), 3);
        assert_eq!(*loaded.uid(), 4);
        assert!(*loaded.use_auto_identifier());
        assert_eq!(*loaded.world_depth(), 5);
        assert_eq!(*loaded.world_x(), 6);
        assert_eq!(*loaded.world_y(), 7);
    }

    #[test]
    fn cannot_create_from_unloaded_level() {
        let mut raw = valid_level();

        raw.layer_instances = None;

        assert!(matches!(LoadedLevel::try_from(&raw), Err(LevelNotLoaded)));
    }
}
