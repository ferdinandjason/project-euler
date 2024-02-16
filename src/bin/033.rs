use std::ops::MulAssign;

use num_integer::Integer;

fn remove(n: u32, x: u32) -> (bool, u32) {
    if n % 10 == x {
        (true, n / 10)
    } else if n / 10 == x {
        (true, n % 10)
    } else {
        (false, n)
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct Fraction {
    a: u32,
    b: u32
}

impl Fraction {
    pub fn new(a: u32, b: u32) -> Self {
        Self { a , b }
    }

    pub fn simplify(&self) -> Self {
        let gcd = self.a.gcd(&self.b);
        Self { a: self.a / gcd, b: self.b / gcd }
    }
}

impl MulAssign<Fraction> for Fraction {
    fn mul_assign(&mut self, rhs: Fraction) {
        self.a *= rhs.a;
        self.b *= rhs.b;
    }
}

fn compute() -> u32 {
    let mut prod = Fraction::new(1, 1);

    for a in 10..100 {
        for b in (a + 1)..100 {
            let frac = Fraction::new(a, b).simplify();

            for i in 1..10 {
                let (can_a, aa) = remove(a, i);
                let (can_b, bb) = remove(b, i);

                if can_a && can_b {
                    let new_frac = Fraction::new(aa, bb).simplify();
                    if new_frac == frac {
                        prod *= new_frac;
                    }
                }
            }
        }
    }

    prod.simplify().b
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
    fn curious_fraction_product_denominator() {
        assert_eq!(100, compute());
    }
}

