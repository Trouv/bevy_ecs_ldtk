use bevy::{
    ecs::system::{EntityCommands, SystemParam, SystemParamFetch, SystemParamItem},
    prelude::{Bundle, ParamSet, SystemParamFunction},
};

use super::{SpawnFunction, SpawnSystemFunction, Spawner, SpawnerType};

pub trait SpawnerCollection<In: SpawnerType> {
    type Param<'w, 's>: SystemParam;

    const SIZE: usize;

    fn spawn(
        &mut self,
        spawner_index: usize,
        entry_index: usize,
        input: In::Input<'_>,
        commands: &mut EntityCommands,
        param_value: <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<'_, '_>>::Item,
    );
}

impl<In: SpawnerType, A: SpawnerCollection<In>, B: SpawnerCollection<In>> SpawnerCollection<In>
    for (A, B)
{
    type Param<'w, 's> = ParamSet<
        'w,
        's,
        (
            SystemParamItem<'w, 's, A::Param<'w, 's>>,
            SystemParamItem<'w, 's, B::Param<'w, 's>>,
        ),
    >;

    const SIZE: usize = A::SIZE + B::SIZE;

    fn spawn(
        &mut self,
        spawner_index: usize,
        entry_index: usize,
        input: In::Input<'_>,
        commands: &mut EntityCommands,
        mut param_value: <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    ) {
        if spawner_index < A::SIZE {
            self.0.spawn(
                spawner_index,
                entry_index,
                input,
                commands,
                param_value.p0(),
            );
        } else {
            self.1.spawn(
                spawner_index - A::SIZE,
                entry_index,
                input,
                commands,
                param_value.p1(),
            );
        }
    }
}

impl<F: SpawnFunction> SpawnerCollection<F::SpawnerType> for Spawner<F> {
    type Param<'w, 's> = F::Param<'w, 's>;

    const SIZE: usize = 1;

    fn spawn(
        &mut self,
        _: usize,
        entry_index: usize,
        input: <F::SpawnerType as SpawnerType>::Input<'_>,
        commands: &mut EntityCommands,
        param_value: <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<
            '_,
            '_,
        >>::Item,
    ) {
        self.system.spawn(
            input,
            commands,
            self.casters[entry_index].as_ref(),
            param_value,
        );
    }
}

pub struct EmptyCollection;

impl<
        Type: SpawnerType,
        Out: Bundle,
        Param: SystemParam,
        Marker,
        F: for<'a> SystemParamFunction<Type::Input<'a>, Out, Param, Marker>,
    > SpawnerCollection<Type> for SpawnSystemFunction<Type, Out, Param, Marker, F>
{
    type Param<'w, 's> = Param;

    const SIZE: usize = 1;

    fn spawn(
        &mut self,
        _: usize,
        _: usize,
        input: <Type as SpawnerType>::Input<'_>,
        commands: &mut EntityCommands,
        param_value: <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<'_, '_>>::Item,
    ) {
        commands.insert(self.get_mut().run(input, param_value));
    }
}

impl<Type: SpawnerType> SpawnerCollection<Type> for EmptyCollection {
    type Param<'w, 's> = ();

    const SIZE: usize = 0;

    fn spawn(
        &mut self,
        _: usize,
        _: usize,
        _: Type::Input<'_>,
        _: &mut EntityCommands,
        _: <<Self::Param<'_, '_> as SystemParam>::Fetch as SystemParamFetch<'_, '_>>::Item,
    ) {
    }
}
