// Trial division
//
// Check each number on its own: try dividing by 2, 3, 4, ...
// If something divides it, it's not prime.
//
// Time: O(n^2). n numbers, each can take up to n tries.
//
// Space: O(1).

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let mut cnt: i32 = 0;
        for i in 0..n {
            cnt += i32::from(Self::is_prime(i));
        }
        cnt
    }

    fn is_prime(num: i32) -> bool {
        if num < 2 {
            return false;
        }

        for i in 2..num {
            if num % i == 0 {
                return false;
            }
        }

        return true;
    }
}
