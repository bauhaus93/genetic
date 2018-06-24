use std::cmp::Ordering;
use std::fmt;
use std::iter;
use rand;
use rand::Rng;

use individual::Individual;

fn calculate_fitness(text: &str) -> i32 {
    static TARGET: &'static str = "00000000000000000000";
    text.chars()
        .zip(TARGET.chars())
        .filter(| (a, b) | a == b)
        .count() as i32
}

#[derive(Eq, Clone)]
pub struct Challenge {
    text: String,
    fitness: i32
}

impl Challenge {

    pub fn new(text: String) -> Challenge {
        let fitness = calculate_fitness(text.as_str());
        let c = Challenge {
            text: text,
            fitness: fitness
        };
        c
    }
}

impl Individual for Challenge {

    fn crossover(parents: (&Self, &Self)) -> Self {
        assert!(parents.0.text.len() == parents.1.text.len());
        assert!(parents.0.text.len() > 2);
        let split_index = rand::thread_rng().gen_range::<usize>(1, parents.0.text.len() - 1);
        let child_text = parents.0.text.chars()
                                  .take(split_index)
                                  .chain(parents.1.text.chars()
                                                  .skip(split_index))
                                  .collect::<String>();
        Challenge::new(child_text)
    }

    fn mutate(&mut self) {
        let iter_mut = iter::repeat(rand::thread_rng().gen_range::<f32>(0.0, 1.0));
        let text: String = self.text.chars()
                 .zip(iter_mut)
                 .map(| (c, v) | if v < 0.1 { c }
                                 else { rand::thread_rng()
                                        .gen_ascii_chars()
                                        .next().unwrap() } )
                 .collect();
        self.text = text;
        self.fitness = calculate_fitness(&self.text);
    }
}

impl Default for Challenge {

    fn default() -> Self {
        let text =  rand::thread_rng()
                    .gen_ascii_chars()
                    .take(20)
                    .collect::<String>();
        Challenge::new(text)
    }
}

impl PartialOrd for Challenge {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        Some(self.cmp(rhs))
    }
}

impl Ord for Challenge {
    fn cmp(&self, rhs: &Self) -> Ordering {
        match self.fitness.cmp(&rhs.fitness) {
            Ordering::Equal => self.text.cmp(&rhs.text),
            other => other
        }
    }
}

impl PartialEq for Challenge {

    fn eq(&self, rhs: &Self) -> bool {
        if self.fitness == rhs.fitness {
            self.text == rhs.text
        } else {
            false
        }
    }
}

impl fmt::Display for Challenge {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "fitness = {}, text = {}", self.fitness, self.text)
    }
}
