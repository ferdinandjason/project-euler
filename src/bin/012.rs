use project_euler::prime::*;

fn compute(max: u64) -> u64 {
    let primes = Primes::until(450);

    let mut n = 1_u64;
    loop {
        let t = (n * (n + 1)) >> 1;
        if t.num_of_divisor(&primes) > max {
            break t;
        } 
        n += 1;
    }
}

fn main() {
    project_euler::problem(|| -> String {
        compute(500).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn first_triangle_number_has_5_divisor() {
        assert_eq!(28, compute(5))
    }
}

