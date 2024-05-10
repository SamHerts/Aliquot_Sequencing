extern crate num;

use std::collections::HashMap;
use prime_factorization::Factorization;
use num::{BigUint, FromPrimitive, Integer, One, ToPrimitive, Zero};
use std::fmt::{Display, Formatter, Error, Debug};
use std::ops::{Div, Sub};

#[derive(PartialEq, Eq, PartialOrd, Ord)]
struct PrimePower {
    prime_factor: BigUint,
    power_count: usize,
}

struct FactorVec(Vec<PrimePower>);

impl Display for FactorVec {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        for v in &self.0 {
            write!(f, "{} * ", v)?;
        }
        Ok(())
    }
}

impl Debug for PrimePower {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}*{}", self.prime_factor, self.power_count)
    }
}

impl Display for PrimePower {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "{}*{}", self.prime_factor, self.power_count)
    }
}

impl From<PrimePower> for (BigUint, usize) {
    fn from(e: PrimePower) -> (BigUint, usize) {
        let PrimePower { prime_factor, power_count }: PrimePower = e;
        (prime_factor, power_count)
    }
}

impl From<(BigUint, usize)> for PrimePower {
    fn from(e: (BigUint, usize)) -> PrimePower {
        let (a, b): (BigUint, usize) = e;
        PrimePower{ prime_factor: a, power_count: b }
    }
}

fn remove_factor(mut composite: BigUint, factor_to_remove: u32) -> (BigUint, usize) {
    let mut factor_count: usize = 1;
    let mut div = composite.clone();
    let mut rem;
    while composite.is_multiple_of(&BigUint::from(factor_to_remove)) {
        factor_count += 1;
        (div, rem) = composite.div_rem(&BigUint::from(factor_to_remove));
        println!("{composite} / {factor_to_remove} = {div}");
        if rem == Zero::zero() {
            composite = div.clone();
        }
        else {
            break;
        }
    }
    (div, factor_count)
}


fn main() {
    // let mut num: u128 = 276;
    let mut num: u128 = 169766367014411422183878825491823897888;
    let mut step = 1;
    let mut too_big_flag = false;
    let mut temp_num: BigUint = Default::default();
    let mut working_num: BigUint = Default::default();
    let mut division: BigUint = Default::default();
    let mut remainder: BigUint = Default::default();

    loop {
        let mut two_count = 1;
        if too_big_flag {
            println!("Beginnging of Loop: {working_num}");
            println!("Could not convert {working_num} to u128. Removing powers of 2");
            temp_num = working_num.clone();
            // while temp_num.is_multiple_of(&BigUint::from(2u8)) {
            //     two_count += 1;
            //     (division, remainder) = temp_num.div_rem(&BigUint::from(2u8));
            //     if remainder == Zero::zero() {
            //         println!("{temp_num} / 2 = {division}");
            //         temp_num = division;
            //     }
            // }

            (temp_num, two_count) = remove_factor(temp_num, 2);

            let next_num = temp_num.to_u128();
            match next_num {
                Some(n)=> { num = n; },
                None => {
                    println!("Could not convert {temp_num} to u128. Removing powers of 3");
                    (temp_num, two_count) = remove_factor(temp_num, 3);
                    let next_num = temp_num.to_u128();
                    match next_num {
                        Some(n)=> { num = n; },
                        None => {
                            println!("Could not convert {temp_num} to u128 even after removing powers of 3");
                            break;
                        },
                    }
                },
            }
        }
        else {
            println!("Beginnging of Loop: {num}");
        }

        println!("Trying to factor {num}");
        let mut factors = Factorization::run(num).factors;
        // !TODO: Add factors of 3 back into the vector
        if too_big_flag && two_count > 0 {
            for _ in 1..two_count {
                factors.push(2);
            }
        }
        // Create a HashMap to count occurrences of each integer
        let mut counts: HashMap<BigUint, usize> = HashMap::new();
        for &num  in &factors {
            *counts.entry(BigUint::from(num.clone())).or_insert(0) += 1;
        }

        // Create a new vector with counts
        let mut deduplicated_vector: Vec<PrimePower> = counts.into_iter().map(|(key, value)| PrimePower { prime_factor: key, power_count: value }).collect();
        deduplicated_vector.sort();

        let mut result: BigUint = One::one();

        for prime in &deduplicated_vector  {
            // (p^(a+1) - 1) / (p - 1)
            // let temp: BigUint = (prime.prime_factor.pow(((prime.power_count) + 1) as u32) - BigUint::one()) / (prime.prime_factor.clone() - BigUint::one());
            let pa1 = prime.prime_factor.pow(((prime.power_count) + 1) as u32);
            let pa1minus1 = pa1.clone() - BigUint::one();
            let pminus1 = prime.prime_factor.clone() - BigUint::one();
            let temp = pa1minus1.clone().div(pminus1.clone());
            println!("Working on {prime}, p^(a+1) = {pa1} | p^(a+1) - 1 = {pa1minus1} | p - 1 = {pminus1} | p^(a+1) / (p-1) = {temp}");
            result *= temp;
        }

        if too_big_flag{
            result -= working_num.clone();
            println!("{step} .  {working_num} = {deduplicated_vector:?} | Sum of Divisors: {result}");
        }
        else {
            result -= num;
            println!("{step} .  {num} = {deduplicated_vector:?} | Sum of Divisors: {result}");
        }


        if result == Zero::zero() {
            break;
        }
        step += 1;

        let next_num = result.to_u128();
        match next_num {
            Some(n)=> {num = n; too_big_flag = false},
            None => {too_big_flag = true; working_num = result.clone()},
        }
    }
}
