fn power_cache(n: u32) -> [u32; 10] {
    [
        0,
        1u32.pow(n),
        2u32.pow(n),
        3u32.pow(n),
        4u32.pow(n),
        5u32.pow(n),
        6u32.pow(n),
        7u32.pow(n),
        8u32.pow(n),
        9u32.pow(n),
    ]
}

struct IncreasingNumber {
    digits: Vec<u8>
}

impl IncreasingNumber {
    fn new(n: usize) -> Self {
        Self { digits: vec![0u8; n] }
    }

    fn to_u32(&self) -> u32 {
        let mut num = 0;
        for digit in self.digits.iter() {
            num = num * 10 + *digit as u32;
        }

        num
    }
}

impl Iterator for IncreasingNumber {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.digits.last().unwrap() < &9u8 {
            *self.digits.last_mut().unwrap() += 1;
        } else {
            if self.digits.iter().all(|x| x == &9) {
                return None
            }
            
            *self.digits.last_mut().unwrap() += 1;
            for i in (1..self.digits.len()).rev() {
                self.digits[i - 1] += self.digits[i] / 10;
                self.digits[i] %= 10;
            }

            for i in 1..self.digits.len() {
                if self.digits[i - 1] > self.digits[i] {
                    self.digits[i] = self.digits[i - 1];
                }
            }
        }

        Some(self.to_u32())
    }
}

fn digit_power_sum(mut n: u32, power_cache: &[u32; 10]) -> u32 {
    let mut power_sum = 0;

    while n > 0 {
        power_sum += power_cache[n as usize % 10];
        n /= 10;
    }

    power_sum
}

fn compute(power: u32) -> u32 {
    let power_cache = power_cache(power);
    let inc = IncreasingNumber::new(power as usize + 1);
    let mut sum = 0;

    for i in inc {
        let dps = digit_power_sum(i, &power_cache);
        if digit_power_sum(dps, &power_cache) == dps {
            sum += dps;
        }
    }
    
    sum - 1
}

fn main() {
    project_euler::problem(|| -> String {
        compute(5).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn digit_forth_sum() {
        assert_eq!(19316, compute(4));
    }
}

