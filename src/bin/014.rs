use project_euler::seq::Collatz;

fn compute(n: usize) -> usize {
    if n == 1 {
        return 1;
    }

    let mut map = [0_u64; 1_000_000];
    map[1] = 1;

    (2..n)
        .max_by_key(|&n| compute_collatz(&mut map, n as u64))
        .unwrap()
}

fn compute_collatz(map: &mut [u64; 1_000_000], n: u64) -> u64 {
    if n < 1_000_000 && map[n as usize] > 0 {
        return map[n as usize];
    }

    let mut collatz = Collatz::new(n);
    let len = compute_collatz(map, collatz.next().unwrap()) + 1;
    
    if n < 1_000_000 {
        map[n as usize] = len;
    }

    return len
}

fn main() {
    project_euler::problem(|| -> String {
        compute(1_000_000).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn longest_collatz_under_1() {
        assert_eq!(1, compute(1))
    }
}

