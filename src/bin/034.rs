fn generate_factorial(n: usize) -> Vec<u64> {
    let mut factorials = vec![1_u64; (n + 1) as usize];
    for i in 1..=n {
        factorials[i] = i as u64 * factorials[i - 1];
    }

    factorials
}

fn compute() -> u64 {
    let f = generate_factorial(10);

    let mut sums = 0;
    for i in 3..(f[8] + 600) {
        let (mut x, mut sum) = (i, 0);
        while x > 0 {
            sum += f[x as usize % 10];
            x /= 10;
        }

        if sum == i {
            sums += sum;
        }
    }
    
    sums
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
    fn curious_number_sum() {
        assert_eq!(40730, compute());
    }
}

