fn compute(n: u32) -> u32 {
    let mut sum = 1;
    let (mut num, mut i) = (1, 0);
    let mut step;

    while num < n * n {
        step = (i / 4 + 1) << 1;

        i += 1;
        num += step;
        sum += num;
    }

    sum
}

fn main() {
    project_euler::problem(|| -> String {
        compute(1001).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn sum_of_spiral_diagonal() {
        assert_eq!(101, compute(5));
    }
}

