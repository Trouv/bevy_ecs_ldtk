use std::marker::PhantomData;

use bevy::{ecs::system::EntityCommands, prelude::Bundle};

use super::SpawnFunction;

pub trait Caster<S: SpawnFunction>: Send + Sync + 'static {
    fn cast_and_insert(&self, input: S::Input<'_>, commands: &mut EntityCommands);
}

pub struct PhantomCaster<S: SpawnFunction, T>(PhantomData<(S, T)>);

impl<S: SpawnFunction, T> Default for PhantomCaster<S, T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<S: SpawnFunction, T> Caster<S> for PhantomCaster<S, T>
where
    T: Bundle,
    for<'a> S::Input<'a>: Into<T>,
{
    fn cast_and_insert(&self, value: S::Input<'_>, commands: &mut EntityCommands) {
        commands.insert(value.into());
    }
}
