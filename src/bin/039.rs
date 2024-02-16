use itertools::Itertools;
use num_integer::Roots;
use project_euler::seq::PythagoreanPrimitiveTriplet;

fn compute(limit: u64) -> usize {
    let mut perimeter_solution = [0; 1001];

    for m in 1..(limit.sqrt() + 1).div_ceil(2) {
        for (a, b, c) in PythagoreanPrimitiveTriplet::new(m) {
            let p = a + b + c;
            if p > limit {
                break;
            }

            for k in 1..=limit.div_ceil(p) {
                if k * p > limit {
                    break
                }

                perimeter_solution[(k * p) as usize] += 1;
            }
        }
    }

    perimeter_solution.iter().position_max().unwrap()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(1000).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn max_solution_of_pythagorean_triplet_perimeter() {
        assert_eq!(840, compute(1000));
    }
}

