//! The `adder` crate provides functions that add numbers to other numbers.
//!
//! # Examples
//!
//! ```
//! assert_eq!(4, adder::add_two(2));
//! ```

#![feature(globs)]

extern crate test;

/// This function adds two to its argument.
///
/// # Examples
///
/// ```
/// use adder::add_two;
///
/// assert_eq!(4, add_two(2));
/// ```
pub fn add_two(a: i32) -> i32 {
    a + 2
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use test;

    #[test]
    fn it_works() {
        assert_eq!(4, add_two(2));
    }

    #[bench]
    fn bench_add_two(b: &mut Bencher) {
        b.iter(|| add_two(2));
    }

    #[bench]
    fn bench_xor_1000_ints(b: &mut Bencher) {
        b.iter(|| {
            range(0u, 1000).fold(0, |old, new| old ^ new)
        });
    }

    #[bench]
    fn bench_xor_1000_ints_02(b: &mut Bencher) {
        b.iter(|| {
            let mut n = 1000_u32;

            test::black_box(&mut n); // pretend to modify `n`

            range(0, n).fold(0, |a, b| a ^ b)
        });
    }
}
