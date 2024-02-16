use project_euler::num::FastPow;

fn compute(n: u128) -> u128 {
    let modulo = 10_000_000_000u128;
    (1u128..=n)
        .map(|i| i.fast_pow(i as u64, modulo))
        .fold(0u128, |acc, self_pow| (acc + self_pow) % modulo)
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
    fn self_power_sum() {
        assert_eq!(405071317, compute(10));
        assert_eq!(9110846700, compute(1000));
    }
}
