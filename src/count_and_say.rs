pub struct Solution;

impl Solution {
    pub fn count_and_say(n: i32) -> String {
        let mut s = String::from("1");
        for _ in 2..=n {
            let mut t = String::new();

            let mut it = s.chars();
            let mut prev = it.next().unwrap();
            let mut count = 1;
            for ch in it {
                if prev == ch {
                    count += 1;
                } else {
                    t.push_str(&format!("{}{}", count, prev));
                    prev = ch;
                    count = 1;
                }
            }

            t.push_str(&format!("{}{}", count, prev));

            std::mem::swap(&mut s, &mut t);
        }
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_say() {
        let m = vec![(1, "1"), (2, "11"), (3, "21"), (4, "1211"), (5, "111221"),
            (6, "312211"), (7, "13112221"), (8, "1113213211")];
        for (k, v) in m {
            assert_eq!(Solution::count_and_say(k), v.to_string());
        }
    }
}
