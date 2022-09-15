///! This module holds everything that describes genetic algorithm

use rand::random;

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
    /// The greater the ammount, the more mutations happen.
    fn mutate(&mut self, ammount: f32);

    /// Combines genomes of 2 separate objects
    fn crossover(&self, other: &Self) -> Self;

    /// Creates new phenotype with original's DNA.
    fn from(other: &Self) -> Self;
}

/// Wrapper struct that provides an interface to population.
#[derive(Debug)]
pub struct Population<T>(Vec<T>)
where
    T: Phenotype;

impl<Phen: Phenotype> Population<Phen> {
    /// Gets underlying data. Used for looping.
    pub fn get(&self) -> &Vec<Phen> {
        &self.0
    }

    /// Updates all objects, using Phenotype::update function
    /// # Returns
    /// - ```true``` if all objects' update functions returned true
    /// - ```false``` if all objects' update functions returned false
    /// # Panics
    /// This function will panic if some object's finish their life, and some don't.
    /// In current implementation it will not happen because rockets don't die
    /// when they hit walls or the target.
    pub fn update(&mut self) -> bool {
        let bools: Vec<bool> = (&mut self.0)
            .iter_mut()
            .map(|phen: &mut Phen| phen.update())
            .collect();

        if bools.iter().all(|b| *b) {
            true
        } else if bools.iter().all(|b| !b) {
            false
        } else {
            todo!("This will happen if rocket's life ends prematurely")
        }
    }

    /// Finds ```m``` best phenotypes in current population, where ```m``` is
    /// the number of phenotypes, which can be crossovered into the population of
    /// the same size, or a roughly equal size(see formula for explanation)
    ///
    /// # Formula
    ///
    /// When ```m``` phenotypes are crossovered with each other the result has
    /// ```m(m - 1) / 2``` elements. There are ```p``` phenotypes in population, so
    /// ```m(m - 1) = 2p```. Assuming ```m, n ∈ N```, ```m``` can be solved for as
    /// ```m = (1 + sqrt(1 + 8p)) / 2```
    ///
    /// # Note
    ///
    /// Best elements are not really best, because there are probabilities
    /// involved, aka the better the phenotype is the higher the probability of
    /// selection is.
    ///
    /// # Returns
    /// Array of best phenotypes.
    fn get_best(&self) -> Vec<&Phen> {
        let population_size = self.get().len() as f32;
        let probability: f32 = 1.0;

        let tournament_size = population_size as usize / 3;
        let no_tournaments: usize = ((1.0 + (1.0 + 8.0 * population_size).sqrt()) / 2.0).round() as usize;

        (0..no_tournaments)
            .map(|_| {
                let mut members = (0..tournament_size)
                    .map(|_| random::<usize>() % population_size as usize)
                    .map(|idx| &self.get()[idx])
                    .collect::<Vec<_>>();

                members
                    .sort_by(|p1, p2| p1.calculate_fitness().total_cmp(&p2.calculate_fitness()));

                let chance = random::<f32>();

                let r = 1_f32 - probability;
                let p = probability;

                let winner = (0..)
                    .take_while(|x| chance > p * (1_f32 - r.powi(x + 1)) / (1_f32 - r))
                    .take(members.len() - 1)
                    .count();

                members[members.len() - 1 - winner]
            }).collect()
    }

    /// Combines multiple phenotypes, so that a new generation can be created.
    /// # Returns
    /// The foundation of the next generation.
    fn crossover(phenotypes: Vec<&Phen>) -> Vec<Phen> {
        let mut result = Vec::new();

        for i in 0..phenotypes.len() - 1 {
            for j in i+1..phenotypes.len() {
                let phen = Phen::crossover(phenotypes[i], phenotypes[j]);

                result.push(phen);
            }
        }

        result
    }

    /// Moves population to next generation by combining best phenotypes of
    /// current generation
    pub fn restart(&mut self) {
        let best = self.get_best();

        let mut new_population = Self::crossover(best);
        let orig_size = new_population.len();

        new_population.iter_mut()
            .take(orig_size - 1)
            .for_each(|phen| phen.mutate(0.05));
        self.0 = new_population;
    }
}

impl<T: Phenotype> From<Vec<T>> for Population<T> {
    fn from(vec: Vec<T>) -> Self {
        Population(vec)
    }
}
