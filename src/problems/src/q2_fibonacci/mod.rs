use std::iter::successors;

fn fib_range(max_term: i32) -> Vec<i32> {
    let fib_iter = successors(Some((0, 1)), |(f1, f2)| Some((*f2, f1 + f2)));

    let fib_terms = fib_iter.take_while(|(_, f2)| f2 <= &max_term);

    return fib_terms.map(|(_, f2)| f2).collect::<Vec<i32>>();
}

fn solve() -> i32 {
    let fib = fib_range(4 * i32::pow(10, 6));

    fib.iter().filter(|x| **x % 2 == 0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fib_first_three_terms_correct() {
        let result = fib_range(2);
        assert_eq!(result, vec![1, 1, 2])
    }

    #[test]
    fn fib_terms_under_10_correct() {
        let result = fib_range(10);
        assert_eq!(result, vec![1, 1, 2, 3, 5, 8])
    }

    #[test]
    fn solution_is_correct() {
        let result = solve();
        assert_eq!(result, 4613732)
    }
}
