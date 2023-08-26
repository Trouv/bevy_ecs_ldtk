use std::collections::HashMap;

use crate::ldtk::LdtkJson;
use bevy::prelude::*;

pub trait LdtkProjectGetters {
    fn data(&self) -> &LdtkJson;

    fn tileset_map(&self) -> &HashMap<i32, Handle<Image>>;

    fn int_grid_image_handle(&self) -> &Option<Handle<Image>>;
}
