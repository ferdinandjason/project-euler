use project_euler::prime::Primes;

fn compute(n: u64, limit: u64) -> u64 {
    let primes = Primes::until(n as usize);
    let mut ps = vec![0];
    
    ps.append(
        &mut primes.iter().scan(0, |acc, &x| {
            *acc = *acc + x; 
            Some(*acc)
        }).collect::<Vec<_>>()
    );

    let (mut i, mut j) = (0, ps.len() - 1);
    let (mut cmax, mut max) = (0, 0);
    while j > 0 {
        let sum = ps[j] - ps[i];
        if primes.check(sum) && sum <= limit {
            if j - i > cmax {
                cmax = j - i;
                max = sum;
            }
        }

        if i < j { i += 1; } else { j -= 1; i = 0; }
    }

    max
}

fn main() {
    project_euler::problem(|| -> String {
        compute(4000, 1_000_000).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn consecutive_prime_sum_under_n() {
        assert_eq!(41, compute(100, 100));
        assert_eq!(961, compute(100, 1000));
        assert_eq!(997651, compute(4000, 1_000_000));
    }
}
