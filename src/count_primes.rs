pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        Self::by_direct(n)
    }

    // naive sieve
    pub fn by_sieve(n: i32) -> i32 {
        let mut c = 0;
        let mut v = (2..n).collect::<Vec<_>>();
        while v.len() > 0 {
            let x = v.remove(0);
            c += 1;
            let mut j = 0;
            while j < v.len() {
                if v[j] % x  == 0 {
                    v.remove(j);
                } else {
                    j += 1;
                }
            }
        }

        c
    }

    pub fn by_sieve2(n: i32) -> i32 {
        if n <= 2 { return 0; }
        let mut v = vec![true; n as usize];
        v[0] = false;
        v[1] = false;
        let m = (n as f32).sqrt().floor() as i32;
        for x in 2..=m {
            for i in (x*x..n).step_by(x as usize) {
                v[i as usize] = false;
            }
        }


        v.into_iter().filter(|x| *x).count() as i32
    }

    pub fn by_direct(n: i32) -> i32 {
        (2..n).fold(0, |c, x| {
            if Self::is_prime(x) {
                c+1
            } else {
                c
            }
        })
    }

    fn is_prime(n: i32) -> bool {
        let m = (n as f32).sqrt().floor() as i32;
        n == 2 || (2..=m).all(|i| n % i != 0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_test() {
        let res = [0, 0, 0, 1, 2, 2, 3, 3, 4, 4, 4];
        res.iter().enumerate().for_each(|(i, &x)| {
            assert_eq!(Solution::by_direct(i as i32), x);
        });
        res.iter().enumerate().for_each(|(i, &x)| {
            assert_eq!(Solution::by_sieve2(i as i32), x);
        });

        (11..100).for_each(|i| {
            assert_eq!(Solution::by_direct(i), Solution::by_sieve(i));
            assert_eq!(Solution::by_direct(i), Solution::by_sieve2(i));
        });

        Solution::by_sieve2(999983);
    }
}
