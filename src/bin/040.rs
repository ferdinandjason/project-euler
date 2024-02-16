fn champernownes_digit(nth: u32) -> u32 {
    if nth < 10 {
        nth
    } else {
        let mut n = nth;
        let mut dc = 1;
        let mut c = 9;
        while n > dc * c {
            n -= dc * c;

            dc += 1;
            c *= 10;
        }
        n -= 1;

        let mut num = n / dc + c / 9;
        let num_nth = n % dc;
        let mut nnth = 0;

        for _ in 0..(num.ilog10() + 1 - num_nth) {
            nnth = num % 10;
            num /= 10;
        }
        
        nnth
    }
}

fn compute() -> u32 {
    [1, 10, 100, 1000, 10_000, 100_000, 1_000_000].iter().map(|&i|  champernownes_digit(i)).product()
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
    fn champernownes_digit_nth_product() {
        assert_eq!(210, compute());
    }
}

