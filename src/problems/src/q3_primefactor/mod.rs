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
    if n % 2 == 0 {
        return false;
    }
    let sqrt_n = f64::sqrt(n as f64);
    for i in (3..sqrt_n as u64).step_by(2) {
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

fn prime_factorise(n: u64) -> Vec<u64> {
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

    factorisation
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
    fn is_prime_29() {
        let result = is_prime(29);
        assert_eq!(result, true)
    }

    #[test]
    fn is_not_prime_81() {
        let result = is_prime(81);
        assert_eq!(result, false)
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
