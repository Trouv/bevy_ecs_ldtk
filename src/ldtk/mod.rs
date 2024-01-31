//! Contains all the types for serializing/deserializing an LDtk file.
//!
//! This is mostly based on LDtk's existing rust
//! [QuickType loader](<https://ldtk.io/files/quicktype/LdtkJson.rs>).
//!
//! For the most part, changes to the generated module are avoided to make it simpler to maintain
//! this plugin in the future.
//! However, some usability concerns have been addressed.
//! Any changes should be documented here for maintenance purposes:
//! 1. Comment at the top of the file has been replaced with this documentation.
//! 2. All enums have [Default] derived. The first variant is selected as default, exception for
//!    [LimitBehavior], [LimitScope], [RenderMode], [TileRenderMode], and [Type].
//! 3. [Copy], [Default], [Eq], [PartialEq], and [Reflect] have been derived wherever possible.
//! 4. [i64] and [f64] have been changed to [i32] and [f32].
//! 5. `Component` has been derived for [EntityInstance].
//! 6. Documentation added for [EntityInstance], which required the unused import of [LdtkEntity].
//! 7. [FieldInstance] has been moved to its own module, and is re-exported here.
//! 8. Some "coordinate" fields on [LevelBackgroundPosition], [EntityInstance], and [TileInstance]
//!    have been changed from vectors to [IVec2] and [Vec2].
//! 9. Some "color" fields on [LdtkJson], [EntityDefinition], [IntGridValueDefinition],
//!    [LayerDefinition], [Level], and [EntityInstance] have been changed from [String]s to
//!    [Color]. They also are given a serde attribute for using the "color" or "color::optional"
//!    modules.
//! 10. All urls in docs have been changed to hyperlinks with `<>`
//! 11. `From<&EntityInstance>` implemented for [`EntityInstance`]
//! 12. [`LayerInstance::layer_instance_type`] changed from [`String`] to [`Type`].

use bevy::{
    prelude::{Color, Component, IVec2, Vec2},
    reflect::Reflect,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[allow(unused_imports)]
use crate::prelude::LdtkEntity;

pub mod all_some_iter;
mod color;
#[cfg(test)]
pub mod fake;
mod field_instance;
mod impl_definitions;
pub mod ldtk_fields;
pub mod loaded_level;
pub mod raw_level_accessor;

pub use field_instance::*;

/// This file is a JSON schema of files created by LDtk level editor (<https://ldtk.io>).
///
/// This is the root of any Project JSON file. It contains:  - the project settings, - an
/// array of levels, - a group of definitions (that can probably be safely ignored for most
/// users).
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct LdtkJson {
    /// This object is not actually used by LDtk. It ONLY exists to force explicit references to
    /// all types, to make sure QuickType finds them and integrate all of them. Otherwise,
    /// Quicktype will drop types that are not explicitely used.
    #[serde(rename = "__FORCED_REFS")]
    pub forced_refs: Option<ForcedRefs>,

    /// LDtk application build identifier.<br/>  This is only used to identify the LDtk version
    /// that generated this particular project file, which can be useful for specific bug fixing.
    /// Note that the build identifier is just the date of the release, so it's not unique to
    /// each user (one single global ID per LDtk public release), and as a result, completely
    /// anonymous.
    pub app_build_id: f32,

    /// Number of backup files to keep, if the `backupOnSave` is TRUE
    pub backup_limit: i32,

    /// If TRUE, an extra copy of the project will be created in a sub folder, when saving.
    pub backup_on_save: bool,

    /// Target relative path to store backup files
    pub backup_rel_path: Option<String>,

    /// Project background color
    #[serde(with = "color")]
    pub bg_color: Color,

    /// An array of command lines that can be ran manually by the user
    pub custom_commands: Vec<LdtkCustomCommand>,

    /// Default height for new entities
    pub default_entity_height: i32,

    /// Default width for new entities
    pub default_entity_width: i32,

    /// Default grid size for new layers
    pub default_grid_size: i32,

    /// Default background color of levels
    #[serde(with = "color")]
    pub default_level_bg_color: Color,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Default new level height
    pub default_level_height: Option<i32>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Default new level width
    pub default_level_width: Option<i32>,

    /// Default X pivot (0 to 1) for new entities
    pub default_pivot_x: f32,

    /// Default Y pivot (0 to 1) for new entities
    pub default_pivot_y: f32,

    /// A structure containing all the definitions of this project
    pub defs: Definitions,

    /// If the project isn't in MultiWorlds mode, this is the IID of the internal "dummy" World.
    pub dummy_world_iid: String,

    /// If TRUE, the exported PNGs will include the level background (color or image).
    pub export_level_bg: bool,

    /// **WARNING**: this deprecated value is no longer exported since version 0.9.3  Replaced
    /// by: `imageExportMode`
    pub export_png: Option<bool>,

    /// If TRUE, a Tiled compatible file will also be generated along with the LDtk JSON file
    /// (default is FALSE)
    pub export_tiled: bool,

    /// If TRUE, one file will be saved for the project (incl. all its definitions) and one file
    /// in a sub-folder for each level.
    pub external_levels: bool,

    /// An array containing various advanced flags (ie. options or other states). Possible
    /// values: `DiscardPreCsvIntGrid`, `ExportOldTableOfContentData`,
    /// `ExportPreCsvIntGridFormat`, `IgnoreBackupSuggest`, `PrependIndexToLevelFileNames`,
    /// `MultiWorlds`, `UseMultilinesType`
    pub flags: Vec<Flag>,

    /// Naming convention for Identifiers (first-letter uppercase, full uppercase etc.) Possible
    /// values: `Capitalize`, `Uppercase`, `Lowercase`, `Free`
    pub identifier_style: IdentifierStyle,

    /// Unique project identifier
    pub iid: String,

    /// "Image export" option when saving project. Possible values: `None`, `OneImagePerLayer`,
    /// `OneImagePerLevel`, `LayersAndLevels`
    pub image_export_mode: ImageExportMode,

    /// File format version
    pub json_version: String,

    /// The default naming convention for level identifiers.
    pub level_name_pattern: String,

    /// All levels. The order of this array is only relevant in `LinearHorizontal` and
    /// `linearVertical` world layouts (see `worldLayout` value).<br/>  Otherwise, you should
    /// refer to the `worldX`,`worldY` coordinates of each Level.
    pub levels: Vec<Level>,

    /// If TRUE, the Json is partially minified (no indentation, nor line breaks, default is
    /// FALSE)
    pub minify_json: bool,

    /// Next Unique integer ID available
    pub next_uid: i32,

    /// File naming pattern for exported PNGs
    pub png_file_pattern: Option<String>,

    /// If TRUE, a very simplified will be generated on saving, for quicker & easier engine
    /// integration.
    pub simplified_export: bool,

    /// All instances of entities that have their `exportToToc` flag enabled are listed in this
    /// array.
    pub toc: Vec<LdtkTableOfContentEntry>,

    /// This optional description is used by LDtk Samples to show up some informations and
    /// instructions.
    pub tutorial_desc: Option<String>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Height of the world grid in pixels.
    pub world_grid_height: Option<i32>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  Width of the world grid in pixels.
    pub world_grid_width: Option<i32>,

    /// **WARNING**: this field will move to the `worlds` array after the "multi-worlds" update.
    /// It will then be `null`. You can enable the Multi-worlds advanced project option to enable
    /// the change immediately.<br/><br/>  An enum that describes how levels are organized in
    /// this project (ie. linearly or in a 2D space). Possible values: &lt;`null`&gt;, `Free`,
    /// `GridVania`, `LinearHorizontal`, `LinearVertical`
    pub world_layout: Option<WorldLayout>,

    /// This array will be empty, unless you enable the Multi-Worlds in the project advanced
    /// settings.<br/><br/> - in current version, a LDtk project file can only contain a single
    /// world with multiple levels in it. In this case, levels and world layout related settings
    /// are stored in the root of the JSON.<br/> - with "Multi-worlds" enabled, there will be a
    /// `worlds` array in root, each world containing levels and layout settings. Basically, it's
    /// pretty much only about moving the `levels` array to the `worlds` array, along with world
    /// layout related values (eg. `worldGridWidth` etc).<br/><br/>If you want to start
    /// supporting this future update easily, please refer to this documentation:
    /// <https://github.com/deepnight/ldtk/issues/231>
    pub worlds: Vec<World>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
pub struct LdtkCustomCommand {
    pub command: String,

    /// Possible values: `Manual`, `AfterLoad`, `BeforeSave`, `AfterSave`
    pub when: When,
}

/// Possible values: `Manual`, `AfterLoad`, `BeforeSave`, `AfterSave`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum When {
    #[default]
    #[serde(rename = "AfterLoad")]
    AfterLoad,

    #[serde(rename = "AfterSave")]
    AfterSave,

    #[serde(rename = "BeforeSave")]
    BeforeSave,

    Manual,
}

/// If you're writing your own LDtk importer, you should probably just ignore *most* stuff in
/// the `defs` section, as it contains data that are mostly important to the editor. To keep
/// you away from the `defs` section and avoid some unnecessary JSON parsing, important data
/// from definitions is often duplicated in fields prefixed with a double underscore (eg.
/// `__identifier` or `__type`).  The 2 only definition types you might need here are
/// **Tilesets** and **Enums**.
///
/// A structure containing all the definitions of this project
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct Definitions {
    /// All entities definitions, including their custom fields
    pub entities: Vec<EntityDefinition>,

    /// All internal enums
    pub enums: Vec<EnumDefinition>,

    /// Note: external enums are exactly the same as `enums`, except they have a `relPath` to
    /// point to an external source file.
    pub external_enums: Vec<EnumDefinition>,

    /// All layer definitions
    pub layers: Vec<LayerDefinition>,

    /// All custom fields available to all levels.
    pub level_fields: Vec<FieldDefinition>,

    /// All tilesets
    pub tilesets: Vec<TilesetDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct EntityDefinition {
    /// If enabled, this entity is allowed to stay outside of the current level bounds
    pub allow_out_of_bounds: bool,

    /// Base entity color
    #[serde(with = "color")]
    pub color: Color,

    /// User defined documentation for this element to provide help/tips to level designers.
    pub doc: Option<String>,

    /// If enabled, all instances of this entity will be listed in the project "Table of content"
    /// object.
    pub export_to_toc: bool,

    /// Array of field definitions
    pub field_defs: Vec<FieldDefinition>,

    pub fill_opacity: f32,

    /// Pixel height
    pub height: i32,

    pub hollow: bool,

    /// User defined unique identifier
    pub identifier: String,

    /// Only applies to entities resizable on both X/Y. If TRUE, the entity instance width/height
    /// will keep the same aspect ratio as the definition.
    pub keep_aspect_ratio: bool,

    /// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
    pub limit_behavior: LimitBehavior,

    /// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
    /// values: `PerLayer`, `PerLevel`, `PerWorld`
    pub limit_scope: LimitScope,

    pub line_opacity: f32,

    /// Max instances count
    pub max_count: i32,

    /// Max pixel height (only applies if the entity is resizable on Y)
    pub max_height: Option<i32>,

    /// Max pixel width (only applies if the entity is resizable on X)
    pub max_width: Option<i32>,

    /// Min pixel height (only applies if the entity is resizable on Y)
    pub min_height: Option<i32>,

    /// Min pixel width (only applies if the entity is resizable on X)
    pub min_width: Option<i32>,

    /// An array of 4 dimensions for the up/right/down/left borders (in this order) when using
    /// 9-slice mode for `tileRenderMode`.<br/>  If the tileRenderMode is not NineSlice, then
    /// this array is empty.<br/>  See: <https://en.wikipedia.org/wiki/9-slice_scaling>
    pub nine_slice_borders: Vec<i32>,

    /// Pivot X coordinate (from 0 to 1.0)
    pub pivot_x: f32,

    /// Pivot Y coordinate (from 0 to 1.0)
    pub pivot_y: f32,

    /// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
    pub render_mode: RenderMode,

    /// If TRUE, the entity instances will be resizable horizontally
    pub resizable_x: bool,

    /// If TRUE, the entity instances will be resizable vertically
    pub resizable_y: bool,

    /// Display entity name in editor
    pub show_name: bool,

    /// An array of strings that classifies this entity
    pub tags: Vec<String>,

    /// **WARNING**: this deprecated value is no longer exported since version 1.2.0  Replaced
    /// by: `tileRect`
    pub tile_id: Option<i32>,

    pub tile_opacity: f32,

    /// An object representing a rectangle from an existing Tileset
    pub tile_rect: Option<TilesetRectangle>,

    /// An enum describing how the the Entity tile is rendered inside the Entity bounds. Possible
    /// values: `Cover`, `FitInside`, `Repeat`, `Stretch`, `FullSizeCropped`,
    /// `FullSizeUncropped`, `NineSlice`
    pub tile_render_mode: TileRenderMode,

    /// Tileset ID used for optional tile display
    pub tileset_id: Option<i32>,

    /// Unique Int identifier
    pub uid: i32,

    /// This tile overrides the one defined in `tileRect` in the UI
    pub ui_tile_rect: Option<TilesetRectangle>,

    /// Pixel width
    pub width: i32,
}

/// This section is mostly only intended for the LDtk editor app itself. You can safely
/// ignore it.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct FieldDefinition {
    /// Human readable value type. Possible values: `Int, Float, String, Bool, Color,
    /// ExternEnum.XXX, LocalEnum.XXX, Point, FilePath`.<br/>  If the field is an array, this
    /// field will look like `Array<...>` (eg. `Array<Int>`, `Array<Point>` etc.)<br/>  NOTE: if
    /// you enable the advanced option **Use Multilines type**, you will have "*Multilines*"
    /// instead of "*String*" when relevant.
    #[serde(rename = "__type")]
    pub field_definition_type: String,

    /// Optional list of accepted file extensions for FilePath value type. Includes the dot:
    /// `.ext`
    pub accept_file_types: Option<Vec<String>>,

    /// Possible values: `Any`, `OnlySame`, `OnlyTags`, `OnlySpecificEntity`
    pub allowed_refs: AllowedRefs,

    pub allowed_refs_entity_uid: Option<i32>,

    pub allowed_ref_tags: Vec<String>,

    pub allow_out_of_level_ref: bool,

    /// Array max length
    pub array_max_length: Option<i32>,

    /// Array min length
    pub array_min_length: Option<i32>,

    pub auto_chain_ref: bool,

    /// TRUE if the value can be null. For arrays, TRUE means it can contain null values
    /// (exception: array of Points can't have null values).
    pub can_be_null: bool,

    /// Default value if selected value is null or invalid.
    #[reflect(ignore)]
    pub default_override: Option<serde_json::Value>,

    /// User defined documentation for this field to provide help/tips to level designers about
    /// accepted values.
    pub doc: Option<String>,

    pub editor_always_show: bool,

    pub editor_cut_long_values: bool,

    pub editor_display_color: Option<String>,

    /// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `LevelTile`,
    /// `Points`, `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`,
    /// `ArrayCountWithLabel`, `ArrayCountNoLabel`, `RefLinkBetweenPivots`,
    /// `RefLinkBetweenCenters`
    pub editor_display_mode: EditorDisplayMode,

    /// Possible values: `Above`, `Center`, `Beneath`
    pub editor_display_pos: EditorDisplayPos,

    pub editor_display_scale: f32,

    /// Possible values: `ZigZag`, `StraightArrow`, `CurvedArrow`, `ArrowsLine`, `DashedLine`
    pub editor_link_style: EditorLinkStyle,

    pub editor_show_in_world: bool,

    pub editor_text_prefix: Option<String>,

    pub editor_text_suffix: Option<String>,

    /// If TRUE, the field value will be exported to the `toc` project JSON field. Only applies
    /// to Entity fields.
    pub export_to_toc: bool,

    /// User defined unique identifier
    pub identifier: String,

    /// TRUE if the value is an array of multiple values
    pub is_array: bool,

    /// Max limit for value, if applicable
    pub max: Option<f32>,

    /// Min limit for value, if applicable
    pub min: Option<f32>,

    /// Optional regular expression that needs to be matched to accept values. Expected format:
    /// `/some_reg_ex/g`, with optional "i" flag.
    pub regex: Option<String>,

    /// If enabled, this field will be searchable through LDtk command palette
    pub searchable: bool,

    pub symmetrical_ref: bool,

    /// Possible values: &lt;`null`&gt;, `LangPython`, `LangRuby`, `LangJS`, `LangLua`, `LangC`,
    /// `LangHaxe`, `LangMarkdown`, `LangJson`, `LangXml`, `LangLog`
    pub text_language_mode: Option<TextLanguageMode>,

    /// UID of the tileset used for a Tile
    pub tileset_uid: Option<i32>,

    /// Internal enum representing the possible field types. Possible values: F_Int, F_Float,
    /// F_String, F_Text, F_Bool, F_Color, F_Enum(...), F_Point, F_Path, F_EntityRef, F_Tile
    #[serde(rename = "type")]
    pub purple_type: String,

    /// Unique Int identifier
    pub uid: i32,

    /// If TRUE, the color associated with this field will override the Entity or Level default
    /// color in the editor UI. For Enum fields, this would be the color associated to their
    /// values.
    pub use_for_smart_color: bool,
}

/// Possible values: `Any`, `OnlySame`, `OnlyTags`, `OnlySpecificEntity`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum AllowedRefs {
    #[default]
    Any,

    #[serde(rename = "OnlySame")]
    OnlySame,

    #[serde(rename = "OnlySpecificEntity")]
    OnlySpecificEntity,

    #[serde(rename = "OnlyTags")]
    OnlyTags,
}

/// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `LevelTile`,
/// `Points`, `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`,
/// `ArrayCountWithLabel`, `ArrayCountNoLabel`, `RefLinkBetweenPivots`,
/// `RefLinkBetweenCenters`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum EditorDisplayMode {
    #[default]
    #[serde(rename = "ArrayCountNoLabel")]
    ArrayCountNoLabel,

    #[serde(rename = "ArrayCountWithLabel")]
    ArrayCountWithLabel,

    #[serde(rename = "EntityTile")]
    EntityTile,

    Hidden,

    #[serde(rename = "LevelTile")]
    LevelTile,

    #[serde(rename = "NameAndValue")]
    NameAndValue,

    #[serde(rename = "PointPath")]
    PointPath,

    #[serde(rename = "PointPathLoop")]
    PointPathLoop,

    #[serde(rename = "PointStar")]
    PointStar,

    Points,

    #[serde(rename = "RadiusGrid")]
    RadiusGrid,

    #[serde(rename = "RadiusPx")]
    RadiusPx,

    #[serde(rename = "RefLinkBetweenCenters")]
    RefLinkBetweenCenters,

    #[serde(rename = "RefLinkBetweenPivots")]
    RefLinkBetweenPivots,

    #[serde(rename = "ValueOnly")]
    ValueOnly,
}

/// Possible values: `Above`, `Center`, `Beneath`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum EditorDisplayPos {
    #[default]
    Above,

    Beneath,

    Center,
}

/// Possible values: `ZigZag`, `StraightArrow`, `CurvedArrow`, `ArrowsLine`, `DashedLine`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum EditorLinkStyle {
    #[default]
    #[serde(rename = "ArrowsLine")]
    ArrowsLine,

    #[serde(rename = "CurvedArrow")]
    CurvedArrow,

    #[serde(rename = "DashedLine")]
    DashedLine,

    #[serde(rename = "StraightArrow")]
    StraightArrow,

    #[serde(rename = "ZigZag")]
    ZigZag,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum TextLanguageMode {
    #[default]
    #[serde(rename = "LangC")]
    LangC,

    #[serde(rename = "LangHaxe")]
    LangHaxe,

    #[serde(rename = "LangJS")]
    LangJs,

    #[serde(rename = "LangJson")]
    LangJson,

    #[serde(rename = "LangLog")]
    LangLog,

    #[serde(rename = "LangLua")]
    LangLua,

    #[serde(rename = "LangMarkdown")]
    LangMarkdown,

    #[serde(rename = "LangPython")]
    LangPython,

    #[serde(rename = "LangRuby")]
    LangRuby,

    #[serde(rename = "LangXml")]
    LangXml,
}

/// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum LimitBehavior {
    #[serde(rename = "DiscardOldOnes")]
    DiscardOldOnes,

    #[default]
    #[serde(rename = "MoveLastOne")]
    MoveLastOne,

    #[serde(rename = "PreventAdding")]
    PreventAdding,
}

/// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
/// values: `PerLayer`, `PerLevel`, `PerWorld`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum LimitScope {
    #[serde(rename = "PerLayer")]
    PerLayer,

    #[default]
    #[serde(rename = "PerLevel")]
    PerLevel,

    #[serde(rename = "PerWorld")]
    PerWorld,
}

/// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum RenderMode {
    Cross,

    Ellipse,

    #[default]
    Rectangle,

    Tile,
}

/// This object represents a custom sub rectangle in a Tileset image.
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct TilesetRectangle {
    /// Height in pixels
    pub h: i32,

    /// UID of the tileset
    pub tileset_uid: i32,

    /// Width in pixels
    pub w: i32,

    /// X pixels coordinate of the top-left corner in the Tileset image
    pub x: i32,

    /// Y pixels coordinate of the top-left corner in the Tileset image
    pub y: i32,
}

/// An enum describing how the the Entity tile is rendered inside the Entity bounds. Possible
/// values: `Cover`, `FitInside`, `Repeat`, `Stretch`, `FullSizeCropped`,
/// `FullSizeUncropped`, `NineSlice`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum TileRenderMode {
    Cover,

    #[default]
    #[serde(rename = "FitInside")]
    FitInside,

    #[serde(rename = "FullSizeCropped")]
    FullSizeCropped,

    #[serde(rename = "FullSizeUncropped")]
    FullSizeUncropped,

    #[serde(rename = "NineSlice")]
    NineSlice,

    Repeat,

    Stretch,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct EnumDefinition {
    pub external_file_checksum: Option<String>,

    /// Relative path to the external file providing this Enum
    pub external_rel_path: Option<String>,

    /// Tileset UID if provided
    pub icon_tileset_uid: Option<i32>,

    /// User defined unique identifier
    pub identifier: String,

    /// An array of user-defined tags to organize the Enums
    pub tags: Vec<String>,

    /// Unique Int identifier
    pub uid: i32,

    /// All possible enum values, with their optional Tile infos.
    pub values: Vec<EnumValueDefinition>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct EnumValueDefinition {
    /// **WARNING**: this deprecated value is no longer exported since version 1.4.0  Replaced
    /// by: `tileRect`
    #[serde(rename = "__tileSrcRect")]
    pub tile_src_rect: Option<Vec<i32>>,

    /// Optional color
    pub color: i32,

    /// Enum value
    pub id: String,

    /// **WARNING**: this deprecated value is no longer exported since version 1.4.0  Replaced
    /// by: `tileRect`
    pub tile_id: Option<i32>,

    /// Optional tileset rectangle to represents this value
    pub tile_rect: Option<TilesetRectangle>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct LayerDefinition {
    /// Type of the layer (*IntGrid, Entities, Tiles or AutoLayer*)
    #[serde(rename = "__type")]
    pub layer_definition_type: String,

    /// Contains all the auto-layer rule definitions.
    pub auto_rule_groups: Vec<AutoLayerRuleGroup>,

    pub auto_source_layer_def_uid: Option<i32>,

    /// **WARNING**: this deprecated value is no longer exported since version 1.2.0  Replaced
    /// by: `tilesetDefUid`
    pub auto_tileset_def_uid: Option<i32>,

    pub auto_tiles_killed_by_other_layer_uid: Option<i32>,

    pub biome_field_uid: Option<i32>,

    /// Allow editor selections when the layer is not currently active.
    pub can_select_when_inactive: bool,

    /// Opacity of the layer (0 to 1.0)
    pub display_opacity: f32,

    /// User defined documentation for this element to provide help/tips to level designers.
    pub doc: Option<String>,

    /// An array of tags to forbid some Entities in this layer
    pub excluded_tags: Vec<String>,

    /// Width and height of the grid in pixels
    pub grid_size: i32,

    /// Height of the optional "guide" grid in pixels
    pub guide_grid_hei: i32,

    /// Width of the optional "guide" grid in pixels
    pub guide_grid_wid: i32,

    pub hide_fields_when_inactive: bool,

    /// Hide the layer from the list on the side of the editor view.
    pub hide_in_list: bool,

    /// User defined unique identifier
    pub identifier: String,

    /// Alpha of this layer when it is not the active one.
    pub inactive_opacity: f32,

    /// An array that defines extra optional info for each IntGrid value.<br/>  WARNING: the
    /// array order is not related to actual IntGrid values! As user can re-order IntGrid values
    /// freely, you may value "2" before value "1" in this array.
    pub int_grid_values: Vec<IntGridValueDefinition>,

    /// Group informations for IntGrid values
    pub int_grid_values_groups: Vec<IntGridValueGroupDefinition>,

    /// Parallax horizontal factor (from -1 to 1, defaults to 0) which affects the scrolling
    /// speed of this layer, creating a fake 3D (parallax) effect.
    pub parallax_factor_x: f32,

    /// Parallax vertical factor (from -1 to 1, defaults to 0) which affects the scrolling speed
    /// of this layer, creating a fake 3D (parallax) effect.
    pub parallax_factor_y: f32,

    /// If true (default), a layer with a parallax factor will also be scaled up/down accordingly.
    pub parallax_scaling: bool,

    /// X offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    pub px_offset_x: i32,

    /// Y offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    pub px_offset_y: i32,

    /// If TRUE, the content of this layer will be used when rendering levels in a simplified way
    /// for the world view
    pub render_in_world_view: bool,

    /// An array of tags to filter Entities that can be added to this layer
    pub required_tags: Vec<String>,

    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    pub tile_pivot_x: f32,

    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    pub tile_pivot_y: f32,

    /// Reference to the default Tileset UID being used by this layer definition.<br/>
    /// **WARNING**: some layer *instances* might use a different tileset. So most of the time,
    /// you should probably use the `__tilesetDefUid` value found in layer instances.<br/>  Note:
    /// since version 1.0.0, the old `autoTilesetDefUid` was removed and merged into this value.
    pub tileset_def_uid: Option<i32>,

    /// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
    /// `AutoLayer`
    #[serde(rename = "type")]
    pub purple_type: Type,

    /// User defined color for the UI
    #[serde(with = "color::optional")]
    pub ui_color: Option<Color>,

    /// Unique Int identifier
    pub uid: i32,

    /// Display tags
    pub ui_filter_tags: Vec<String>,

    /// Asynchronous rendering option for large/complex layers
    pub use_async_render: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct AutoLayerRuleGroup {
    pub active: bool,

    pub biome_requirement_mode: i32,

    /// *This field was removed in 1.0.0 and should no longer be used.*
    pub collapsed: Option<bool>,

    pub color: Option<String>,

    pub icon: Option<TilesetRectangle>,

    pub is_optional: bool,

    pub name: String,

    pub required_biome_values: Vec<String>,

    pub rules: Vec<AutoLayerRuleDefinition>,

    pub uid: i32,

    pub uses_wizard: bool,
}

/// This complex section isn't meant to be used by game devs at all, as these rules are
/// completely resolved internally by the editor before any saving. You should just ignore
/// this part.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct AutoLayerRuleDefinition {
    /// If FALSE, the rule effect isn't applied, and no tiles are generated.
    pub active: bool,

    pub alpha: f32,

    /// When TRUE, the rule will prevent other rules to be applied in the same cell if it matches
    /// (TRUE by default).
    pub break_on_match: bool,

    /// Chances for this rule to be applied (0 to 1)
    pub chance: f32,

    /// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
    pub checker: Checker,

    /// If TRUE, allow rule to be matched by flipping its pattern horizontally
    pub flip_x: bool,

    /// If TRUE, allow rule to be matched by flipping its pattern vertically
    pub flip_y: bool,

    /// If TRUE, then the rule should be re-evaluated by the editor at one point
    pub invalidated: bool,

    /// Default IntGrid value when checking cells outside of level bounds
    pub out_of_bounds_value: Option<i32>,

    /// Rule pattern (size x size)
    pub pattern: Vec<i32>,

    /// If TRUE, enable Perlin filtering to only apply rule on specific random area
    pub perlin_active: bool,

    pub perlin_octaves: f32,

    pub perlin_scale: f32,

    pub perlin_seed: f32,

    /// X pivot of a tile stamp (0-1)
    pub pivot_x: f32,

    /// Y pivot of a tile stamp (0-1)
    pub pivot_y: f32,

    /// Pattern width & height. Should only be 1,3,5 or 7.
    pub size: i32,

    /// **WARNING**: this deprecated value is no longer exported since version 1.5.0  Replaced
    /// by: `tileRectsIds`
    pub tile_ids: Option<Vec<i32>>,

    /// Defines how tileIds array is used Possible values: `Single`, `Stamp`
    pub tile_mode: TileMode,

    /// Max random offset for X tile pos
    pub tile_random_x_max: i32,

    /// Min random offset for X tile pos
    pub tile_random_x_min: i32,

    /// Max random offset for Y tile pos
    pub tile_random_y_max: i32,

    /// Min random offset for Y tile pos
    pub tile_random_y_min: i32,

    /// Array containing all the possible tile IDs rectangles (picked randomly).
    pub tile_rects_ids: Vec<Vec<i32>>,

    /// Tile X offset
    pub tile_x_offset: i32,

    /// Tile Y offset
    pub tile_y_offset: i32,

    /// Unique Int identifier
    pub uid: i32,

    /// X cell coord modulo
    pub x_modulo: i32,

    /// X cell start offset
    pub x_offset: i32,

    /// Y cell coord modulo
    pub y_modulo: i32,

    /// Y cell start offset
    pub y_offset: i32,
}

/// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum Checker {
    #[default]
    Horizontal,

    None,

    Vertical,
}

/// Defines how tileIds array is used Possible values: `Single`, `Stamp`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum TileMode {
    #[default]
    Single,

    Stamp,
}

/// IntGrid value definition
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct IntGridValueDefinition {
    #[serde(with = "color")]
    pub color: Color,

    /// Parent group identifier (0 if none)
    pub group_uid: i32,

    /// User defined unique identifier
    pub identifier: Option<String>,

    pub tile: Option<TilesetRectangle>,

    /// The IntGrid value itself
    pub value: i32,
}

/// IntGrid value group definition
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
pub struct IntGridValueGroupDefinition {
    /// User defined color
    pub color: Option<String>,

    /// User defined string identifier
    pub identifier: Option<String>,

    /// Group unique ID
    pub uid: i32,
}

/// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
/// `AutoLayer`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum Type {
    #[serde(rename = "AutoLayer")]
    AutoLayer,

    Entities,

    #[serde(rename = "IntGrid")]
    IntGrid,

    #[default]
    Tiles,
}

/// The `Tileset` definition is the most important part among project definitions. It
/// contains some extra informations about each integrated tileset. If you only had to parse
/// one definition section, that would be the one.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct TilesetDefinition {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub c_hei: i32,

    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub c_wid: i32,

    /// The following data is used internally for various optimizations. It's always synced with
    /// source image changes.
    #[reflect(ignore)]
    pub cached_pixel_data: Option<HashMap<String, Option<serde_json::Value>>>,

    /// An array of custom tile metadata
    pub custom_data: Vec<TileCustomMetadata>,

    /// If this value is set, then it means that this atlas uses an internal LDtk atlas image
    /// instead of a loaded one. Possible values: &lt;`null`&gt;, `LdtkIcons`
    pub embed_atlas: Option<EmbedAtlas>,

    /// Tileset tags using Enum values specified by `tagsSourceEnumId`. This array contains 1
    /// element per Enum value, which contains an array of all Tile IDs that are tagged with it.
    pub enum_tags: Vec<EnumTagValue>,

    /// User defined unique identifier
    pub identifier: String,

    /// Distance in pixels from image borders
    pub padding: i32,

    /// Image height in pixels
    pub px_hei: i32,

    /// Image width in pixels
    pub px_wid: i32,

    /// Path to the source file, relative to the current project JSON file<br/>  It can be null
    /// if no image was provided, or when using an embed atlas.
    pub rel_path: Option<String>,

    /// Array of group of tiles selections, only meant to be used in the editor
    #[reflect(ignore)]
    pub saved_selections: Vec<HashMap<String, Option<serde_json::Value>>>,

    /// Space in pixels between all tiles
    pub spacing: i32,

    /// An array of user-defined tags to organize the Tilesets
    pub tags: Vec<String>,

    /// Optional Enum definition UID used for this tileset meta-data
    pub tags_source_enum_uid: Option<i32>,

    pub tile_grid_size: i32,

    /// Unique Intidentifier
    pub uid: i32,
}

/// In a tileset definition, user defined meta-data of a tile.
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct TileCustomMetadata {
    pub data: String,

    pub tile_id: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum EmbedAtlas {
    #[default]
    #[serde(rename = "LdtkIcons")]
    LdtkIcons,
}

/// In a tileset definition, enum based tag infos
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct EnumTagValue {
    pub enum_value_id: String,

    pub tile_ids: Vec<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum Flag {
    #[default]
    #[serde(rename = "DiscardPreCsvIntGrid")]
    DiscardPreCsvIntGrid,

    #[serde(rename = "ExportOldTableOfContentData")]
    ExportOldTableOfContentData,

    #[serde(rename = "ExportPreCsvIntGridFormat")]
    ExportPreCsvIntGridFormat,

    #[serde(rename = "IgnoreBackupSuggest")]
    IgnoreBackupSuggest,

    #[serde(rename = "MultiWorlds")]
    MultiWorlds,

    #[serde(rename = "PrependIndexToLevelFileNames")]
    PrependIndexToLevelFileNames,

    #[serde(rename = "UseMultilinesType")]
    UseMultilinesType,
}

/// This object is not actually used by LDtk. It ONLY exists to force explicit references to
/// all types, to make sure QuickType finds them and integrate all of them. Otherwise,
/// Quicktype will drop types that are not explicitely used.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "PascalCase")]
pub struct ForcedRefs {
    pub auto_layer_rule_group: Option<AutoLayerRuleGroup>,

    pub auto_rule_def: Option<AutoLayerRuleDefinition>,

    pub custom_command: Option<LdtkCustomCommand>,

    pub definitions: Option<Definitions>,

    pub entity_def: Option<EntityDefinition>,

    pub entity_instance: Option<EntityInstance>,

    pub entity_reference_infos: Option<ReferenceToAnEntityInstance>,

    pub enum_def: Option<EnumDefinition>,

    pub enum_def_values: Option<EnumValueDefinition>,

    pub enum_tag_value: Option<EnumTagValue>,

    pub field_def: Option<FieldDefinition>,

    pub field_instance: Option<FieldInstance>,

    pub grid_point: Option<GridPoint>,

    pub int_grid_value_def: Option<IntGridValueDefinition>,

    pub int_grid_value_group_def: Option<IntGridValueGroupDefinition>,

    pub int_grid_value_instance: Option<IntGridValueInstance>,

    pub layer_def: Option<LayerDefinition>,

    pub layer_instance: Option<LayerInstance>,

    pub level: Option<Level>,

    pub level_bg_pos_infos: Option<LevelBackgroundPosition>,

    pub neighbour_level: Option<NeighbourLevel>,

    pub table_of_content_entry: Option<LdtkTableOfContentEntry>,

    pub tile: Option<TileInstance>,

    pub tile_custom_metadata: Option<TileCustomMetadata>,

    pub tileset_def: Option<TilesetDefinition>,

    pub tileset_rect: Option<TilesetRectangle>,

    pub toc_instance_data: Option<LdtkTocInstanceData>,

    pub world: Option<World>,
}

/// Component added to any LDtk Entity by default.
///
/// When loading levels, you can flesh out LDtk entities in your own system by querying for
/// `Added<EntityInstance>`.
/// Or, you can hook into the entity's spawning process using [LdtkEntity].
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect, Component)]
#[serde(rename_all = "camelCase")]
pub struct EntityInstance {
    /// Grid-based coordinates (`[x,y]` format)
    #[serde(rename = "__grid")]
    pub grid: IVec2,

    /// Entity definition identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,

    /// Pivot coordinates  (`[x,y]` format, values are from 0 to 1) of the Entity
    #[serde(rename = "__pivot")]
    pub pivot: Vec2,

    /// The entity "smart" color, guessed from either Entity definition, or one its field
    /// instances.
    #[serde(rename = "__smartColor", with = "color")]
    pub smart_color: Color,

    /// Array of tags defined in this Entity definition
    #[serde(rename = "__tags")]
    pub tags: Vec<String>,

    /// Optional TilesetRect used to display this entity (it could either be the default Entity
    /// tile, or some tile provided by a field value, like an Enum).
    #[serde(rename = "__tile")]
    pub tile: Option<TilesetRectangle>,

    /// X world coordinate in pixels. Only available in GridVania or Free world layouts.
    #[serde(rename = "__worldX")]
    pub world_x: Option<i32>,

    /// Y world coordinate in pixels Only available in GridVania or Free world layouts.
    #[serde(rename = "__worldY")]
    pub world_y: Option<i32>,

    /// Reference of the **Entity definition** UID
    pub def_uid: i32,

    /// An array of all custom fields and their values.
    pub field_instances: Vec<FieldInstance>,

    /// Entity height in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    pub height: i32,

    /// Unique instance identifier
    pub iid: String,

    /// Pixel coordinates (`[x,y]` format) in current level coordinate space. Don't forget
    /// optional layer offsets, if they exist!
    pub px: IVec2,

    /// Entity width in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    pub width: i32,
}

impl From<&EntityInstance> for EntityInstance {
    fn from(value: &EntityInstance) -> Self {
        value.clone()
    }
}

/// This object describes the "location" of an Entity instance in the project worlds.
///
/// IID information of this instance
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct ReferenceToAnEntityInstance {
    /// IID of the refered EntityInstance
    pub entity_iid: String,

    /// IID of the LayerInstance containing the refered EntityInstance
    pub layer_iid: String,

    /// IID of the Level containing the refered EntityInstance
    pub level_iid: String,

    /// IID of the World containing the refered EntityInstance
    pub world_iid: String,
}

/// This object is just a grid-based coordinate used in Field values.
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub struct GridPoint {
    /// X grid-based coordinate
    pub cx: i32,

    /// Y grid-based coordinate
    pub cy: i32,
}

/// IntGrid value instance
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct IntGridValueInstance {
    /// Coordinate ID in the layer grid
    pub coord_id: i32,

    /// IntGrid value
    pub v: i32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct LayerInstance {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub c_hei: i32,

    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub c_wid: i32,

    /// Grid size
    #[serde(rename = "__gridSize")]
    pub grid_size: i32,

    /// Layer definition identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,

    /// Layer opacity as Float [0-1]
    #[serde(rename = "__opacity")]
    pub opacity: f32,

    /// Total layer X pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetX")]
    pub px_total_offset_x: i32,

    /// Total layer Y pixel offset, including both instance and definition offsets.
    #[serde(rename = "__pxTotalOffsetY")]
    pub px_total_offset_y: i32,

    /// The definition UID of corresponding Tileset, if any.
    #[serde(rename = "__tilesetDefUid")]
    pub tileset_def_uid: Option<i32>,

    /// The relative path to corresponding Tileset, if any.
    #[serde(rename = "__tilesetRelPath")]
    pub tileset_rel_path: Option<String>,

    /// Layer type (possible values: IntGrid, Entities, Tiles or AutoLayer)
    #[serde(rename = "__type")]
    pub layer_instance_type: Type,

    /// An array containing all tiles generated by Auto-layer rules. The array is already sorted
    /// in display order (ie. 1st tile is beneath 2nd, which is beneath 3rd etc.).<br/><br/>
    /// Note: if multiple tiles are stacked in the same cell as the result of different rules,
    /// all tiles behind opaque ones will be discarded.
    pub auto_layer_tiles: Vec<TileInstance>,

    pub entity_instances: Vec<EntityInstance>,

    pub grid_tiles: Vec<TileInstance>,

    /// Unique layer instance identifier
    pub iid: String,

    /// **WARNING**: this deprecated value is no longer exported since version 1.0.0  Replaced
    /// by: `intGridCsv`
    pub int_grid: Option<Vec<IntGridValueInstance>>,

    /// A list of all values in the IntGrid layer, stored in CSV format (Comma Separated
    /// Values).<br/>  Order is from left to right, and top to bottom (ie. first row from left to
    /// right, followed by second row, etc).<br/>  `0` means "empty cell" and IntGrid values
    /// start at 1.<br/>  The array size is `__cWid` x `__cHei` cells.
    pub int_grid_csv: Vec<i32>,

    /// Reference the Layer definition UID
    pub layer_def_uid: i32,

    /// Reference to the UID of the level containing this layer instance
    pub level_id: i32,

    /// An Array containing the UIDs of optional rules that were enabled in this specific layer
    /// instance.
    pub optional_rules: Vec<i32>,

    /// This layer can use another tileset by overriding the tileset UID here.
    pub override_tileset_uid: Option<i32>,

    /// X offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, so you should probably prefer using `__pxTotalOffsetX`
    /// which contains the total offset value)
    pub px_offset_x: i32,

    /// Y offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, so you should probably prefer using `__pxTotalOffsetX`
    /// which contains the total offset value)
    pub px_offset_y: i32,

    /// Random seed used for Auto-Layers rendering
    pub seed: i32,

    /// Layer instance visibility
    pub visible: bool,
}

/// This structure represents a single tile from a given Tileset.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
pub struct TileInstance {
    /// Alpha/opacity of the tile (0-1, defaults to 1)
    pub a: f32,

    /// Internal data used by the editor.<br/>  For auto-layer tiles: `[ruleId, coordId]`.<br/>
    /// For tile-layer tiles: `[coordId]`.
    pub d: Vec<i32>,

    /// "Flip bits", a 2-bits integer to represent the mirror transformations of the tile.<br/>
    /// - Bit 0 = X flip<br/>   - Bit 1 = Y flip<br/>   Examples: f=0 (no flip), f=1 (X flip
    /// only), f=2 (Y flip only), f=3 (both flips)
    pub f: i32,

    /// Pixel coordinates of the tile in the **layer** (`[x,y]` format). Don't forget optional
    /// layer offsets, if they exist!
    pub px: IVec2,

    /// Pixel coordinates of the tile in the **tileset** (`[x,y]` format)
    pub src: IVec2,

    /// The *Tile ID* in the corresponding tileset.
    pub t: i32,
}

/// This section contains all the level data. It can be found in 2 distinct forms, depending
/// on Project current settings:  - If "*Separate level files*" is **disabled** (default):
/// full level data is *embedded* inside the main Project JSON file, - If "*Separate level
/// files*" is **enabled**: level data is stored in *separate* standalone `.ldtkl` files (one
/// per level). In this case, the main Project JSON file will still contain most level data,
/// except heavy sections, like the `layerInstances` array (which will be null). The
/// `externalRelPath` string points to the `ldtkl` file.  A `ldtkl` file is just a JSON file
/// containing exactly what is described below.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct Level {
    /// Background color of the level (same as `bgColor`, except the default value is
    /// automatically used here if its value is `null`)
    #[serde(rename = "__bgColor", with = "color")]
    pub bg_color: Color,

    /// Position informations of the background image, if there is one.
    #[serde(rename = "__bgPos")]
    pub bg_pos: Option<LevelBackgroundPosition>,

    /// An array listing all other levels touching this one on the world map. Since 1.4.0, this
    /// includes levels that overlap in the same world layer, or in nearby world layers.<br/>
    /// Only relevant for world layouts where level spatial positioning is manual (ie. GridVania,
    /// Free). For Horizontal and Vertical layouts, this array is always empty.
    #[serde(rename = "__neighbours")]
    pub neighbours: Vec<NeighbourLevel>,

    /// The "guessed" color for this level in the editor, decided using either the background
    /// color or an existing custom field.
    #[serde(rename = "__smartColor", with = "color")]
    pub smart_color: Color,

    /// Background color of the level. If `null`, the project `defaultLevelBgColor` should be
    /// used.
    #[serde(rename = "bgColor", with = "color::optional")]
    pub level_bg_color: Option<Color>,

    /// Background image X pivot (0-1)
    pub bg_pivot_x: f32,

    /// Background image Y pivot (0-1)
    pub bg_pivot_y: f32,

    /// An enum defining the way the background image (if any) is positioned on the level. See
    /// `__bgPos` for resulting position info. Possible values: &lt;`null`&gt;, `Unscaled`,
    /// `Contain`, `Cover`, `CoverDirty`, `Repeat`
    #[serde(rename = "bgPos")]
    pub level_bg_pos: Option<BgPos>,

    /// The *optional* relative path to the level background image.
    pub bg_rel_path: Option<String>,

    /// This value is not null if the project option "*Save levels separately*" is enabled. In
    /// this case, this **relative** path points to the level Json file.
    pub external_rel_path: Option<String>,

    /// An array containing this level custom field values.
    pub field_instances: Vec<FieldInstance>,

    /// User defined unique identifier
    pub identifier: String,

    /// Unique instance identifier
    pub iid: String,

    /// An array containing all Layer instances. **IMPORTANT**: if the project option "*Save
    /// levels separately*" is enabled, this field will be `null`.<br/>  This array is **sorted
    /// in display order**: the 1st layer is the top-most and the last is behind.
    pub layer_instances: Option<Vec<LayerInstance>>,

    /// Height of the level in pixels
    pub px_hei: i32,

    /// Width of the level in pixels
    pub px_wid: i32,

    /// Unique Int identifier
    pub uid: i32,

    /// If TRUE, the level identifier will always automatically use the naming pattern as defined
    /// in `Project.levelNamePattern`. Becomes FALSE if the identifier is manually modified by
    /// user.
    pub use_auto_identifier: bool,

    /// Index that represents the "depth" of the level in the world. Default is 0, greater means
    /// "above", lower means "below".<br/>  This value is mostly used for display only and is
    /// intended to make stacking of levels easier to manage.
    pub world_depth: i32,

    /// World X coordinate in pixels.<br/>  Only relevant for world layouts where level spatial
    /// positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the
    /// value is always -1 here.
    pub world_x: i32,

    /// World Y coordinate in pixels.<br/>  Only relevant for world layouts where level spatial
    /// positioning is manual (ie. GridVania, Free). For Horizontal and Vertical layouts, the
    /// value is always -1 here.
    pub world_y: i32,
}

/// Level background image position info
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct LevelBackgroundPosition {
    /// An array of 4 float values describing the cropped sub-rectangle of the displayed
    /// background image. This cropping happens when original is larger than the level bounds.
    /// Array format: `[ cropX, cropY, cropWidth, cropHeight ]`
    pub crop_rect: Vec<f32>,

    /// An array containing the `[scaleX,scaleY]` values of the **cropped** background image,
    /// depending on `bgPos` option.
    pub scale: Vec2,

    /// An array containing the `[x,y]` pixel coordinates of the top-left corner of the
    /// **cropped** background image, depending on `bgPos` option.
    pub top_left_px: IVec2,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum BgPos {
    #[default]
    Contain,

    Cover,

    #[serde(rename = "CoverDirty")]
    CoverDirty,

    Repeat,

    Unscaled,
}

/// Nearby level info
#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct NeighbourLevel {
    /// A lowercase string tipping on the level location (`n`orth, `s`outh, `w`est,
    /// `e`ast).<br/>  Since 1.4.0, this value can also be `<` (neighbour depth is lower), `>`
    /// (neighbour depth is greater) or `o` (levels overlap and share the same world
    /// depth).<br/>  Since 1.5.3, this value can also be `nw`,`ne`,`sw` or `se` for levels only
    /// touching corners.
    pub dir: String,

    /// Neighbour Instance Identifier
    pub level_iid: String,

    /// **WARNING**: this deprecated value is no longer exported since version 1.2.0  Replaced
    /// by: `levelIid`
    pub level_uid: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct LdtkTableOfContentEntry {
    pub identifier: String,

    /// **WARNING**: this deprecated value will be *removed* completely on version 1.7.0+
    /// Replaced by: `instancesData`
    pub instances: Option<Vec<ReferenceToAnEntityInstance>>,

    pub instances_data: Vec<LdtkTocInstanceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default, Eq, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct LdtkTocInstanceData {
    /// An object containing the values of all entity fields with the `exportToToc` option
    /// enabled. This object typing depends on actual field value types.
    #[reflect(ignore)]
    pub fields: Option<serde_json::Value>,

    pub hei_px: i32,

    /// IID information of this instance
    pub iids: ReferenceToAnEntityInstance,

    pub wid_px: i32,

    pub world_x: i32,

    pub world_y: i32,
}

/// **IMPORTANT**: this type is available as a preview. You can rely on it to update your
/// importers, for when it will be officially available.  A World contains multiple levels,
/// and it has its own layout settings.
#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Reflect)]
#[serde(rename_all = "camelCase")]
pub struct World {
    /// Default new level height
    pub default_level_height: i32,

    /// Default new level width
    pub default_level_width: i32,

    /// User defined unique identifier
    pub identifier: String,

    /// Unique instance identifer
    pub iid: String,

    /// All levels from this world. The order of this array is only relevant in
    /// `LinearHorizontal` and `linearVertical` world layouts (see `worldLayout` value).
    /// Otherwise, you should refer to the `worldX`,`worldY` coordinates of each Level.
    pub levels: Vec<Level>,

    /// Height of the world grid in pixels.
    pub world_grid_height: i32,

    /// Width of the world grid in pixels.
    pub world_grid_width: i32,

    /// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
    /// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`, `null`
    pub world_layout: Option<WorldLayout>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum WorldLayout {
    #[default]
    Free,

    #[serde(rename = "GridVania")]
    GridVania,

    #[serde(rename = "LinearHorizontal")]
    LinearHorizontal,

    #[serde(rename = "LinearVertical")]
    LinearVertical,
}

/// Naming convention for Identifiers (first-letter uppercase, full uppercase etc.) Possible
/// values: `Capitalize`, `Uppercase`, `Lowercase`, `Free`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum IdentifierStyle {
    #[default]
    Capitalize,

    Free,

    Lowercase,

    Uppercase,
}

/// "Image export" option when saving project. Possible values: `None`, `OneImagePerLayer`,
/// `OneImagePerLevel`, `LayersAndLevels`
#[derive(Debug, Clone, Serialize, Deserialize, Copy, Default, Eq, PartialEq, Reflect)]
pub enum ImageExportMode {
    #[default]
    #[serde(rename = "LayersAndLevels")]
    LayersAndLevels,

    None,

    #[serde(rename = "OneImagePerLayer")]
    OneImagePerLayer,

    #[serde(rename = "OneImagePerLevel")]
    OneImagePerLevel,
}
