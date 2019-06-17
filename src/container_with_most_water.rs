#[allow(dead_code)]
pub struct Solution;

#[allow(dead_code)]
impl Solution {
    pub fn max_area(height: Vec<i32>) -> i32 {
        Self::by_shrink(height)
    }

    fn by_shrink(heights: Vec<i32>) -> i32 {
        let mut l = 0;
        let mut r = heights.len() - 1;
        let mut minh = heights[l].min(heights[r]);

        let mut marea = minh * (r-l) as i32;

        while l < r {
            if heights[l] <= heights[r] {
                l += 1;
                while l < r && heights[l] <= minh {
                    l += 1;
                }
            } else {
                r -= 1;
                while l < r && heights[r] <= minh {
                    r -= 1;
                }
            }

            minh = heights[l].min(heights[r]);
            marea = {
                let area =  minh * (r-l) as i32;
                marea.max(area)
            };
        }

        marea
    }
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    //extern crate test;
    //use test::Bencher;
    use super::*;

    pub fn test_method<F>(f: F) where F: Fn(Vec<i32>) -> i32 {
        {
            let v = vec![0,8];
            assert_eq!(f(v), 0);
        }
        {
            let v = vec![2,8];
            assert_eq!(f(v), 2);
        }
        {
            let v = vec![1,8,6,2,5,4,8,3,7];
            assert_eq!(f(v), 49);
        }
        {
            let v = vec![1,2,3,8,5,6,5,4,1,2,3,2];
            assert_eq!(f(v), 24);
        }
    }

    #[test]
    pub fn test_by_shrink() {
        test_method(Solution::by_shrink);
    }
}



