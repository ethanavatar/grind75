struct Solution;

impl Solution {
    pub fn is_palindrome(s: String) -> bool {
        let mut lower: String = s.clone();
        lower.make_ascii_lowercase();
        let chars: Vec<char> = lower.chars().filter(|&c| {c.is_alphanumeric() && !c.is_whitespace()}).collect();
        chars.iter().rev().eq(chars.iter())
    }
}

fn main() {
    let s = "A man, a plan, a canal: Panama".to_string();
    let ret = Solution::is_palindrome(s);
    assert_eq!(ret, true);

    let s = "race a car".to_string();
    let ret = Solution::is_palindrome(s);
    assert_eq!(ret, false);

    let s = "".to_string();
    let ret = Solution::is_palindrome(s);
    assert_eq!(ret, true);

    let s = "a.".to_string();
    let ret = Solution::is_palindrome(s);
    assert_eq!(ret, true);

    let s = " ".to_string();
    let ret = Solution::is_palindrome(s);
    assert_eq!(ret, true);
}