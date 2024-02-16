use bit_vec::BitVec;
use project_euler::prime::Primes;

fn is_circular(mut p: u64, is_prime: &BitVec) -> bool {
    let d = (p as f64).log(10.0) as u64 + 1;

    for _ in 0..d {
        let q = p % 10;
        
        p = q * 10u64.pow(d as u32 - 1) + (p / 10);
        if !is_prime[p as usize] {
            return false;
        }
    }

    true
}

fn compute(n: u32) -> usize {
    let is_prime = Primes::from_sieve(n as usize);
    let mut primes = Vec::new();

    primes.push(2);
    for i in (3..=n).step_by(2) {
        if is_prime.get(i as usize).unwrap() {
            primes.push(i as u64);
        }
    }

    primes.iter().filter(|&&p| is_circular(p, &is_prime)).count()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(1_000_000).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn curious_number_sum() {
        assert_eq!(13, compute(100));
        assert_eq!(55, compute(1_000_000));
    }
}

