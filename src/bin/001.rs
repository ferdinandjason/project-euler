fn compute(n: u32) -> u32 {
    (1..n).filter(|i| i % 3 == 0 || i % 5 == 0).sum()
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
    fn sum_under_10() {
        assert_eq!(23, compute(10))
    }
}
