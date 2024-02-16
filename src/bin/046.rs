use project_euler::prime::Primes;

fn compute() -> u64 {
    let primes = Primes::until(4000);
    let mut num = 33;
    loop {
        num += 2;
        if primes.check(num) {
            continue
        }

        let limit = (((num - 2) >> 1) as f64).sqrt() as u64 + 1;
        if (0..limit).map(|i| primes.check(num - 2 * i * i)).find(|&b| b).is_none() {
            return num;
        }
    }
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
    fn goldback_other_conjecture() {
        assert_eq!(5777, compute());
    }
}

