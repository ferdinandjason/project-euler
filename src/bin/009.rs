fn compute(n: usize) -> u64 {
    for a in 2..n {
        let b_numerator = ((n * n) >> 1) - n * a;
        let b_denominator = n - a;

        if b_numerator % b_denominator == 0 {
            let b = b_numerator / b_denominator;
            let c = n - a - b;
            return (a * b * c) as u64;
        }
    }

    unreachable!()
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
    fn pythagorean_triplet_sum_product() {
        assert_eq!(60, compute(12))
    }
}

