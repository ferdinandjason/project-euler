fn compute(n: u32) -> u32 {
    let s0 = ((n * (n + 1)) >> 1).pow(2);
    let s1 = (n * (n + 1) * (2 * n + 1)) / 6;

    s0 - s1
}

fn main() {
    project_euler::problem(|| -> String {
        compute(100).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn difference_sum_of_square_and_square_of_sum_under_10() {
        assert_eq!(2640, compute(10))
    }
}

