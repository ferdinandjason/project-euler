use itertools::Itertools;
use project_euler::prime::Primes;

fn compute() -> u64 {
    let prime = Primes::until(3000);

    (1..=7).rev()
        .permutations(7)
        .map(|x| x.iter().fold(0_u64, |acc, elem| acc * 10 + elem))
        .find(|&x| prime.check(x))
        .unwrap()
}

fn main() {
    project_euler::problem(|| -> String {
        compute().to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn largest_n_pandigital_prime() {
        assert_eq!(7652413, compute());
    }
}

