use num_integer::Integer;

use project_euler::seq::Fibonacci;

fn compute(n: u64) -> u64 {
    Fibonacci::<u64>::new()
        .take_while(|&fib| fib < n)
        .filter(|&fib| fib.is_even())
        .sum::<u64>()
} 

fn main() {
    project_euler::problem(|| -> String {
        compute(4_000_000).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fibonacci_even_sum_under_10() {
        assert_eq!(10, compute(10))
    }
}
