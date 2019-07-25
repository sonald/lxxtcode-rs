pub struct Solution;

impl Solution {
    pub fn can_finish(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        //Self::by_dfs(num_courses, prerequisites)
        Self::by_bfs(num_courses, prerequisites)
    }

    pub fn by_bfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut g = vec![vec![]; num_courses as usize];
        let mut indeg = vec![0; num_courses as usize];
        let mut done = vec![false; num_courses as usize];
        for v in prerequisites {
            g[v[0] as usize].push(v[1] as usize);
            indeg[v[1] as usize] += 1;
        }

        loop {
            let mut changed = false;
            for u in 0..num_courses as usize {
                if done[u] {
                    continue;
                }

                if indeg[u] == 0 {
                    for &v in &g[u] {
                        if indeg[v] > 0 {
                            changed = true;
                            indeg[v] -= 1;
                        }
                    }
                    done[u] = true;
                }
            }

            if !changed {
                break
            }
        }

        indeg.iter().all(|d| *d == 0)
    }

    pub fn by_dfs(num_courses: i32, prerequisites: Vec<Vec<i32>>) -> bool {
        let mut g = vec![vec![]; num_courses as usize];
        for v in prerequisites {
            g[v[0] as usize].push(v[1] as usize);
        }

        let mut sp = vec![false; num_courses as usize];
        let mut visited = vec![false; num_courses as usize];

        for i in 0..num_courses as usize {
            if !visited[i] {
                visited[i] = true;
                sp[i] = true;
                if Self::has_cycle(&g, i, &mut visited, &mut sp) {
                    return false;
                }
                sp[i] = false;
            }
        }

        true
    }

    fn has_cycle(g: &Vec<Vec<usize>>, u: usize, visited: &mut Vec<bool>, sp: &mut Vec<bool>) -> bool {
        for &v in &g[u as usize] {
            if !visited[v] {
                visited[v] = true;
                sp[v] = true;
                if Self::has_cycle(g, v, visited, sp) {
                    return true;
                }
                sp[v] = false;
            } else {
                if sp[v] {
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

    #[test]
    fn can_test() {
        method_test(Solution::by_dfs);
        method_test(Solution::by_bfs);
    }

    fn method_test(f: impl Fn(i32, Vec<Vec<i32>>) -> bool) {
        let v = vec![vec![1, 0]];
        assert_eq!(f(2, v), true);

        let v = vec![vec![1, 0], vec![0, 1]];
        assert_eq!(f(2, v), false);

        let v = vec![vec![0,1],vec![0,2],vec![1,2]];
        assert_eq!(f(3, v), true);
            
        let v = vec![vec![0,2],vec![1,2],vec![2,0]];
        assert_eq!(f(3, v), false);
        
        use std::time::*;

        let v = include!("./course_schedule.txt").iter().map(|v|v.to_vec()).collect::<Vec<_>>();
        let now = Instant::now();
        assert_eq!(f(2000, v.clone()), true);
        eprintln!("-------{}", now.elapsed().as_millis());
    }
}
