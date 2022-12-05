fn is_palindrome(s: String) -> bool {
    if s.chars().nth(0) == s.chars().last() {
        if s.len() <= 2 {
            return true;
        }
        return is_palindrome(s[1..s.len() - 1].to_string());
    }
    return false;
}

fn solve() -> i32 {
    let mut palindromic: Vec<i32> = Vec::new();
    for i in 101..999 {
        for j in 101..999 {
            let result = i * j;
            let result_str = result.to_string();
            if is_palindrome(result_str) {
                palindromic.push(result);
            }
        }
    }

    return *palindromic.iter().max().unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn palindrome_is_palindrome() {
        let result = is_palindrome("racecar".to_string());
        assert_eq!(result, true)
    }

    #[test]
    fn not_palindrome_is_not_palindrome() {
        let result = is_palindrome("palindrome".to_string());
        assert_eq!(result, false)
    }

    #[test]
    fn solution_is_correct() {
        let result = solve();
        assert_eq!(result, 906609);
    }
}
