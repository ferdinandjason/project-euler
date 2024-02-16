use project_euler::prime::*;

fn compute(max: u64) -> u64 {
    let sum_of_div = (0..max).map(|num| num.sum_of_divisor() - num).collect::<Vec<_>>();

    sum_of_div
        .iter()
        .cloned()
        .enumerate()
        .filter(|&(n, d)| d < max as u64 && n as u64 != d)
        .filter(|&(n, d)| sum_of_div[d as usize] == n as u64)
        .map(|(_, d)| d)
        .sum()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(10000).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_of_amicable_number_under_10000() {
        assert_eq!(31626, compute(10000))
    }
}

