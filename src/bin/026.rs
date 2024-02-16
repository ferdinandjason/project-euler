fn get_cycle_len(n: u32, mut reminder: [u32; 1000]) -> u32 {
    reminder.fill(0);
    let mut num = 1;
    let mut index = 0;
    let mut rem;
    loop {
        rem = num % n;
        match reminder[rem as usize] {
            0 => reminder[rem as usize] = index,
            last =>  break index - last
        }

        num = rem * 10;
        index += 1;
    }
}

fn compute(n: u32) -> u32 {
    let reminder = [0; 1000];
    (2..n).max_by_key(|x| get_cycle_len(*x, reminder)).unwrap()
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
    fn reciprocal_cycle_max_len() {
        assert_eq!(7, compute(10));
    }
}

