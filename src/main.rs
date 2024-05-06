use prime_factorization::Factorization;


fn main() {
    // Factorize following semiprime
    let mut num: u128 = 62;
    let mut step = 1;

    loop {
        let mut factors = Factorization::run(num).factors;
        let mut extra_factors = factors.clone();
        extra_factors.dedup();
        factors.append(&mut extra_factors);
        factors.sort();
        factors.push(0);
        // let sum: u128 = factors.iter().sum();
        let mut multiplication = 0;
        let mut prime_sum = 0;
        let mut prime_index = 0;
        let mut previous = factors[0];

        for val in factors.iter() {
            if previous == *val {
                prime_sum += (*val).pow(prime_index);
            }
            else {
                multiplication += prime_sum;
                prime_index = 0;
                prime_sum = (*val).pow(prime_index);
            }

            prime_index +=1;
            previous = *val;
        }

        println!("Number: {}, Factors: {:?}, Sum: {}, Step: {}", num, factors, multiplication, step);
        if multiplication < 0 {
            break;
        }
        step += 1;

        num = multiplication;
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
