fn sum_multiples(n: i32) -> i32 {
    let mut sum = 0;
    for i in 1..n {
        if i % 3 == 0 || i % 5 == 0 {
            sum += i
        }
    }
    return sum;
}

pub fn solve() -> i32 {
    sum_multiples(1000)
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
        let result = solve();
        assert_eq!(result, 233168)
    }
}
