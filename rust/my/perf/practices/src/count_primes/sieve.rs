// Sieve of Eratosthenes
//
// Find a prime, cross out all its multiples. What's never crossed out is prime.
//
// Time: O(n log n).
//       A number i crosses out about n/i multiples.
//       Only primes do it:   n/2 + n/3 + n/5 + n/7 + ...
//                          = n * (1/2 + 1/3 + 1/5 + 1/7 + ...)
//       If every i did it:   n/2 + n/3 + n/4 + n/5 + ...
//                          = n * (1/2 + 1/3 + 1/4 + 1/5 + ...)
//                          = n * (harmonic sum - 1), about n log n.
//       The second sum has extra terms (1/4, 1/6, ...), so it is bigger:
//       real work is at most that, which makes n log n an upper bound.
//
// Space: O(n) for the bool array.

pub struct Solution;

impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut cnt: i32 = 0;

        // Store is_composite for every i
        // True if i is a composite, thus not prime
        // False mean i is non a composite, thus prime
        // By default is false, thus if not marked before
        // the i would be prime
        let mut sieve = vec![false; n];

        // Skip zero and one as not prime
        for i in 2..n {
            // If prime
            if sieve[i] == false {
                cnt += 1;
                // For a prime i we start crossing out at i * i
                // instead of 2 * i. Everything below i * i was already crossed out
                // by smaller primes. Some repeats remain (e.g. 30 by 2, 3 and 5), which is harmless.
                let mut j = i * i;
                while j < n {
                    // Mark as composite
                    sieve[j] = true;
                    j += i;
                }
            }
        }

        cnt
    }
}

/*

[_, _, f, f, t, f, t, f, t, t, t ]
 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10

c = 1

*/
