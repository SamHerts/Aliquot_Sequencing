extern crate num;


use std::collections::HashMap;
use prime_factorization::Factorization;
use num::{BigUint, FromPrimitive, Integer, One, Zero};


fn main() {
    // let mut num: u128 = 276;
    let mut num: u128 = 169766367014411422183878825491823897888;
    let mut step = 0;
    let mut too_big_flag = false;
    let mut temp_num: BigUint = Default::default();
    let mut division: BigUint = Default::default();
    let mut remainder: BigUint = Default::default();

    loop {
        if too_big_flag {
            (division, remainder) = temp_num.div_rem(&BigUint::from_u32(2).expect("Nope"));
            let mut two_count = 0;
            while remainder == Zero::zero() {
                two_count += 1;
                (division, remainder) = temp_num.div_rem(&BigUint::from_u32(2).expect("Nope"));
            }
            println!("two count: {}", two_count);
            let next_num = u128::try_from(temp_num.clone());
            match next_num {
                Ok(n)=> {
                    num = n;

                    },
                Err(..) => {println!("Nope"); break;},
            }
        }
        
        let mut factors = Factorization::run(num).factors;
        if too_big_flag {
            factors.push(2);
        }
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

        let next_num = u128::try_from(result.clone());
        match next_num {
            Ok(n)=> num = n,
            Err(..) => {too_big_flag = true; temp_num = result.clone()},
        }
    }
}
