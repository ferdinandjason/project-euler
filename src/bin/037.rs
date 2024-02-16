use project_euler::prime::Primes;

struct NumberTruncator {
    base: u64,
    left: bool,
    len: u32
}

impl NumberTruncator {
    fn new(base: u64, left: bool) -> Self {
        Self { base, left, len: base.ilog10() }
    }
}

impl Iterator for NumberTruncator {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        if self.left {
            self.base %= 10_u64.pow(self.len)
        } else {
            self.base /= 10;
        };

        self.len -= 1;
        
        if (self.len + 1) == 0 {
            None
        } else {
            Some(self.base)
        }
    }
}

fn compute() -> u64 {
    let (prime, is_prime) = Primes::until_with_sieve(750_000);

    prime.iter()
        .skip_while(|&&x| x < 10)
        .filter(|&&x| {
            NumberTruncator::new(x, true).all(|x| is_prime.get(x as usize).unwrap()) &&
            NumberTruncator::new(x, false).all(|x| is_prime.get(x as usize).unwrap())
        })
        .sum()
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
    fn truncatable_prime_sum() {
        assert_eq!(748317, compute());
    }
}

