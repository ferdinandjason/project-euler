use num_integer::Integer;

fn compute(min: u32, max: u32) -> u32 {
    let mut n = max;
    for i in min..max {
        n = n.lcm(&i);
    }

    n
}

fn main() {
    project_euler::problem(|| {
        compute(1, 20).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smallest_num_evenly_divisible_in_1_to_10() {
        assert_eq!(2520, compute(1, 10))
    }
}

