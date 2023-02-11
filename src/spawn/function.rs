use std::marker::PhantomData;

use bevy::{
    ecs::system::{EntityCommands, SystemParam, SystemParamFetch},
    prelude::SystemParamFunction,
};

use super::{Caster, Spawner, SpawnerType};

pub trait SpawnFunction: Send + Sync + 'static {
    type Param<'w, 's>: SystemParam;
    type SpawnerType: SpawnerType;
    type Input<'a>;

    fn spawn(
        &mut self,
        input: <Self::SpawnerType as SpawnerType>::Input<'_>,
        commands: &mut EntityCommands,
        caster: &dyn Caster<Self>,
        param_value: <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    );

    fn create(self) -> Spawner<Self>
    where
        Self: Sized,
    {
        Spawner::new(self)
    }
}

pub struct SpawnSystemFunction<
    Type: SpawnerType,
    Out,
    Param: SystemParam,
    Marker,
    F: for<'a> SystemParamFunction<Type::Input<'a>, Out, Param, Marker>,
>(F, PhantomData<fn(Type, Out, Param, Marker)>);

impl<
        Type: SpawnerType,
        Out,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<Type::Input<'a>, Out, Param, Marker>,
    > SpawnSystemFunction<Type, Out, Param, Marker, F>
{
    pub fn new(f: F) -> Self {
        Self(f, PhantomData)
    }

    pub fn get_mut(&mut self) -> &mut F {
        &mut self.0
    }
}
