use project_euler::prime::Primes;

fn compute() -> i32 {
    let is_prime = Primes::from_sieve(100_000);
    let (mut max_prime, mut aa, mut bb) = (0, -1000, 0);

    for b in 0..1001 {
        if is_prime.get(b as usize).unwrap() {
            for a in (-1000_i32)..1001 {
                let (mut n, mut c) = (0, 0);
                let mut p;
                loop {
                    p = n * n + a * n + b;
                    if p > 1 && is_prime.get(p as usize).unwrap() {
                        c += 1;
                        n += 1;
                    } else {
                        break;
                    }
                }

                if c > max_prime {
                    max_prime = c;
                    aa = a;
                    bb = b;
                }
            }
        }
    }

    aa * bb
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
    fn max_coefficient_product_of_quadratic_prime() {
        assert_eq!(-59231, compute());
    }
}

