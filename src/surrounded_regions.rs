pub struct Solution;

impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        Self::from_border(board)
        //Self::flood_fill(board)
    }

    pub fn flood_fill(board: &mut Vec<Vec<char>>) {
        fn fill(b: &mut Vec<Vec<char>>, i: usize, j: usize) {

        }

        if board.len() == 0 || board[0].len() == 0 {
            return;
        }

        let nr = board.len();
        let nc = board[0].len();

        for r in 1..nr {
            for c in 1..nc {
                if board[r][c] == 'O' {
                    fill(board, r, c)
                }
            }
        }
    }

    // two passes
    // pass1: mark border O as Y
    // pass2: fill O to X & restore Y to O
    pub fn from_border(board: &mut Vec<Vec<char>>) {
        fn mark_border(b: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if b[i][j] != 'O' {
                return
            }

            b[i][j] = 'Y';

            if i > 0 && b[i-1][j] == 'O' {
                mark_border(b, i-1, j);
            }

            if j < b[0].len()-1 && b[i][j+1] == 'O' {
                mark_border(b, i, j+1);
            }

            if i < b.len()-1 && b[i+1][j] == 'O' {
                mark_border(b, i+1, j);
            }

            if j > 0 && b[i][j-1] == 'O' {
                mark_border(b, i, j-1);
            }
        }

        if board.len() == 0 || board[0].len() == 0 {
            return;
        }

        let nr = board.len();
        let nc = board[0].len();

        for c in 0..nc {
            mark_border(board, 0, c);
            mark_border(board, nr-1, c);
        }

        for r in 0..nr {
            mark_border(board, r, 0);
            mark_border(board, r, nc-1);
        }

        for r in 0..nr {
            for c in 0..nc {
                if board[r][c] == 'O' {
                    board[r][c] = 'X';
                } else if board[r][c] == 'Y' {
                    board[r][c] = 'O';
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve_test() {
        {
            let mut v = vec!{
                vec!['O', 'X', 'X', 'X'],
            };
            let res = vec!{
                vec!['O', 'X', 'X', 'X'],
            };
            Solution::solve(&mut v);

            assert_eq!(v, res);
        }
        {
            let mut v = vec!{
                vec!['X'],
                vec!['O'],
                vec!['X'],
                vec!['X'],
            };
            let res = vec!{
                vec!['X'],
                vec!['O'],
                vec!['X'],
                vec!['X'],
            };
            Solution::solve(&mut v);

            assert_eq!(v, res);
        }
        {
            let mut v = vec!{
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            };
            let res = vec!{
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            };
            Solution::solve(&mut v);

            assert_eq!(v, res);
        }

        {
            let mut v = vec!{
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            };
            let res = vec!{
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
            };
            Solution::solve(&mut v);

            assert_eq!(v, res);
        }

        {
            let mut v = vec!{
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'O', 'X', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            };
            let res = vec!{
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'X', 'X'],
                vec!['X', 'X', 'O', 'X'],
                vec!['X', 'O', 'O', 'X'],
                vec!['X', 'O', 'X', 'X'],
            };
            Solution::solve(&mut v);

            assert_eq!(v, res);
        }
    }
}
