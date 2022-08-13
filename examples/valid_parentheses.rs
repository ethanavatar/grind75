struct Solution;

impl Solution {
    pub fn is_valid(s: String) -> bool {
        let mut stack: Vec<char> = Vec::new();
        for c in s.chars() {
            match c {
                a @ ('(' | '[' | '{') => stack.push(a),
                a @ (')' | ']' | '}') => {
                    if stack.is_empty() {
                        return false;
                    }

                    match a {
                        ')' => {
                            match stack.pop() {
                                Some('(') => (),
                                _ => return false,
                            }
                        },
                        ']' => {
                            match stack.pop() {
                                Some('[') => (),
                                _ => return false,
                            }
                        },
                        '}' => {
                            match stack.pop() {
                                Some('{') => (),
                                _ => return false,
                            }
                        },
                        _ => unreachable!()
                    }
                }
                _ => panic!()
            }
        }
        stack.is_empty()
    }
}

fn main() {
    let s = "()".to_string();
    let ret = Solution::is_valid(s);
    assert_eq!(ret, true);

    let s = "()[]{}".to_string();
    let ret = Solution::is_valid(s);
    assert_eq!(ret, true);

    let s = "(]".to_string();
    let ret = Solution::is_valid(s);
    assert_eq!(ret, false);
}