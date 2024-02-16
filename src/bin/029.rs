use std::collections::HashSet;

use project_euler::prime::Factorize;

fn compute(n: u32) -> usize {
    let mut sequence = HashSet::new();

    for a in 2..n {
        let prime_representation = (a as u64).factorize();
        for b in 2..n {
            let mut new = prime_representation.clone();
            new.iter_mut().for_each(|p| p.1 *= b);

            sequence.insert(new);
        }
    }

    sequence.len()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(101).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn distinct_power_of_ab() {
        assert_eq!(15, compute(6));
    }
}

