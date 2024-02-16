use project_euler::prime::*;

fn compute(n: u64) -> u64 {
    n.factorize().last().unwrap().0
}

fn main() {
    project_euler::problem(|| -> String {
        compute(600851475143).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_prime_factor_of_13195() {
        assert_eq!(29, compute(13195))
    }
}

