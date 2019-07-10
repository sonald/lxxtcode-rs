pub struct Solution;

impl Solution {
    pub fn eval_rpn(mut tokens: Vec<String>) -> i32 {
        let mut sp = Vec::<i32>::new();
        for s in tokens {
            match s.as_str() {
                "+" | "-" | "*" | "/" => {
                    let r = sp.pop().unwrap();
                    let l = sp.pop().unwrap();
                    let res = match s.as_str() {
                        "+" => l + r,
                        "-" => l - r,
                        "*" => l * r,
                        "/" => l / r,
                        _ => unreachable!()
                    };

                    sp.push(res);

                }
                v => {
                    sp.push(s.parse::<i32>().unwrap());
                }
            }
        }
        
        sp[0]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn eval_test() {
        {
            let v = ["2", "1", "+", "3", "*"].iter().map(|s| s.to_string()).collect::<Vec<_>>();
            assert_eq!(Solution::eval_rpn(v), 9);
        }
        {
            let v = ["4", "13", "5", "/", "+"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
            assert_eq!(Solution::eval_rpn(v), 6);
        }
        {
            let v = ["10", "6", "9", "3", "+", "-11", "*", "/", "*", "17", "+", "5", "+"].iter().map(|s| s.to_string()).collect::<Vec<String>>();
            assert_eq!(Solution::eval_rpn(v), 22);
        }
    }
}
