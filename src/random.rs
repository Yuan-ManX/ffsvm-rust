use rand::{distributions, random};

/// Randomizes a data structure
pub trait Randomize {
    /// Randomizes data in a structure (mostly its vectors) within the structure's parameters.
    fn randomize(self) -> Self;
}

pub trait Random {
    /// Creates a new random thing.
    fn new_random() -> Self;
}

/// Creates a vector of random
pub fn random_vec<T>(size: usize) -> Vec<T>
where
    T: Default + Clone,
    distributions::Standard: distributions::Distribution<T>,
{
    let mut array: Vec<T> = vec![Default::default(); size];
    for e in &mut array {
        *e = random()
    }
    array
}
