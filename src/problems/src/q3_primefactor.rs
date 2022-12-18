use std::collections::HashMap;

fn sieve_of_erathosthenes(n: u64) -> Vec<u64> {
    let mut primes: Vec<u64> = (2..=n).collect();

    for i in 0..primes.len() {
        let prime = primes[i];

        if primes[i] == 0 {
            continue;
        } else {
            for j in (i + prime as usize..primes.len()).step_by(prime as usize) {
                primes[j as usize] = 0;
            }
        }
    }
    return primes.into_iter().filter(|x| *x != 0).collect::<Vec<u64>>();
}

fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }

    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = f64::sqrt(n as f64);
    for i in (3..=sqrt_n as u64).step_by(2) {
        if n % i == 0 {
            return false;
        }
    }
    return true;
}

fn largest_prime_factor(n: u64) -> u64 {
    let mut n = n.clone();
    let mut largest_prime_factor = 2;

    while n % 2 == 0 {
        n = n / 2;
    }

    let sqrt_n = f64::sqrt(n as f64);

    for i in (3..sqrt_n as u64).step_by(2) {
        if is_prime(i) {
            if n % i == 0 {
                largest_prime_factor = i;
                n = n / i;
            }
        }
    }

    largest_prime_factor
}

pub fn prime_factorise(n: u64) -> Vec<u64> {
    let sqrt_n = f64::sqrt(n as f64);
    let primes = sieve_of_erathosthenes(sqrt_n as u64);
    let mut factorisation: Vec<u64> = vec![];

    let mut n = n.clone();

    for prime in &primes {
        if let Result::Ok(_) = primes.binary_search(&n) {
            factorisation.push(n);
            break;
        }
        while n % prime == 0 {
            factorisation.push(*prime);
            n /= prime;
        }
    }

    if n >= 2 {
        factorisation.push(n);
    }

    factorisation
}

pub fn factorisation_to_hashmap(factors: Vec<u64>) -> HashMap<u64, u64> {
    let mut result: HashMap<u64, u64> = HashMap::new();
    for factor in factors {
        result.entry(factor).and_modify(|x| *x += 1).or_insert(1);
    }
    result
}

pub fn solve_full_factorisation() -> u64 {
    let prime_factorisation = prime_factorise(600851475143);
    return *prime_factorisation.last().unwrap();
}

pub fn solve() -> u64 {
    let largest_prime = largest_prime_factor(600851475143);
    return largest_prime;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_are_primes() {
        let test_vals = vec![2, 3, 5, 7, 11, 13, 17, 19];
        for val in test_vals {
            let result = is_prime(val);
            assert_eq!(result, true, "{val} is prime!")
        }
    }

    #[test]
    fn not_primes_are_not_primes() {
        let test_vals = vec![4, 6, 8, 9, 10, 12, 14, 15, 16, 18];
        for val in test_vals {
            let result = is_prime(val);
            assert_eq!(result, false, "{val} is not prime!")
        }
    }

    #[test]
    fn largest_prime_example_correct() {
        let result = largest_prime_factor(13195);
        assert_eq!(result, 29)
    }

    #[test]
    fn primes_under_20_correct() {
        let result = sieve_of_erathosthenes(20);
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19])
    }

    #[test]
    fn prime_factorise_correct() {
        assert_eq!(prime_factorise(2), [2]);
        assert_eq!(prime_factorise(3), [3]);
        assert_eq!(prime_factorise(4), [2, 2]);
        assert_eq!(prime_factorise(5), [5]);
        assert_eq!(prime_factorise(6), [2, 3]);
    }
    #[test]
    fn prime_factorise_example_correct() {
        let result = prime_factorise(13195);
        assert_eq!(result, vec![5, 7, 13, 29])
    }

    #[test]
    fn solution_is_correct_full_factorisation() {
        let result = solve_full_factorisation();
        assert_eq!(result, 6857)
    }

    #[test]
    fn solution_is_correct() {
        let result = solve();
        assert_eq!(result, 6857)
    }
}
