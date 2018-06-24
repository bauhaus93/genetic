use std::iter;
use rand;
use rand::Rng;
use rand::distributions::{ Sample, Normal };

use individual::Individual;

pub struct Population<T: Individual> {
    max_size: usize,
    mutation_chance: f32,
    population: Vec<T>,
}

impl<T: Individual> Population<T> {

    pub fn new(max_size: usize, mutation_chance: f32) -> Population<T> {
        let pop = Population {
            max_size: max_size,
            mutation_chance: mutation_chance,
            population: iter::repeat(T::default()).take(max_size).collect(),
        };
        pop
    }

    pub fn print_best(&self) {
        println!("Best: {}", self.population[0]);
    }

    pub fn cycle(&mut self, count: u32) {
        for _ in 0..count {
            self.cycle_once()
        }
    }

    pub fn cycle_once(&mut self) {
        let child = {
            let parents = self.select_parents();
            let mut child = T::crossover(parents);
            if rand::thread_rng().gen_range::<f32>(0.0, 1.0) < self.mutation_chance {
                child.mutate();
            }
            child
        };
        self.population.push(child);
        self.population.sort_unstable();
        self.population.reverse();
        self.population.pop();
    }

    fn select_parents(&self) -> (&T, &T) {
        let mut dist = Normal::new(0.0, f64::sqrt(self.max_size as f64 / 3.0));
        let mut indices = ((dist.sample(&mut rand::thread_rng())).abs() as usize % self.max_size,
                      (dist.sample(&mut rand::thread_rng())).abs() as usize % self.max_size);

        if indices.0 == indices.1 {
            indices.1 = (indices.0 + 1) % self.max_size;
        }

        (&self.population[indices.0], &self.population[indices.1])
    }
}
