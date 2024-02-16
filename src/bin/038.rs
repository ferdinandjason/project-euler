use bit_vec::BitVec;
use itertools::Itertools;
use project_euler::num;

fn compute() -> u64 {
    let mut digits = BitVec::from_elem(10, false);
    
    let nums = (1..=9).rev()
        .permutations(4)
        .find(|x| {
            let n = x.iter().fold(0_u64, |acc, elem| acc * 10 + elem);
            let n2 = n * 2;
            
            num::is_pandigital(n * 10_u64.pow(n2.ilog10() + 1) + n2, &mut digits)
        })
        .unwrap()
        .iter()
        .fold(0_u64, |acc, elem| acc * 10 + elem);
    let nums2 = nums * 2;
    
    nums * 10_u64.pow(nums2.ilog10() + 1) + nums2
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
    fn largest_pandigital_multiply() {
        assert_eq!(932718654, compute());
    }
}

