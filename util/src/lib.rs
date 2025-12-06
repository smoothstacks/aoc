pub use euclid;
pub use glam;

pub mod math {
    use num::Unsigned;

    pub fn num_digits<T>(mut n: T) -> usize
    where
        T: num::PrimInt + Unsigned,
    {
        let ten = T::from(10u8).expect("primitive ints can be created from u8");

        let mut count = 0;
        while n > num::zero() {
            n = n / ten;
            count += 1;
        }

        count
    }

    pub fn digit_at<T>(n: T, at: usize) -> T
    where
        T: num::PrimInt + Unsigned,
    {
        let ten = T::from(10u8).expect("primitive ints can be created from u8");
        (n / ten.pow(at as u32)) % ten
    }

    pub fn split_num_at<T>(n: T, at: usize) -> (T, T)
    where
        T: num::PrimInt + Unsigned,
    {
        let ten = T::from(10u8).expect("primitive ints can be created from u8");
        let pow = ten.pow(at as u32);
        let right = n % pow;
        let left = (n - right) / pow;
        (left, right)
    }

    /// Treats the incoming iterator as a list of digits, and create a number from them
    /// Assumes that all entries in the iterator are <= 9
    pub fn num_from_iter<T>(iter: impl Iterator<Item = u8>) -> T
    where
        T: num::PrimInt + Unsigned + From<u8>,
    {
        let mut total = T::zero();
        let mut pow = T::one();

        let ten = <T as From<u8>>::from(10u8);

        for digit in iter {
            total = total + <T as From<u8>>::from(digit) * pow;
            pow = pow * ten;
        }
        total
    }

    #[cfg(test)]
    mod tests {
        #[test]
        fn num_digits() {
            assert_eq!(super::num_digits(999u64), 3);
            assert_eq!(super::num_digits(999u32), 3);
            assert_eq!(super::num_digits(999u16), 3);
            assert_eq!(super::num_digits(111u8), 3);
            assert_eq!(super::num_digits(8291469824u64), 10);
        }

        #[test]
        fn split_num_at() {
            assert_eq!(super::split_num_at(123456u32, 3), (123, 456));
            assert_eq!(super::split_num_at(0u32, 3), (0, 0));
            assert_eq!(super::split_num_at(10u32, 1), (1, 0));
            assert_eq!(super::split_num_at(1000u32, 2), (10, 0));
        }

        #[test]
        fn digit_at() {
            let all: u32 = 987654321;
            for i in 0..9 {
                assert_eq!(super::digit_at(all, i), (i + 1) as u32);
            }
            let all: u16 = 54321;
            for i in 0..5 {
                assert_eq!(super::digit_at(all, i), (i + 1) as u16);
            }
        }
    }
}

pub mod parse {
    use std::str::FromStr;

    pub use nom;
    use nom::{AsChar, IResult, Input, Parser, character::complete::digit0, combinator::map_res};

    pub fn parse_num<I, T>(input: I) -> IResult<I, T>
    where
        I: Input + AsRef<str>,
        <I as Input>::Item: AsChar,
        T: FromStr,
    {
        map_res(digit0, |s: I| s.as_ref().parse::<T>()).parse(input)
    }
}

pub mod grid;
