// See the LICENSE file at the top-level directory of this distribution
// for information about copyright and licensing of this code.

use std::ops::Rem;

extern crate num;
use self::num::Zero;

/// Returns greatest common divisor of 2 given numbers.
///
/// # Examples
///
/// ```rust
/// # use red_orange::numeric::gcd;
///
/// assert_eq!(1, gcd(1, 1));
/// 
/// assert_eq!(1, gcd(2, 1));
/// assert_eq!(1, gcd(1, 2));
/// 
/// assert_eq!(2, gcd(2, 2));
///
/// assert_eq!(3, gcd(6, 9));
/// assert_eq!(3, gcd(9, 6));
///
/// assert_eq!(21, gcd(252, 105));
/// assert_eq!(21, gcd(105, 252));
/// ```
pub fn gcd<T: Copy + Zero + PartialOrd + Rem<Output=T>>(mut a : T, mut b : T) -> T
{
   assert!(a > T::zero());
   assert!(b > T::zero());

   while b > T::zero() {
      let t = b;
      b = a % b; 
      a = t;
   }

   a
}
