use num_bigint::BigUint;
use num_traits::One;

fn compute(n: u32) -> u32 {
    let mut factorial = BigUint::one();
    for i in 2_u32..n {
        factorial *= i;
    }

    factorial.to_string().bytes().map(|x| (x - b'0') as u32).sum()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(100).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_digit_sum() {
        assert_eq!(27, compute(10))
    }
}

