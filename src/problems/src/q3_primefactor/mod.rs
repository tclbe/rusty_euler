fn primes_less_than(n: i64) -> Vec<i64> {
    let mut primes: Vec<i64> = vec![2];

    for i in (3..n).step_by(2) {
        let mut is_prime = true;
        for prime in &primes {
            if i % prime == 0 {
                is_prime = false;
                break;
            }
        }

        if is_prime {
            primes.push(i)
        }
    }

    primes
}

fn prime_factorise(n: i64) -> Vec<i64> {
    let sqrt_n = f64::sqrt(n as f64);
    let primes = primes_less_than(sqrt_n as i64);
    let mut factorisation: Vec<i64> = vec![];

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

pub fn solve() -> i64 {
    let prime_factorisation = prime_factorise(600851475143);
    return *prime_factorisation.last().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn primes_under_20_correct() {
        let result = primes_less_than(20);
        assert_eq!(result, vec![2, 3, 5, 7, 11, 13, 17, 19])
    }

    #[test]
    fn prime_factorise_example_correct() {
        let result = prime_factorise(13195);
        assert_eq!(result, vec![5, 7, 13, 29])
    }

    #[test]
    #[ignore]
    fn solution_is_correct() {
        let result = solve();
        assert_eq!(result, 6857)
    }
}
