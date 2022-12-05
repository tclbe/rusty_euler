fn sum_multiples(n: i64) -> i64 {
    let result: i64 = (1..n).filter(|x| x % 3 == 0 || x % 5 == 0).sum();

    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_is_correct() {
        let result = sum_multiples(10);
        assert_eq!(result, 23)
    }

    #[test]
    fn solution_is_correct() {
        let result = sum_multiples(1000);
        assert_eq!(result, 233168)
    }
}
