fn compute(n: u32) -> u32 {
    let r5 = 5f64.sqrt();
    let phi = (1.0 + r5) / 2.0;

    ((n as f64 + r5.log(10.0)) / phi.log(10.0)).ceil() as u32
}

fn main() {
    project_euler::problem(|| -> String {
        compute(999).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn fibonacci_digit_number() {
        assert_eq!(7, compute(1));
        assert_eq!(12, compute(2));
        assert_eq!(4782, compute(999));
    }
}

