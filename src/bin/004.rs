use num_integer::Integer;

fn to_palindrome(n: u32) -> u32 {
    let mut m = n.clone();
    let mut x = n.clone();
    while m != 0 {
        x *= 10;
        x += m % 10;
        m /= 10;
    }

    x
}

fn compute(min: u32, max: u32) -> u32 {
    let r = min..=max;
    for num in (min..=max).rev() {
        let p = to_palindrome(num);
        for d in min..=max {
            if d * d > p {
                break;
            }

            let (div, rem) = p.div_rem(&d);
            if rem == 0 && &r.contains(&div) == &true {
                return p;
            }
        }
    }

    unreachable!()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(100, 999).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_palindrome_from_2_digit_product() {
        assert_eq!(9009, compute(10, 99))
    }
}

