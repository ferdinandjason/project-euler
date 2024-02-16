use bit_vec::BitVec;
use num_integer::Integer;
use num_traits::{FromPrimitive, One};

pub fn is_pandigital(mut num: u64, digits: &mut BitVec) -> bool {
    digits.clear();

    while num > 0 {
        let i = num % 10;
        if digits[i as usize] {
            return false
        }

        digits.set(i as usize, true);
        num /= 10;
    }
    digits.set(0, true);
    digits.all()
}

pub fn is_n_pandigital(mut num: u64, digits: &mut BitVec) -> bool {
    digits.clear();

    let n = num.ilog10() as usize + 1;

    while num > 0 {
        let i = num % 10;
        if digits[i as usize] {
            return false
        }

        digits.set(i as usize, true);
        num /= 10;
    }

    digits.set(0, true);
    digits.iter().take(n + 1).all(|x| x)
}

pub trait FastPow: Integer + FromPrimitive + Clone + Copy {
    fn fast_pow(&self, mut exp: u64, modulo: Self) -> Self {
        let mut res: Self = One::one();
        let mut base = self.clone();
        while exp > 0 {
            if exp & 1 == 1 {
                res = (res * base) % modulo;
            }
            base = (base * base) % modulo;
            exp >>= 1;
        }

        res
    }
}

impl FastPow for u64 {}
impl FastPow for u128 {}
