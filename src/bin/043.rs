use bit_vec::BitVec;
use itertools::Itertools;

fn generate_3pan_div(div: u64) -> Vec<u64> {
    (0..10)
        .permutations(3)
        .map(|x| x.iter().fold(0, |acc, x| acc * 10 + x))
        .filter(|x| x % div == 0)
        .collect_vec()
}

fn is_pandigital_and_fill_one(mut num: u64, digits: &mut BitVec) -> (bool, u64) {
    let num_temp = num;
    digits.clear();


    for _ in 0..9 {
        let i = num % 10;
        if digits[i as usize] {
            return (false, 0)
        }

        digits.set(i as usize, true);
        num /= 10;
    }

    let missing = digits.iter().position(|b| !b).unwrap() as u64;

    (true, missing * 10u64.pow(9) + num_temp)
}

fn compute() -> u64 {
    let primes = [17, 13, 11, 7, 5, 3, 2];
    let pan_primes = primes.iter().map(|&p| generate_3pan_div(p)).collect::<Vec<_>>();
    let mut combined_pan_primes = pan_primes.first().unwrap().to_owned();

    for (i, _) in primes.iter().enumerate() {
        if i == 0 {
            continue
        }

        let mult = 10u64.pow(i as u32);
        let mut combined_pan = Vec::new();
        for pan in pan_primes.get(i).unwrap() {
            for combined in &combined_pan_primes {
                if combined / mult == pan % 100 {
                    combined_pan.push(pan * mult + combined % mult);
                }
            }
        }

        combined_pan_primes = combined_pan;
    }

    let mut digits = BitVec::from_elem(10, false);
    let mut sum = 0;
    for pan in combined_pan_primes.iter() {
        let (is_pandigital, new_pan) = is_pandigital_and_fill_one(*pan, &mut digits);
        
        if is_pandigital {
            sum += new_pan
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
    fn pandigital_sub_string_divisibility() {
        assert_eq!(16695334890, compute());
    }
}

