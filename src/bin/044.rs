fn compute() -> u64 {
    let (mut n, mut last) = (1, 1u64);
    let mut pentagons = vec![1];
    loop {
        n += 3;
        last += n;
        pentagons.push(last);

        if let Some(res) = pentagons.iter().find(|&&p| is_pentagon(last + p) && is_pentagon(last - p)) {
            return last - *res
        }
    }
}

fn is_pentagon(p: u64) -> bool {
    let x = (24.0 * p as f64 + 1.0).sqrt();
    if x != x.floor() {
        return false
    }

    x as u64 % 6 == 5
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
    fn diff_of_pentagons_number_sum_and_diff() {
        assert_eq!(5482660, compute());
    }
}

