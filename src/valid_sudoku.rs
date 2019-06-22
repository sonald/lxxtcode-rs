pub struct Solution;

impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        // use [0; 9]; could be more efficient
        let mut h = HashSet::new();
        let mut v = HashSet::new();
        let mut subs = vec![HashSet::new(); 9];

        for i in 0..9 {
            v.clear();
            for j in 0..9 {
                let ch2 = board[i][j];
                if ch2 == '.' { continue; }
                match v.get(&ch2) {
                    None => {
                        v.insert(ch2);
                    }
                    _ => return false,
                }
            }

            h.clear();
            for j in 0..9 {
                let ch = board[j][i];
                if ch == '.' { continue; }

                match h.get(&ch) {
                    None => {
                        h.insert(ch);
                    }
                    _ => return false
                }

                let d = (i/3) * 3 + (j/3);
                match subs[d].get(&ch) {
                    None => {
                        subs[d].insert(ch);
                    }
                    _ => return false
                }
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sudoku() {
        test_method(Solution::is_valid_sudoku);
    }

    fn test_method(f: impl Fn(Vec<Vec<char>>) -> bool) {
        {
            let vv = vec![
                vec!['5','3','.','.','7','.','.','.','.'],
                vec!['6','.','.','1','9','5','.','.','.'],
                vec!['.','9','8','.','.','.','.','6','.'],
                vec!['8','.','.','.','6','.','.','.','3'],
                vec!['4','.','.','8','.','3','.','.','1'],
                vec!['7','.','.','.','2','.','.','.','6'],
                vec!['.','6','.','.','.','.','2','8','.'],
                vec!['.','.','.','4','1','9','.','.','5'],
                vec!['.','.','.','.','8','.','.','7','9']
            ];
            assert_eq!(f(vv), true);
        }
        {
            let vv = vec![
                vec!['8','3','.','.','7','.','.','.','.'],
                vec!['6','.','.','1','9','5','.','.','.'],
                vec!['.','9','8','.','.','.','.','6','.'],
                vec!['8','.','.','.','6','.','.','.','3'],
                vec!['4','.','.','8','.','3','.','.','1'],
                vec!['7','.','.','.','2','.','.','.','6'],
                vec!['.','6','.','.','.','.','2','8','.'],
                vec!['.','.','.','4','1','9','.','.','5'],
                vec!['.','.','.','.','8','.','.','7','9']
            ];
            assert_eq!(f(vv), false);
        }
    }
}

