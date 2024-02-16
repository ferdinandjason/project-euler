use bit_vec::BitVec;
use project_euler::prime::*;

fn compute() -> u64 {
    let n = 28124_u64;
    let mut is_abundant = BitVec::from_elem(n as usize, false);
    let mut abundant = Vec::new();

    for i in 0..n {
        let d = i.sum_of_divisor() - i;
        if i < d {
            is_abundant.set(i as usize, true);
            abundant.push(i);
        }
    }

    let mut sum = 0;
    for i in 0..n {
        let mut is_two_abundant_sum = false;
        for a in abundant.iter() {
            if a > &(i / 2 + 1) {
                break;
            }

            let b = i - a;
            if is_abundant.get(b as usize).unwrap() {
                is_two_abundant_sum = true;
                break;
            }
        }

        if !is_two_abundant_sum {
            sum += i;
        }
    }

    sum
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
    fn non_two_abundant_sum() {
        assert_eq!(4179871, compute())
    }
}

