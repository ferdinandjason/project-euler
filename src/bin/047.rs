fn compute(c: usize) -> usize {
    const N: usize = 150_000;
    let mut prime_factor_counter = [0; N + 1];
    for i in 2..N {
        if prime_factor_counter[i] == 0 {
            prime_factor_counter[i] = 1;
            for j in ((i+i)..=N).step_by(i) {
                prime_factor_counter[j] += 1
            }
        }
    }

    let mut window_counter = vec![0; c + 1];
    for i in 0..(N - c) {
        if prime_factor_counter[i] <= c {
            window_counter[prime_factor_counter[i]] += 1;
        }

        if i >= c && prime_factor_counter[i - c] <= c {
            window_counter[prime_factor_counter[i - c]] -= 1;
        }
        
        if window_counter[c] == c {
            return i - (c - 1)
        }
    }

    0
}

fn main() {
    project_euler::problem(|| -> String {
        compute(4).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn n_consecutive_n_distinct_prime_factor() {
        assert_eq!(14, compute(2));
        assert_eq!(644, compute(3));
        assert_eq!(134043, compute(4));
    }
}

