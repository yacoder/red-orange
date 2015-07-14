// See the LICENSE file at the top-level directory of this distribution
// for information about copyright and licensing of this code.

extern crate bit_vec;
use self::bit_vec::BitVec;

/// Returns a bit vector representing the Sieve of Erathosphenes,
/// up to (but not including) `n`.
fn build_sieve(n: usize) -> BitVec {
   // https://en.wikipedia.org/wiki/Sieve_of_Eratosthenes
   
   let mut sieve = BitVec::from_elem(n, true);
   sieve.set(0, false);
   sieve.set(1, false);

   for i in 2 .. 1 + (n as f64).sqrt() as usize {
      if sieve[i] {
         let mut j = i*2;
         while j < n {
            sieve.set(j, false);
            j += i;
         }
      }
   }

   sieve
}

/// Returns a vector of prime numbers up to (but not including) `n`.
///
/// # Examples
///
/// ```rust
/// # use red_orange::numeric::get_primes;
///
/// assert_eq!(get_primes(2),  [2]);
/// assert_eq!(get_primes(3),  [2,3]);
/// assert_eq!(get_primes(4),  [2,3]);
/// assert_eq!(get_primes(5),  [2,3,5]);
/// assert_eq!(get_primes(20), [2,3,5,7,11,13,17,19]);
/// ```
pub fn get_primes(n: usize) -> Vec<u32> {
   let sieve = build_sieve(n+1);

   let pi = (n as f64) / (n as f64).ln();
   let mut result = Vec::with_capacity(pi as usize);

   for (index, flag) in (0..).zip(sieve.iter()) {
      if flag {
         result.push(index);
      }
   }

   result
}
