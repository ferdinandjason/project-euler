use num_bigint::BigUint;
use num_integer::Integer;
use num_traits::One;

fn compute(mut n: u32) -> u32 {
    let mut result = BigUint::one();
    let mut base = BigUint::one() + BigUint::one();
    while n > 0 {
        if n.is_odd() {
            result *= &base;
        }

        base = base.pow(2);
        n /= 2;
    }

    result.to_string().bytes().map(|x| (x - b'0') as u32).sum()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(1000).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_digit_sum_on_2_15() {
        assert_eq!(26, compute(15))
    }

    #[test]
    fn power_digit_sum_on_2_1000() {
        assert_eq!(1366, compute(1000))
    }
}

