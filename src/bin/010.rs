use project_euler::prime::Primes;

fn compute(n: usize) -> u64 {
    Primes::until(n).iter().sum::<u64>()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(2_000_000).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_prime_below_10() {
        assert_eq!(17, compute(10))
    }
}

