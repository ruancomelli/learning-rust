use std::{collections::HashMap, ops::Add};

use counter::Counter;
use rayon::{
    iter::{IntoParallelRefIterator, ParallelIterator},
    ThreadPoolBuilder,
};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    ThreadPoolBuilder::new()
        .num_threads(worker_count)
        .build()
        .unwrap()
        .install(|| {
            input.par_iter().map(|row| {
                row.chars()
                    .filter(|c| c.is_alphabetic())
                    .flat_map(|c| c.to_lowercase())
                    .collect::<Counter<_>>()
            })
        })
        .reduce(Counter::new, Counter::add)
        .into_map()
}
