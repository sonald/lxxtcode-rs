pub struct Solution;

impl Solution {
    pub fn num_islands(grid: Vec<Vec<char>>) -> i32 {
        Self::by_dfs(grid)
    }

    pub fn by_dfs(mut grid: Vec<Vec<char>>) -> i32 {
        fn clear_island(g: &mut Vec<Vec<char>>, i: usize, j: usize) {
            if g[i][j] != '1' {
                return;
            }

            g[i][j] = '0';

            if i > 0 {
                clear_island(g, i-1, j);
            }
            if i+1 < g.len() {
                clear_island(g, i+1, j);
            }
            if j > 0 {
                clear_island(g, i, j-1);
            }
            if j+1 < g[0].len() {
                clear_island(g, i, j+1);
            }
        }

        if grid.len() == 0 {
            return 0;
        }

        let mut count = 0;
        for i in 0..grid.len() {
            for j in 0..grid[0].len() {
                if grid[i][j] == '1' {
                    count += 1;
                    clear_island(&mut grid, i, j);
                }
            }
        }

        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn island_test() {
        let v = vec! {
            vec!['1', '1', '0', '1', '0'],
        };
        assert_eq!(Solution::by_dfs(v), 2);

        let v = vec! {
            vec!['1'],
            vec!['0'],
            vec!['0'],
            vec!['1'],
        };
        assert_eq!(Solution::by_dfs(v), 2);

        let v = vec! {
            vec!['1', '1', '1', '1', '0'],
            vec!['1', '1', '0', '1', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '0', '0', '0'],
        };
        assert_eq!(Solution::by_dfs(v), 1);

        let v = vec! {
            vec!['1', '1', '0', '0', '0'],
            vec!['1', '1', '0', '0', '0'],
            vec!['0', '0', '1', '0', '0'],
            vec!['0', '0', '0', '1', '1'],
        };
        assert_eq!(Solution::by_dfs(v), 3);
    }
}
