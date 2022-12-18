use super::q3_primefactor::{factorisation_to_hashmap, prime_factorise};
use std::collections::HashMap;

fn solve_factorise(n: u64) -> u64 {
    let mut factors: HashMap<u64, u64> = HashMap::new();
    for i in 2..=n {
        let prime_factorisation = prime_factorise(i);
        let factorisation = factorisation_to_hashmap(prime_factorisation);
        for (factor, power) in &factorisation {
            let curr_pow = *factors.entry(*factor).or_insert(1);
            factors.insert(*factor, curr_pow.max(*power));
        }
    }
    factors
        .into_iter()
        .map(|(factor, power)| factor.pow(power as u32))
        .product()
}

fn bruteforce(n: u64) -> u64 {
    let mut nums = n..;
    nums.find(|x| (2..=n).all(|d| *x % d == 0)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn example_factorisation_is_correct() {
        let result = solve_factorise(10);
        assert_eq!(result, 2520)
    }

    #[test]
    fn solution_factorisation_is_correct() {
        let result = solve_factorise(20);
        assert_eq!(result, 232792560)
    }

    #[test]
    fn example_bruteforce_is_correct() {
        let result = bruteforce(10);
        assert_eq!(result, 2520)
    }

    #[test]
    fn solution_bruteforce_is_correct() {
        let result = bruteforce(20);
        assert_eq!(result, 232792560)
    }
}
