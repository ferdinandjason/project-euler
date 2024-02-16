fn generate_factorial(n: usize) -> Vec<u64> {
    let mut factorials = vec![1_u64; (n + 1) as usize];
    for i in 1..=n {
        factorials[i] = i as u64 * factorials[i - 1];
    }

    factorials
}

fn compute(mut nth: u64, n: usize) -> String {
    let f = generate_factorial(n);
    let mut d = (0..n).collect::<Vec<usize>>();
    let mut ans = Vec::with_capacity(n as usize);

    while d.len() > 0 {
        let dn = d.len();
        let i = nth / f[dn - 1];

        ans.push(d[i as usize]);
        d.remove(i as usize);

        nth -= f[dn - 1] * i;
    }

    String::from_utf8(ans.iter().map(|&x| x as u8 + b'0').collect::<Vec<_>>()).unwrap()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(999_999, 10).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn lexicographic_permutation_test() {
        assert_eq!("012", compute(0, 3));
        assert_eq!("021", compute(1, 3));
        assert_eq!("102", compute(2, 3));
        assert_eq!("120", compute(3, 3));
        assert_eq!("201", compute(4, 3));
        assert_eq!("210", compute(5, 3));
    }

    #[test]
    fn lexicographic_permutation() {
        assert_eq!("2783915460", compute(999_999, 10));
    }
}

