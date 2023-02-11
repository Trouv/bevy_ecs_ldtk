use bevy::prelude::Bundle;

mod caster;
mod collection;
mod entity;
mod function;
mod int_cell;
mod registry;

pub use self::{caster::*, collection::*, entity::*, function::*, int_cell::*, registry::*};

pub trait SpawnerType {
    type Input<'a>;
    type Selector;
}

pub struct Spawner<F: SpawnFunction> {
    system: F,
    selectors: Vec<<F::SpawnerType as SpawnerType>::Selector>,
    casters: Vec<Box<dyn Caster<F>>>,
}

impl<F: SpawnFunction> Spawner<F> {
    fn new(function: F) -> Self {
        Self {
            system: function,
            selectors: Vec::new(),
            casters: Vec::new(),
        }
    }

    fn add<T: Bundle>(mut self, selector: <F::SpawnerType as SpawnerType>::Selector) -> Self
    where
        for<'a> F::Input<'a>: Into<T>,
    {
        self.selectors.push(selector);
        self.casters.push(Box::<PhantomCaster<F, T>>::default());
        self
    }
}

pub struct SpawnSelector {
    spawner_index: usize,
    entry_index: usize,
}
