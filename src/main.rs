//! # Rust solution 5 by Kulasko
//!
//! As it's written in the readme, this solution focuses in different multithreading algorithms.
//! Each algorithm is run with a set of different prime flag storage types.

#![warn(missing_docs)]

mod data_type;
mod sieve;

pub use data_type::{DataType, Integer};

use sieve::flag_data::{FlagData, STRIPE_SIZE};
use sieve::{algorithm, flag_data, Algorithm, Sieve, SieveExecute};

use std::time::{Duration, Instant};

pub fn main() {
    let sieve_size: usize = 100000;
    let duration: usize = 5;
    let set_size: usize = 32;

    println!("Starting prime sieve");
    println!("Working set size is {} kB", set_size);

    perform_bench::<Sieve<algorithm::Tile, FlagData<flag_data::Stripe, [u8; STRIPE_SIZE]>, [u8; STRIPE_SIZE]>, algorithm::Tile>(algorithm::Tile(set_size * 1024), sieve_size, duration);
}

/// Executes a specific bench and prints the result.
fn perform_bench<S: SieveExecute<A>, A: Algorithm>(
    algorithm: A,
    sieve_size: usize,
    duration: usize,
) {
    let mut elapsed = Duration::from_secs(0);
    let id_string = format!("{}-{}-u{}", A::ID_STR, S::ID_STR, S::BITS);

    println!();
    println!(
        "Running {} with {} primes for {} seconds",
        id_string, sieve_size, duration
    );

    let start = Instant::now();

    let mut sieve = S::new(sieve_size, algorithm);
    sieve.sieve();
    elapsed = Instant::now() - start;

    // let sieve = last_sieve.expect("Used a duration of zero!");
    let result = sieve.count_primes();

    println!(
        "Time: {}, Threads: {}, Prime count: {}",
        elapsed.as_secs_f64(),
        sieve.thread_count(),
        result
    );
    if let Ok(index) = PRIMES_IN_SIEVE.binary_search_by_key(&sieve_size, |(key, _)| *key) {
        if PRIMES_IN_SIEVE[index].1 == result {
            eprintln!("This result is verified to be correct");
        } else {
            eprintln!("ERROR: Incorrect sieve result!");
        }
    }
}

/// Known prime counts for specific sieve sizes.
const PRIMES_IN_SIEVE: [(usize, usize); 11] = [
    (2, 1),
    (3, 2),
    (4, 2),
    (10, 4),
    (100, 25),
    (1000, 168),
    (10000, 1229),
    (100000, 9592),
    (1000000, 78498),
    (10000000, 664579),
    (100000000, 5761455),
];
