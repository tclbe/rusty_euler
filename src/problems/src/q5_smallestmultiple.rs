fn bruteforce(n: u64) -> u64 {
    let mut nums = n..;
    nums.find(|x| (2..=n).all(|d| *x % d == 0)).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_bruteforce_is_correct() {
        let result = bruteforce(10);
        assert_eq!(result, 2520)
    }

    #[test]
    fn solution_bruteforce_is_correct() {
        let result = bruteforce(20);
        assert_eq!(result, 2520)
    }
}
