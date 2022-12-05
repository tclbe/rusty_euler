fn fib_range(max_term: i32) -> Vec<i32> {
    let mut result: Vec<i32> = vec![1, 2];

    while result.last().unwrap() <= &max_term {
        let length = result.len();
        result.push(result[length - 2] + result[length - 1]);
    }

    if result.last().unwrap() > &max_term {
        result.pop();
    }

    return result;
}

pub fn solve() -> i32 {
    let fib = fib_range(4 * i32::pow(10, 6));

    let mut sum = 0;
    for i in fib {
        if i % 2 == 0 {
            sum += i;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_first_two_terms_correct() {
        let result = fib_range(2);
        assert_eq!(result, vec![1, 2])
    }

    #[test]
    fn fib_terms_under_10_correct() {
        let result = fib_range(10);
        assert_eq!(result, vec![1, 2, 3, 5, 8])
    }

    #[test]
    fn solution_is_correct() {
        let result = solve();
        assert_eq!(result, 4613732)
    }
}
