extern crate genetic;

use genetic::PopulationBuilder;
use genetic::example::challenge::Challenge;

fn main() {
    let mut pop = PopulationBuilder::<Challenge>::new()
                .set_size(20)
                .set_mutation_chance(0.25)
                .finish();


    for _ in 0..100 {
        pop.cycle(100000);
        pop.print_best();
    }

}
