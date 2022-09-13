use crate::population::Phenotype;

/// Represents a rocket that is trying to find the goal.
#[derive(Debug)]
pub struct Rocket {

}

impl Phenotype for Rocket {
    fn update(&mut self) -> bool {
        false
    }

    fn calculate_fitness(&self) -> f32 {
        0.0
    }

    fn mutate(&mut self) {
        
    }

    fn crossover(&self, _other: &Self) -> Self {
        Self {}
    }
}
