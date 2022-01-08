/// Contains all the types for serializing/deserializing an LDtk file.
///
/// This is mostly based on LDtk's existing rust
/// [QuickType loader](https://ldtk.io/files/quicktype/LdtkJson.rs).
///
/// For the most part, changes to the generated module are avoided to make it simpler to maintain
/// this plugin in the future.
/// However, some usability concerns have been addressed.
/// Any changes should be documented here for maintenance purposes:
/// 1. [serde] has been imported with `use` instead of `extern`
/// 2. All struct fields have been made public.
/// 3. [Eq], [PartialEq], [Debug], [Default], and [Clone] have been derived wherever possible.
/// 4. [i64] and [f64] have been changed to [i32] and [f32].
/// 5. [LimitBehavior], [LimitScope], [RenderMode], and [TileRenderMode] have been given custom
///    [Default] implementations.
/// 6. [bevy::ecs::Component] has been derived for [EntityInstance]
/// 7. [FieldInstance] has been moved to its own module, and is re-exported here.
/// 8. The `layer_instance_type` field of [LayerInstance] has been re-typed to [Type]
/// 9. Comment at the top of the file has been replaced with this documentation.
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

mod field_instance;

pub use field_instance::*;

/// This file is a JSON schema of files created by LDtk level editor <https://ldtk.io>.
///
/// This is the root of any Project JSON file. It contains:  - the project settings, - an
/// array of levels, - a group of definitions (that can probably be safely ignored for most
/// users).
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LdtkJson {
    /// Number of backup files to keep, if the `backupOnSave` is TRUE
    #[serde(rename = "backupLimit")]
    pub backup_limit: i32,

    /// If TRUE, an extra copy of the project will be created in a sub folder, when saving.
    #[serde(rename = "backupOnSave")]
    pub backup_on_save: bool,

    /// Project background color
    #[serde(rename = "bgColor")]
    pub bg_color: String,

    /// Default grid size for new layers
    #[serde(rename = "defaultGridSize")]
    pub default_grid_size: i32,

    /// Default background color of levels
    #[serde(rename = "defaultLevelBgColor")]
    pub default_level_bg_color: String,

    /// Default new level height
    #[serde(rename = "defaultLevelHeight")]
    pub default_level_height: i32,

    /// Default new level width
    #[serde(rename = "defaultLevelWidth")]
    pub default_level_width: i32,

    /// Default X pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotX")]
    pub default_pivot_x: f32,

    /// Default Y pivot (0 to 1) for new entities
    #[serde(rename = "defaultPivotY")]
    pub default_pivot_y: f32,

    /// A structure containing all the definitions of this project
    #[serde(rename = "defs")]
    pub defs: Definitions,

    /// **WARNING**: this deprecated value is no longer exported since version 0.9.3  Replaced
    /// by: `imageExportMode`
    #[serde(rename = "exportPng")]
    pub export_png: Option<bool>,

    /// If TRUE, a Tiled compatible file will also be generated along with the LDtk JSON file
    /// (default is FALSE)
    #[serde(rename = "exportTiled")]
    pub export_tiled: bool,

    /// If TRUE, one file will be saved for the project (incl. all its definitions) and one file
    /// in a sub-folder for each level.
    #[serde(rename = "externalLevels")]
    pub external_levels: bool,

    /// An array containing various advanced flags (ie. options or other states). Possible
    /// values: `DiscardPreCsvIntGrid`, `IgnoreBackupSuggest`
    #[serde(rename = "flags")]
    pub flags: Vec<Flag>,

    /// "Image export" option when saving project. Possible values: `None`, `OneImagePerLayer`,
    /// `OneImagePerLevel`
    #[serde(rename = "imageExportMode")]
    pub image_export_mode: ImageExportMode,

    /// File format version
    #[serde(rename = "jsonVersion")]
    pub json_version: String,

    /// The default naming convention for level identifiers.
    #[serde(rename = "levelNamePattern")]
    pub level_name_pattern: String,

    /// All levels. The order of this array is only relevant in `LinearHorizontal` and
    /// `linearVertical` world layouts (see `worldLayout` value). Otherwise, you should refer to
    /// the `worldX`,`worldY` coordinates of each Level.
    #[serde(rename = "levels")]
    pub levels: Vec<Level>,

    /// If TRUE, the Json is partially minified (no indentation, nor line breaks, default is
    /// FALSE)
    #[serde(rename = "minifyJson")]
    pub minify_json: bool,

    /// Next Unique integer ID available
    #[serde(rename = "nextUid")]
    pub next_uid: i32,

    /// File naming pattern for exported PNGs
    #[serde(rename = "pngFilePattern")]
    pub png_file_pattern: Option<String>,

    /// Height of the world grid in pixels.
    #[serde(rename = "worldGridHeight")]
    pub world_grid_height: i32,

    /// Width of the world grid in pixels.
    #[serde(rename = "worldGridWidth")]
    pub world_grid_width: i32,

    /// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
    /// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`
    #[serde(rename = "worldLayout")]
    pub world_layout: WorldLayout,
}

/// A structure containing all the definitions of this project
///
/// If you're writing your own LDtk importer, you should probably just ignore *most* stuff in
/// the `defs` section, as it contains data that are mostly important to the editor. To keep
/// you away from the `defs` section and avoid some unnecessary JSON parsing, important data
/// from definitions is often duplicated in fields prefixed with a double underscore (eg.
/// `__identifier` or `__type`).  The 2 only definition types you might need here are
/// **Tilesets** and **Enums**.
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Definitions {
    /// All entities definitions, including their custom fields
    #[serde(rename = "entities")]
    pub entities: Vec<EntityDefinition>,

    /// All internal enums
    #[serde(rename = "enums")]
    pub enums: Vec<EnumDefinition>,

    /// Note: external enums are exactly the same as `enums`, except they have a `relPath` to
    /// point to an external source file.
    #[serde(rename = "externalEnums")]
    pub external_enums: Vec<EnumDefinition>,

    /// All layer definitions
    #[serde(rename = "layers")]
    pub layers: Vec<LayerDefinition>,

    /// All custom fields available to all levels.
    #[serde(rename = "levelFields")]
    pub level_fields: Vec<FieldDefinition>,

    /// All tilesets
    #[serde(rename = "tilesets")]
    pub tilesets: Vec<TilesetDefinition>,
}

#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntityDefinition {
    /// Base entity color
    #[serde(rename = "color")]
    pub color: String,

    /// Array of field definitions
    #[serde(rename = "fieldDefs")]
    pub field_defs: Vec<FieldDefinition>,

    #[serde(rename = "fillOpacity")]
    pub fill_opacity: f32,

    /// Pixel height
    #[serde(rename = "height")]
    pub height: i32,

    #[serde(rename = "hollow")]
    pub hollow: bool,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// Only applies to entities resizable on both X/Y. If TRUE, the entity instance width/height
    /// will keep the same aspect ratio as the definition.
    #[serde(rename = "keepAspectRatio")]
    pub keep_aspect_ratio: bool,

    /// Possible values: `DiscardOldOnes`, `PreventAdding`, `MoveLastOne`
    #[serde(rename = "limitBehavior")]
    pub limit_behavior: LimitBehavior,

    /// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
    /// values: `PerLayer`, `PerLevel`, `PerWorld`
    #[serde(rename = "limitScope")]
    pub limit_scope: LimitScope,

    #[serde(rename = "lineOpacity")]
    pub line_opacity: f32,

    /// Max instances count
    #[serde(rename = "maxCount")]
    pub max_count: i32,

    /// Pivot X coordinate (from 0 to 1.0)
    #[serde(rename = "pivotX")]
    pub pivot_x: f32,

    /// Pivot Y coordinate (from 0 to 1.0)
    #[serde(rename = "pivotY")]
    pub pivot_y: f32,

    /// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
    #[serde(rename = "renderMode")]
    pub render_mode: RenderMode,

    /// If TRUE, the entity instances will be resizable horizontally
    #[serde(rename = "resizableX")]
    pub resizable_x: bool,

    /// If TRUE, the entity instances will be resizable vertically
    #[serde(rename = "resizableY")]
    pub resizable_y: bool,

    /// Display entity name in editor
    #[serde(rename = "showName")]
    pub show_name: bool,

    /// An array of strings that classifies this entity
    #[serde(rename = "tags")]
    pub tags: Vec<String>,

    /// Tile ID used for optional tile display
    #[serde(rename = "tileId")]
    pub tile_id: Option<i32>,

    /// Possible values: `Cover`, `FitInside`, `Repeat`, `Stretch`
    #[serde(rename = "tileRenderMode")]
    pub tile_render_mode: TileRenderMode,

    /// Tileset ID used for optional tile display
    #[serde(rename = "tilesetId")]
    pub tileset_id: Option<i32>,

    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: i32,

    /// Pixel width
    #[serde(rename = "width")]
    pub width: i32,
}

/// This section is mostly only intended for the LDtk editor app itself. You can safely
/// ignore it.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct FieldDefinition {
    /// Human readable value type (eg. `Int`, `Float`, `Point`, etc.). If the field is an array,
    /// this field will look like `Array<...>` (eg. `Array<Int>`, `Array<Point>` etc.)
    #[serde(rename = "__type")]
    pub field_definition_type: String,

    /// Optional list of accepted file extensions for FilePath value type. Includes the dot:
    /// `.ext`
    #[serde(rename = "acceptFileTypes")]
    pub accept_file_types: Option<Vec<String>>,

    /// Array max length
    #[serde(rename = "arrayMaxLength")]
    pub array_max_length: Option<i32>,

    /// Array min length
    #[serde(rename = "arrayMinLength")]
    pub array_min_length: Option<i32>,

    /// TRUE if the value can be null. For arrays, TRUE means it can contain null values
    /// (exception: array of Points can't have null values).
    #[serde(rename = "canBeNull")]
    pub can_be_null: bool,

    /// Default value if selected value is null or invalid.
    #[serde(rename = "defaultOverride")]
    pub default_override: Option<serde_json::Value>,

    #[serde(rename = "editorAlwaysShow")]
    pub editor_always_show: bool,

    #[serde(rename = "editorCutLongValues")]
    pub editor_cut_long_values: bool,

    /// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `Points`,
    /// `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`
    #[serde(rename = "editorDisplayMode")]
    pub editor_display_mode: EditorDisplayMode,

    /// Possible values: `Above`, `Center`, `Beneath`
    #[serde(rename = "editorDisplayPos")]
    pub editor_display_pos: EditorDisplayPos,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// TRUE if the value is an array of multiple values
    #[serde(rename = "isArray")]
    pub is_array: bool,

    /// Max limit for value, if applicable
    #[serde(rename = "max")]
    pub max: Option<f32>,

    /// Min limit for value, if applicable
    #[serde(rename = "min")]
    pub min: Option<f32>,

    /// Optional regular expression that needs to be matched to accept values. Expected format:
    /// `/some_reg_ex/g`, with optional "i" flag.
    #[serde(rename = "regex")]
    pub regex: Option<String>,

    /// Possible values: &lt;`null`&gt;, `LangPython`, `LangRuby`, `LangJS`, `LangLua`, `LangC`,
    /// `LangHaxe`, `LangMarkdown`, `LangJson`, `LangXml`
    #[serde(rename = "textLanguageMode")]
    pub text_language_mode: Option<TextLanguageMode>,

    /// Internal type enum
    #[serde(rename = "type")]
    pub purple_type: Option<serde_json::Value>,

    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: i32,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnumDefinition {
    #[serde(rename = "externalFileChecksum")]
    pub external_file_checksum: Option<String>,

    /// Relative path to the external file providing this Enum
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: Option<String>,

    /// Tileset UID if provided
    #[serde(rename = "iconTilesetUid")]
    pub icon_tileset_uid: Option<i32>,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: i32,

    /// All possible enum values, with their optional Tile infos.
    #[serde(rename = "values")]
    pub values: Vec<EnumValueDefinition>,
}

#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EnumValueDefinition {
    /// An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width,
    /// height ]`
    #[serde(rename = "__tileSrcRect")]
    pub tile_src_rect: Option<Vec<i32>>,

    /// Optional color
    #[serde(rename = "color")]
    pub color: i32,

    /// Enum value
    #[serde(rename = "id")]
    pub id: String,

    /// The optional ID of the tile
    #[serde(rename = "tileId")]
    pub tile_id: Option<i32>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct LayerDefinition {
    /// Type of the layer (*IntGrid, Entities, Tiles or AutoLayer*)
    #[serde(rename = "__type")]
    pub layer_definition_type: String,

    /// Contains all the auto-layer rule definitions.
    #[serde(rename = "autoRuleGroups")]
    pub auto_rule_groups: Vec<AutoLayerRuleGroup>,

    #[serde(rename = "autoSourceLayerDefUid")]
    pub auto_source_layer_def_uid: Option<i32>,

    /// Reference to the Tileset UID being used by this auto-layer rules. WARNING: some layer
    /// *instances* might use a different tileset. So most of the time, you should probably use
    /// the `__tilesetDefUid` value from layer instances.
    #[serde(rename = "autoTilesetDefUid")]
    pub auto_tileset_def_uid: Option<i32>,

    /// Opacity of the layer (0 to 1.0)
    #[serde(rename = "displayOpacity")]
    pub display_opacity: f32,

    /// An array of tags to forbid some Entities in this layer
    #[serde(rename = "excludedTags")]
    pub excluded_tags: Vec<String>,

    /// Width and height of the grid in pixels
    #[serde(rename = "gridSize")]
    pub grid_size: i32,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// An array that defines extra optional info for each IntGrid value. The array is sorted
    /// using value (ascending).
    #[serde(rename = "intGridValues")]
    pub int_grid_values: Vec<IntGridValueDefinition>,

    /// X offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: i32,

    /// Y offset of the layer, in pixels (IMPORTANT: this should be added to the `LayerInstance`
    /// optional offset)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: i32,

    /// An array of tags to filter Entities that can be added to this layer
    #[serde(rename = "requiredTags")]
    pub required_tags: Vec<String>,

    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    #[serde(rename = "tilePivotX")]
    pub tile_pivot_x: f32,

    /// If the tiles are smaller or larger than the layer grid, the pivot value will be used to
    /// position the tile relatively its grid cell.
    #[serde(rename = "tilePivotY")]
    pub tile_pivot_y: f32,

    /// Reference to the Tileset UID being used by this Tile layer. WARNING: some layer
    /// *instances* might use a different tileset. So most of the time, you should probably use
    /// the `__tilesetDefUid` value from layer instances.
    #[serde(rename = "tilesetDefUid")]
    pub tileset_def_uid: Option<i32>,

    /// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
    /// `AutoLayer`
    #[serde(rename = "type")]
    pub purple_type: Type,

    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: i32,
}

#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct AutoLayerRuleGroup {
    #[serde(rename = "active")]
    pub active: bool,

    #[serde(rename = "collapsed")]
    pub collapsed: bool,

    #[serde(rename = "isOptional")]
    pub is_optional: bool,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "rules")]
    pub rules: Vec<AutoLayerRuleDefinition>,

    #[serde(rename = "uid")]
    pub uid: i32,
}

/// This complex section isn't meant to be used by game devs at all, as these rules are
/// completely resolved internally by the editor before any saving. You should just ignore
/// this part.
#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct AutoLayerRuleDefinition {
    /// If FALSE, the rule effect isn't applied, and no tiles are generated.
    #[serde(rename = "active")]
    pub active: bool,

    /// When TRUE, the rule will prevent other rules to be applied in the same cell if it matches
    /// (TRUE by default).
    #[serde(rename = "breakOnMatch")]
    pub break_on_match: bool,

    /// Chances for this rule to be applied (0 to 1)
    #[serde(rename = "chance")]
    pub chance: f32,

    /// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
    #[serde(rename = "checker")]
    pub checker: Checker,

    /// If TRUE, allow rule to be matched by flipping its pattern horizontally
    #[serde(rename = "flipX")]
    pub flip_x: bool,

    /// If TRUE, allow rule to be matched by flipping its pattern vertically
    #[serde(rename = "flipY")]
    pub flip_y: bool,

    /// Default IntGrid value when checking cells outside of level bounds
    #[serde(rename = "outOfBoundsValue")]
    pub out_of_bounds_value: Option<i32>,

    /// Rule pattern (size x size)
    #[serde(rename = "pattern")]
    pub pattern: Vec<i32>,

    /// If TRUE, enable Perlin filtering to only apply rule on specific random area
    #[serde(rename = "perlinActive")]
    pub perlin_active: bool,

    #[serde(rename = "perlinOctaves")]
    pub perlin_octaves: f32,

    #[serde(rename = "perlinScale")]
    pub perlin_scale: f32,

    #[serde(rename = "perlinSeed")]
    pub perlin_seed: f32,

    /// X pivot of a tile stamp (0-1)
    #[serde(rename = "pivotX")]
    pub pivot_x: f32,

    /// Y pivot of a tile stamp (0-1)
    #[serde(rename = "pivotY")]
    pub pivot_y: f32,

    /// Pattern width & height. Should only be 1,3,5 or 7.
    #[serde(rename = "size")]
    pub size: i32,

    /// Array of all the tile IDs. They are used randomly or as stamps, based on `tileMode` value.
    #[serde(rename = "tileIds")]
    pub tile_ids: Vec<i32>,

    /// Defines how tileIds array is used Possible values: `Single`, `Stamp`
    #[serde(rename = "tileMode")]
    pub tile_mode: TileMode,

    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: i32,

    /// X cell coord modulo
    #[serde(rename = "xModulo")]
    pub x_modulo: i32,

    /// Y cell coord modulo
    #[serde(rename = "yModulo")]
    pub y_modulo: i32,
}

/// IntGrid value definition
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct IntGridValueDefinition {
    #[serde(rename = "color")]
    pub color: String,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: Option<String>,

    /// The IntGrid value itself
    #[serde(rename = "value")]
    pub value: i32,
}

/// The `Tileset` definition is the most important part among project definitions. It
/// contains some extra informations about each integrated tileset. If you only had to parse
/// one definition section, that would be the one.
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct TilesetDefinition {
    /// Grid-based height
    #[serde(rename = "__cHei")]
    pub c_hei: i32,

    /// Grid-based width
    #[serde(rename = "__cWid")]
    pub c_wid: i32,

    /// The following data is used internally for various optimizations. It's always synced with
    /// source image changes.
    #[serde(rename = "cachedPixelData")]
    pub cached_pixel_data: Option<HashMap<String, Option<serde_json::Value>>>,

    /// An array of custom tile metadata
    #[serde(rename = "customData")]
    pub custom_data: Vec<HashMap<String, Option<serde_json::Value>>>,

    /// Tileset tags using Enum values specified by `tagsSourceEnumId`. This array contains 1
    /// element per Enum value, which contains an array of all Tile IDs that are tagged with it.
    #[serde(rename = "enumTags")]
    pub enum_tags: Vec<HashMap<String, Option<serde_json::Value>>>,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// Distance in pixels from image borders
    #[serde(rename = "padding")]
    pub padding: i32,

    /// Image height in pixels
    #[serde(rename = "pxHei")]
    pub px_hei: i32,

    /// Image width in pixels
    #[serde(rename = "pxWid")]
    pub px_wid: i32,

    /// Path to the source file, relative to the current project JSON file
    #[serde(rename = "relPath")]
    pub rel_path: String,

    /// Array of group of tiles selections, only meant to be used in the editor
    #[serde(rename = "savedSelections")]
    pub saved_selections: Vec<HashMap<String, Option<serde_json::Value>>>,

    /// Space in pixels between all tiles
    #[serde(rename = "spacing")]
    pub spacing: i32,

    /// Optional Enum definition UID used for this tileset meta-data
    #[serde(rename = "tagsSourceEnumUid")]
    pub tags_source_enum_uid: Option<i32>,

    #[serde(rename = "tileGridSize")]
    pub tile_grid_size: i32,

    /// Unique Intidentifier
    #[serde(rename = "uid")]
    pub uid: i32,
}

/// This section contains all the level data. It can be found in 2 distinct forms, depending
/// on Project current settings:  - If "*Separate level files*" is **disabled** (default):
/// full level data is *embedded* inside the main Project JSON file, - If "*Separate level
/// files*" is **enabled**: level data is stored in *separate* standalone `.ldtkl` files (one
/// per level). In this case, the main Project JSON file will still contain most level data,
/// except heavy sections, like the `layerInstances` array (which will be null). The
/// `externalRelPath` string points to the `ldtkl` file.  A `ldtkl` file is just a JSON file
/// containing exactly what is described below.
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct Level {
    /// Background color of the level (same as `bgColor`, except the default value is
    /// automatically used here if its value is `null`)
    #[serde(rename = "__bgColor")]
    pub bg_color: String,

    /// Position informations of the background image, if there is one.
    #[serde(rename = "__bgPos")]
    pub bg_pos: Option<LevelBackgroundPosition>,

    /// An array listing all other levels touching this one on the world map. In "linear" world
    /// layouts, this array is populated with previous/next levels in array, and `dir` depends on
    /// the linear horizontal/vertical layout.
    #[serde(rename = "__neighbours")]
    pub neighbours: Vec<NeighbourLevel>,

    /// Background color of the level. If `null`, the project `defaultLevelBgColor` should be
    /// used.
    #[serde(rename = "bgColor")]
    pub level_bg_color: Option<String>,

    /// Background image X pivot (0-1)
    #[serde(rename = "bgPivotX")]
    pub bg_pivot_x: f32,

    /// Background image Y pivot (0-1)
    #[serde(rename = "bgPivotY")]
    pub bg_pivot_y: f32,

    /// An enum defining the way the background image (if any) is positioned on the level. See
    /// `__bgPos` for resulting position info. Possible values: &lt;`null`&gt;, `Unscaled`,
    /// `Contain`, `Cover`, `CoverDirty`
    #[serde(rename = "bgPos")]
    pub level_bg_pos: Option<BgPos>,

    /// The *optional* relative path to the level background image.
    #[serde(rename = "bgRelPath")]
    pub bg_rel_path: Option<String>,

    /// This value is not null if the project option "*Save levels separately*" is enabled. In
    /// this case, this **relative** path points to the level Json file.
    #[serde(rename = "externalRelPath")]
    pub external_rel_path: Option<String>,

    /// An array containing this level custom field values.
    #[serde(rename = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,

    /// Unique String identifier
    #[serde(rename = "identifier")]
    pub identifier: String,

    /// An array containing all Layer instances. **IMPORTANT**: if the project option "*Save
    /// levels separately*" is enabled, this field will be `null`.<br/>  This array is **sorted
    /// in display order**: the 1st layer is the top-most and the last is behind.
    #[serde(rename = "layerInstances")]
    pub layer_instances: Option<Vec<LayerInstance>>,

    /// Height of the level in pixels
    #[serde(rename = "pxHei")]
    pub px_hei: i32,

    /// Width of the level in pixels
    #[serde(rename = "pxWid")]
    pub px_wid: i32,

    /// Unique Int identifier
    #[serde(rename = "uid")]
    pub uid: i32,

    /// If TRUE, the level identifier will always automatically use the naming pattern as defined
    /// in `Project.levelNamePattern`. Becomes FALSE if the identifier is manually modified by
    /// user.
    #[serde(rename = "useAutoIdentifier")]
    pub use_auto_identifier: bool,

    /// World X coordinate in pixels
    #[serde(rename = "worldX")]
    pub world_x: i32,

    /// World Y coordinate in pixels
    #[serde(rename = "worldY")]
    pub world_y: i32,
}

/// Level background image position info
#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct LevelBackgroundPosition {
    /// An array of 4 float values describing the cropped sub-rectangle of the displayed
    /// background image. This cropping happens when original is larger than the level bounds.
    /// Array format: `[ cropX, cropY, cropWidth, cropHeight ]`
    #[serde(rename = "cropRect")]
    pub crop_rect: Vec<f32>,

    /// An array containing the `[scaleX,scaleY]` values of the **cropped** background image,
    /// depending on `bgPos` option.
    #[serde(rename = "scale")]
    pub scale: Vec<f32>,

    /// An array containing the `[x,y]` pixel coordinates of the top-left corner of the
    /// **cropped** background image, depending on `bgPos` option.
    #[serde(rename = "topLeftPx")]
    pub top_left_px: Vec<i32>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
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
    #[serde(rename = "autoLayerTiles")]
    pub auto_layer_tiles: Vec<TileInstance>,

    #[serde(rename = "entityInstances")]
    pub entity_instances: Vec<EntityInstance>,

    #[serde(rename = "gridTiles")]
    pub grid_tiles: Vec<TileInstance>,

    /// **WARNING**: this deprecated value will be *removed* completely on version 0.10.0+
    /// Replaced by: `intGridCsv`
    #[serde(rename = "intGrid")]
    pub int_grid: Option<Vec<IntGridValueInstance>>,

    /// A list of all values in the IntGrid layer, stored from left to right, and top to bottom
    /// (ie. first row from left to right, followed by second row, etc). `0` means "empty cell"
    /// and IntGrid values start at 1. This array size is `__cWid` x `__cHei` cells.
    #[serde(rename = "intGridCsv")]
    pub int_grid_csv: Vec<i32>,

    /// Reference the Layer definition UID
    #[serde(rename = "layerDefUid")]
    pub layer_def_uid: i32,

    /// Reference to the UID of the level containing this layer instance
    #[serde(rename = "levelId")]
    pub level_id: i32,

    /// An Array containing the UIDs of optional rules that were enabled in this specific layer
    /// instance.
    #[serde(rename = "optionalRules")]
    pub optional_rules: Vec<i32>,

    /// This layer can use another tileset by overriding the tileset UID here.
    #[serde(rename = "overrideTilesetUid")]
    pub override_tileset_uid: Option<i32>,

    /// X offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, see `__pxTotalOffsetX`)
    #[serde(rename = "pxOffsetX")]
    pub px_offset_x: i32,

    /// Y offset in pixels to render this layer, usually 0 (IMPORTANT: this should be added to
    /// the `LayerDef` optional offset, see `__pxTotalOffsetY`)
    #[serde(rename = "pxOffsetY")]
    pub px_offset_y: i32,

    /// Random seed used for Auto-Layers rendering
    #[serde(rename = "seed")]
    pub seed: i32,

    /// Layer instance visibility
    #[serde(rename = "visible")]
    pub visible: bool,
}

/// This structure represents a single tile from a given Tileset.
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct TileInstance {
    /// Internal data used by the editor.<br/>  For auto-layer tiles: `[ruleId, coordId]`.<br/>
    /// For tile-layer tiles: `[coordId]`.
    #[serde(rename = "d")]
    pub d: Vec<i32>,

    /// "Flip bits", a 2-bits integer to represent the mirror transformations of the tile.<br/>
    /// - Bit 0 = X flip<br/>   - Bit 1 = Y flip<br/>   Examples: f=0 (no flip), f=1 (X flip
    /// only), f=2 (Y flip only), f=3 (both flips)
    #[serde(rename = "f")]
    pub f: i32,

    /// Pixel coordinates of the tile in the **layer** (`[x,y]` format). Don't forget optional
    /// layer offsets, if they exist!
    #[serde(rename = "px")]
    pub px: Vec<i32>,

    /// Pixel coordinates of the tile in the **tileset** (`[x,y]` format)
    #[serde(rename = "src")]
    pub src: Vec<i32>,

    /// The *Tile ID* in the corresponding tileset.
    #[serde(rename = "t")]
    pub t: i32,
}

#[derive(PartialEq, Debug, Default, Clone, Serialize, Deserialize, bevy::prelude::Component)]
pub struct EntityInstance {
    /// Grid-based coordinates (`[x,y]` format)
    #[serde(rename = "__grid")]
    pub grid: Vec<i32>,

    /// Entity definition identifier
    #[serde(rename = "__identifier")]
    pub identifier: String,

    /// Pivot coordinates  (`[x,y]` format, values are from 0 to 1) of the Entity
    #[serde(rename = "__pivot")]
    pub pivot: Vec<f32>,

    /// Optional Tile used to display this entity (it could either be the default Entity tile, or
    /// some tile provided by a field value, like an Enum).
    #[serde(rename = "__tile")]
    pub tile: Option<EntityInstanceTile>,

    /// Reference of the **Entity definition** UID
    #[serde(rename = "defUid")]
    pub def_uid: i32,

    /// An array of all custom fields and their values.
    #[serde(rename = "fieldInstances")]
    pub field_instances: Vec<FieldInstance>,

    /// Entity height in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    #[serde(rename = "height")]
    pub height: i32,

    /// Pixel coordinates (`[x,y]` format) in current level coordinate space. Don't forget
    /// optional layer offsets, if they exist!
    #[serde(rename = "px")]
    pub px: Vec<i32>,

    /// Entity width in pixels. For non-resizable entities, it will be the same as Entity
    /// definition.
    #[serde(rename = "width")]
    pub width: i32,
}

/// Tile data in an Entity instance
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct EntityInstanceTile {
    /// An array of 4 Int values that refers to the tile in the tileset image: `[ x, y, width,
    /// height ]`
    #[serde(rename = "srcRect")]
    pub src_rect: Vec<i32>,

    /// Tileset ID
    #[serde(rename = "tilesetUid")]
    pub tileset_uid: i32,
}

/// IntGrid value instance
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct IntGridValueInstance {
    /// Coordinate ID in the layer grid
    #[serde(rename = "coordId")]
    pub coord_id: i32,

    /// IntGrid value
    #[serde(rename = "v")]
    pub v: i32,
}

/// Nearby level info
#[derive(Eq, PartialEq, Debug, Default, Clone, Serialize, Deserialize)]
pub struct NeighbourLevel {
    /// A single lowercase character tipping on the level location (`n`orth, `s`outh, `w`est,
    /// `e`ast).
    #[serde(rename = "dir")]
    pub dir: String,

    #[serde(rename = "levelUid")]
    pub level_uid: i32,
}

/// Possible values: `Hidden`, `ValueOnly`, `NameAndValue`, `EntityTile`, `Points`,
/// `PointStar`, `PointPath`, `PointPathLoop`, `RadiusPx`, `RadiusGrid`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum EditorDisplayMode {
    #[serde(rename = "EntityTile")]
    EntityTile,

    #[serde(rename = "Hidden")]
    Hidden,

    #[serde(rename = "NameAndValue")]
    NameAndValue,

    #[serde(rename = "PointPath")]
    PointPath,

    #[serde(rename = "PointPathLoop")]
    PointPathLoop,

    #[serde(rename = "PointStar")]
    PointStar,

    #[serde(rename = "Points")]
    Points,

    #[serde(rename = "RadiusGrid")]
    RadiusGrid,

    #[serde(rename = "RadiusPx")]
    RadiusPx,

    #[serde(rename = "ValueOnly")]
    ValueOnly,
}

/// Possible values: `Above`, `Center`, `Beneath`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum EditorDisplayPos {
    #[serde(rename = "Above")]
    Above,

    #[serde(rename = "Beneath")]
    Beneath,

    #[serde(rename = "Center")]
    Center,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TextLanguageMode {
    #[serde(rename = "LangC")]
    LangC,

    #[serde(rename = "LangHaxe")]
    LangHaxe,

    #[serde(rename = "LangJS")]
    LangJs,

    #[serde(rename = "LangJson")]
    LangJson,

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
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LimitBehavior {
    #[serde(rename = "DiscardOldOnes")]
    DiscardOldOnes,

    #[serde(rename = "MoveLastOne")]
    MoveLastOne,

    #[serde(rename = "PreventAdding")]
    PreventAdding,
}

impl Default for LimitBehavior {
    fn default() -> Self {
        Self::MoveLastOne
    }
}

/// If TRUE, the maxCount is a "per world" limit, if FALSE, it's a "per level". Possible
/// values: `PerLayer`, `PerLevel`, `PerWorld`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum LimitScope {
    #[serde(rename = "PerLayer")]
    PerLayer,

    #[serde(rename = "PerLevel")]
    PerLevel,

    #[serde(rename = "PerWorld")]
    PerWorld,
}

impl Default for LimitScope {
    fn default() -> Self {
        Self::PerLevel
    }
}

/// Possible values: `Rectangle`, `Ellipse`, `Tile`, `Cross`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum RenderMode {
    #[serde(rename = "Cross")]
    Cross,

    #[serde(rename = "Ellipse")]
    Ellipse,

    #[serde(rename = "Rectangle")]
    Rectangle,

    #[serde(rename = "Tile")]
    Tile,
}

impl Default for RenderMode {
    fn default() -> Self {
        Self::Rectangle
    }
}

/// Possible values: `Cover`, `FitInside`, `Repeat`, `Stretch`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TileRenderMode {
    #[serde(rename = "Cover")]
    Cover,

    #[serde(rename = "FitInside")]
    FitInside,

    #[serde(rename = "Repeat")]
    Repeat,

    #[serde(rename = "Stretch")]
    Stretch,
}

impl Default for TileRenderMode {
    fn default() -> Self {
        Self::FitInside
    }
}

/// Checker mode Possible values: `None`, `Horizontal`, `Vertical`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Checker {
    #[serde(rename = "Horizontal")]
    Horizontal,

    #[serde(rename = "None")]
    None,

    #[serde(rename = "Vertical")]
    Vertical,
}

/// Defines how tileIds array is used Possible values: `Single`, `Stamp`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TileMode {
    #[serde(rename = "Single")]
    Single,

    #[serde(rename = "Stamp")]
    Stamp,
}

/// Type of the layer as Haxe Enum Possible values: `IntGrid`, `Entities`, `Tiles`,
/// `AutoLayer`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Type {
    #[serde(rename = "AutoLayer")]
    AutoLayer,

    #[serde(rename = "Entities")]
    Entities,

    #[serde(rename = "IntGrid")]
    IntGrid,

    #[serde(rename = "Tiles")]
    Tiles,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Flag {
    #[serde(rename = "DiscardPreCsvIntGrid")]
    DiscardPreCsvIntGrid,

    #[serde(rename = "IgnoreBackupSuggest")]
    IgnoreBackupSuggest,
}

/// "Image export" option when saving project. Possible values: `None`, `OneImagePerLayer`,
/// `OneImagePerLevel`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum ImageExportMode {
    #[serde(rename = "None")]
    None,

    #[serde(rename = "OneImagePerLayer")]
    OneImagePerLayer,

    #[serde(rename = "OneImagePerLevel")]
    OneImagePerLevel,
}

#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum BgPos {
    #[serde(rename = "Contain")]
    Contain,

    #[serde(rename = "Cover")]
    Cover,

    #[serde(rename = "CoverDirty")]
    CoverDirty,

    #[serde(rename = "Unscaled")]
    Unscaled,
}

/// An enum that describes how levels are organized in this project (ie. linearly or in a 2D
/// space). Possible values: `Free`, `GridVania`, `LinearHorizontal`, `LinearVertical`
#[derive(Eq, PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum WorldLayout {
    #[serde(rename = "Free")]
    Free,

    #[serde(rename = "GridVania")]
    GridVania,

    #[serde(rename = "LinearHorizontal")]
    LinearHorizontal,

    #[serde(rename = "LinearVertical")]
    LinearVertical,
}
