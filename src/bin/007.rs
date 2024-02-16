use project_euler::prime::Primes;

fn compute(n: usize) -> u64 {
    *Primes::until(105_000).iter().nth(n - 1).unwrap()
}

fn main() {
    project_euler::problem(|| {
        compute(10_001).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prime_6th() {
        assert_eq!(13, compute(6))
    }
}

