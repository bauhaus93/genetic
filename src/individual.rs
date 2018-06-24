use std::fmt::Display;

pub trait Individual: Default + Clone + Ord + Display {
    fn crossover(parents: (&Self, &Self)) -> Self;
    fn mutate(&mut self);
}
