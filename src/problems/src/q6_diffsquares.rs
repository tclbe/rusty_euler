fn solve(n: u64) -> u64 {
    let sum_sq = (1..=n).into_iter().map(|x| x.pow(2)).sum::<u64>();
    let sq_sum = (1..=n).into_iter().sum::<u64>().pow(2);
    sum_sq.abs_diff(sq_sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn example_correct() {
        let result = solve(10);
        assert_eq!(result, 2640)
    }

    #[test]
    fn solution_correct() {
        let result = solve(100);
        assert_eq!(result, 25164150)
    }
}
