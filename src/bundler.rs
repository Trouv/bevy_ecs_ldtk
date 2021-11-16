use crate::ldtk::EntityInstance;
use bevy::prelude::*;
use std::{collections::HashMap, marker::PhantomData};

pub trait Bundler: Bundle {
    fn bundle(entity_instance: EntityInstance) -> Self;
}

pub trait AddBundle {
    fn add_bundle<B: Bundler>(&mut self, identifier: &str) -> &mut App;
}

impl AddBundle for App {
    fn add_bundle<B: Bundler>(&mut self, identifier: &str) -> &mut App {
        let new_entry = Box::new(BundleEntry::<B> {
            bundler: PhantomData,
        });
        match self.world.get_non_send_resource_mut::<BundleMap>() {
            Some(entries) => {
                entries.insert(identifier.to_string(), new_entry);
            }
            None => {
                let bundle_map = BundleMap::new();
                bundle_map.insert(identifier.to_string(), new_entry);
                self.world.insert_non_send::<BundleMap>(bundle_map);
            }
        }
        self
    }
}

pub type BundleMap = HashMap<String, Box<dyn BundleEntryTrait>>;

pub struct BundleEntry<B: Bundler> {
    bundler: PhantomData<B>,
}

pub trait BundleEntryTrait {
    fn bundle(&self, entity_instance: EntityInstance);
}

impl<B: Bundler> BundleEntryTrait for BundleEntry<B> {
    fn bundle(&self, entity_instance: EntityInstance) {
        B::bundle(entity_instance);
    }
}
