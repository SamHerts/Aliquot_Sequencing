extern crate num;

use std::collections::HashMap;
use prime_factorization::Factorization;
use num::{BigUint, FromPrimitive, Integer, One, Zero};
use std::fmt::{Display, Formatter, Error, Debug};

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

// impl Debug for PrimePower {
//     fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
//         write!(f, "{}*{}", self.prime_factor, self.power_count)
//     }
// }

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


fn main() {
    let mut num: u128 = 276;
    // let mut num: u128 = 169766367014411422183878825491823897888;
    let mut step = 1;
    let mut too_big_flag = false;
    let mut temp_num: BigUint = Default::default();
    let mut division: BigUint = Default::default();
    let mut remainder: BigUint = Default::default();

    loop {
        let mut two_count = 0;
        if too_big_flag {
            (division, remainder) = temp_num.div_rem(&BigUint::from_u32(2).expect("Can't divide by 2"));
            while remainder == Zero::zero() {
                two_count += 1;
                (division, remainder) = temp_num.div_rem(&BigUint::from_u32(2).expect("Can't divide by 2 again"));
                if remainder == Zero::zero() {
                    temp_num = division;
                }
            }
            println!("two count: {}", two_count);
            let next_num = u128::try_from(temp_num.clone());
            match next_num {
                Ok(n)=> {
                    num = n;

                    },
                Err(..) => {println!("Can't convert to int"); break;},
            }
        }
        
        let mut factors = Factorization::run(num).factors;
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
            let temp: BigUint = (prime.prime_factor.pow(((prime.power_count) + 1) as u32) - BigUint::one()) / (prime.prime_factor.clone() - BigUint::one());
            result *= temp;
        }
        result -= num.clone();

        println!("{} .  {} = {}", step, result, deduplicated_vector);
        if result == Zero::zero() {
            break;
        }
        step += 1;

        let next_num = u128::try_from(result.clone());
        match next_num {
            Ok(n)=> num = n,
            Err(..) => {too_big_flag = true; temp_num = result.clone()},
        }
    }
}
