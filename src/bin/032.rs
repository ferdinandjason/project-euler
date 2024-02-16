use std::collections::HashSet;
use project_euler::num;

use bit_vec::BitVec;

fn compute() -> u64 {
    let mut pan_product = HashSet::new();
    let mut digits = BitVec::from_elem(10, false);

    for a in 10..100 {
        for b in ((99 + a % 3)..1000).step_by(3) {
            if b < 100 {
                continue
            }

            let c = a * b;
            if c > 9999 {
                break;
            }
            if c < 1000 || c % 3 != a % 3 {
                continue;
            }

            if num::is_pandigital(a * 10000000 + b * 10000 + c, &mut digits) {
                pan_product.insert(c);
            }
        }
    }

    for a in 1..10 {
        for b in ((999 + a % 3)..10000).step_by(3) {
            if b < 1000 {
                continue
            }

            let c = a * b;
            if c > 9999 {
                break;
            }
            if c < 1000 || c % 3 != a % 3 {
                continue;
            }

            if num::is_pandigital(a * 100000000 + b * 10000 + c, &mut digits) {
                pan_product.insert(c);
            }
        }
    }

    pan_product.iter().sum()
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
    fn pan_product_all() {
        assert_eq!(45228, compute());
    }
}

