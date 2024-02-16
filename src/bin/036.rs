struct BinaryPalindromes {
    base: u64,
    duplicate_middle: bool
}

impl BinaryPalindromes {
    fn new(duplicate_middle: bool) -> Self {
        Self { base: 1, duplicate_middle }
    }
}

impl Iterator for BinaryPalindromes {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        let (mut base, mut num) = (self.base, self.base);
        if !self.duplicate_middle {
            base >>= 1;
        }

        while base > 0 {
            num <<= 1;
            num |= base & 1;

            base >>= 1;
        }

        self.base += 1;
        Some(num)
    }
}

fn is_palindrome(mut p: u64) -> bool {
    let mut q = 0;
    let r = p;
    while p != 0 {
        q = q * 10 + p % 10;
        p /= 10;
    }

    q == r
}


fn compute(n: u64) -> u64 {
    BinaryPalindromes::new(true)
        .take_while(|&x| x < n)
        .filter(|&x| is_palindrome(x))
        .chain(
            BinaryPalindromes::new(false)
                .take_while(|&x| x < n)
                .filter(|&x| is_palindrome(x))
        )
        .sum()
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
    fn double_base_palindrome_sum() {
        assert_eq!(1055, compute(600));
        assert_eq!(872187, compute(1_000_000));
    }
}

