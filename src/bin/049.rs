use project_euler::prime::Primes;

fn unique_digit_mask(mut n: u64) -> u64 {
    let mut mask = 0;
    while n != 0 {
        mask |= 1 << (n % 10);
        n /= 10;
    }

    mask
}

fn compute() -> u64 {
    let primes = Primes::until(9999);
    let d = 3330;

    let pp = primes.iter()
        .skip_while(|&&p| p < 1000)
        .take_while(|&&p| p <= 9999 - 2 * d)
        .filter(|&&p| p != 1487)
        .map(|p| (p, p + d, p + d + d))
        .filter(|(_, q, r)| primes.check(*q) && primes.check(*r))
        .find(|(p, q, r)| unique_digit_mask(**p) == unique_digit_mask(*q) && unique_digit_mask(*r) == unique_digit_mask(*q))
        .unwrap();

    pp.0 * 100000000 + pp.1 * 10000 + pp.2
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
    fn prime_permutations() {
        assert_eq!(296962999629, compute());
    }
}
