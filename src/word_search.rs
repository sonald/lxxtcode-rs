pub struct Solution;

impl Solution {
    pub fn exist(board: Vec<Vec<char>>, word: String) -> bool {
        if word.len() == 0 {
            return true;
        }
        Self::by_dfs(board, word)        
    }

    pub fn by_dfs(board: Vec<Vec<char>>, word: String) -> bool {
        let nr = board.len();
        let nc = board[0].len();

        fn dfs(board: &Vec<Vec<char>>, i: usize, j: usize,
               meet: &mut Vec<Vec<bool>>, sp: &[u8], k: usize) -> bool {
            if k == sp.len() {
                return true;
            }

            if meet[i][j] {
                return false;
            }


            //eprintln!("[{}, {}] = {}, {}", i, j, board[i][j], sp[k]);
            let ch = char::from(sp[k]);

            if ch != board[i][j] {
                return false;
            }

            meet[i][j] = true;
            if i < board.len()-1 && dfs(&board, i+1, j, meet, &sp, k+1) {
                return true;
            }

            if j < board[0].len()-1 && dfs(&board, i, j+1, meet, &sp, k+1) {
                return true;
            }

            if i > 0 && dfs(&board, i-1, j, meet, &sp, k+1) {
                return true;
            }

            if j > 0 && dfs(&board, i, j-1, meet, &sp, k+1) {
                return true;
            }

            meet[i][j] = false;
            k+1 == sp.len()
        }


        let mut meet = vec![vec![false; nc]; nr];
        let sp = word.as_bytes();
        for i in 0..nr {
            for j in 0..nc {
                if dfs(&board, i, j, &mut meet, &sp, 0) {
                    return true;
                }
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn method_test(f: impl Fn(Vec<Vec<char>>, String) -> bool) {
        {
            let board = vec!{ 
                vec!['A'],
            };

            assert_eq!(f(board.clone(), "A".to_string()), true);
            assert_eq!(f(board.clone(), "E".to_string()), false);
        }
        {
            let board = vec!{ 
                vec!['A','B','C','E'],
            };

            assert_eq!(f(board.clone(), "A".to_string()), true);
            assert_eq!(f(board.clone(), "B".to_string()), true);
            assert_eq!(f(board.clone(), "C".to_string()), true);
            assert_eq!(f(board.clone(), "E".to_string()), true);
        }
        {
            let board = vec!{ 
                vec!['A','B','C','E'],
                vec!['S','F','C','S'],
                vec!['A','D','E','E']
            };

            assert_eq!(f(board.clone(), "".to_string()), true);
            assert_eq!(f(board.clone(), "ABCCED".to_string()), true);
            assert_eq!(f(board.clone(), "SEE".to_string()), true);
            assert_eq!(f(board.clone(), "ABCB".to_string()), false);
        }
    }

    #[test]
    fn exist_test() {
        method_test(Solution::by_dfs);
    }
}
