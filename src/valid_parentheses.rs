#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn is_valid(s: String) -> bool {
        Self::by_stack(s)
    }

    fn by_stack(s: String) -> bool {
        let mut sp = vec![];

        for ch in s.chars() {
            match &ch {
                '(' | '[' | '{' => sp.push(ch),
                ')' | ']' | '}' => {
                    if let Some(&ch2) = sp.last() {
                        if (ch == ')' && ch2 == '(') || 
                        (ch == ']' && ch2 == '[') || 
                        (ch == '}' && ch2 == '{') {
                            sp.pop();
                            continue;
                        }
                    }
                    return false;
                },
                _ => {}

            }
        }

        sp.len() == 0
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use super::*;

    pub fn test_method<F>(f: F) where F: Fn(String) -> bool {
        assert_eq!(f("()".to_string()), true);
        assert_eq!(f("".to_string()), true);
        assert_eq!(f("()[]{}".to_string()), true);
        assert_eq!(f("([{}])".to_string()), true);
        assert_eq!(f("((([])))[[{{()}}]]".to_string()), true);

        assert_eq!(f("((([])))[[{{(})}]]".to_string()), false);
        assert_eq!(f("(]".to_string()), false);
        assert_eq!(f("([)]".to_string()), false);
        assert_eq!(f("([)]".to_string()), false);
        assert_eq!(f("(([]())()))".to_string()), false);
        assert_eq!(f("((([]())())]".to_string()), false);
        assert_eq!(f("((([]())())".to_string()), false);
    }

    #[test]
    pub fn test_by_stack() {
        test_method(Solution::by_stack);
    }
}




