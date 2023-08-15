/// Indices pointing to the location of a level in an [`LdtkProject`] or [`LdtkJson`].
///
/// This type supports multi-world projects by storing an optional `world` index.
/// If this is present, the level index is used within that world.
/// If not, the level index is used in the project root's level collection.
///
/// [`LdtkProject`]: crate::assets::LdtkProject
/// [`LdtkJson`]: crate::ldtk::LdtkJson
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct LevelIndices {
    /// The index of the world the level belongs to, if the project is multi-world.
    pub world: Option<usize>,
    /// The index of the level, either within a world or in the root of the project.
    pub level: usize,
}
