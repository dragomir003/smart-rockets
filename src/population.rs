///! This module holds everything that describes genetic algorithm

/// Trait that encapsulates behaviour one object or ```phenotype```
pub trait Phenotype {

    /// Updates object's state. Represents one action a phenotype takes while
    /// it is alive.
    /// # Returns
    /// - ```true``` if object reached the end of it's life
    /// - ```false``` if object is not done living
    fn update(&mut self) -> bool;

    /// Fitness calculation function.
    /// The higher the return value is, the better did the object do.
    fn calculate_fitness(&self) -> f32;

    /// Mutates object's genome
    fn mutate(&mut self);

    /// Combines genomes of 2 separate objects
    fn crossover(&self, other: &Self) -> Self;
}

/// Wrapper struct that provides an interface to population.
#[derive(Debug)]
pub struct Population<T>(Vec<T>)
    where T: Phenotype;

impl<T: Phenotype> Population<T> {

    /// Gets underlying data. Used for looping.
    pub fn get(&self) -> &Vec<T> {
        &self.0
    }

    /// Updates all objects, using Phenotype::update function
    /// # Returns
    /// - ```true``` if all objects' update functions returned true
    /// - ```false``` if all objects' update functions returned false
    /// # Panics
    /// This function will panic if some object's finish their life, and some don't.
    /// That scenario is not supposed to happen because all objects have equal life times.
    pub fn update(&mut self) -> bool {
        let bools: Vec<bool> = (&mut self.0).iter_mut().map(|phen: &mut T| phen.update()).collect();

        if bools.iter().all(|b| *b) {
            true
        } else if bools.iter().all(|b| !b) {
            false
        } else {
            todo!("Can this ever happen?")
        }
    }

    /// Finds best phenotypes in current population.
    /// # Returns
    /// Array of best phenotypes.
    fn get_best(&self) -> Vec<T> {
        vec![]
    }

    /// Combines multiple phenotypes, so that a new generation can be created.
    /// # Returns
    /// The foundation of the next generation.
    fn crossover(_phenotypes: Vec<T>) -> Vec<T> {
        vec![]
    }

    /// Moves population to next generation by combining best phenotypes of
    /// current generation
    pub fn restart(&mut self) {

        let best = self.get_best();

        let mut new_population = Self::crossover(best);

        new_population.iter_mut().for_each(|phen| phen.mutate());
        
        self.0 = new_population;
    }

}

impl<T: Phenotype> From<Vec<T>> for Population<T> {
    fn from(vec: Vec<T>) -> Self {
        Population(vec)
    }
}
