use std::marker::PhantomData;


use individual::Individual;
use population::Population;

pub struct PopulationBuilder<T: Individual> {
    max_size: usize,
    mutation_chance: f32,
    phantom: PhantomData<T>
}

impl<T: Individual> PopulationBuilder<T> {
    pub fn new() -> PopulationBuilder<T> {
        let builder = PopulationBuilder {
            max_size: 20,
            mutation_chance: 0.2,
            phantom: PhantomData
        };
        builder
    }

    pub fn set_size(mut self, size: usize) -> Self {
        self.max_size = size;
        self
    }

    pub fn set_mutation_chance(mut self, chance: f32) -> Self {
        self.mutation_chance = chance;
        self
    }

    pub fn finish(self) -> Population<T> {
        Population::<T>::new(self.max_size, self.mutation_chance)
    }
}
