extern crate num;


use std::collections::HashMap;
use prime_factorization::Factorization;
use num::{BigUint, One, Zero};


fn main() {
    let mut num: u128 = 276;
    let mut step = 0;

    loop {
        let factors = Factorization::run(num).factors;
        // Create a HashMap to count occurrences of each integer
        let mut counts: HashMap<BigUint, usize> = HashMap::new();
        for &num  in &factors {
            *counts.entry(BigUint::from(num.clone())).or_insert(0) += 1;
        }

        // Create a new vector with counts
        let deduplicated_vector: Vec<(BigUint, usize)> = counts.into_iter().collect();

        let mut result: BigUint = One::one();

        for (val, power) in &deduplicated_vector {
            let temp: BigUint = (val.pow((*power + 1) as u32) - BigUint::one()) / (val - BigUint::one());
            result *= temp;
        }
        result -= num.clone();
        println!("{result}");


        println!("Step: {}, Factors: {:?}, Sum: {}, Number: {}", step, deduplicated_vector, result, num);
        if result == Zero::zero() {
            break;
        }
        step += 1;

        num = u128::try_from(result).unwrap();
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
