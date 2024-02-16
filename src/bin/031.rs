fn compute(amount: u32, len: usize) -> u32 {
    static COINS: [u32; 8] = [1, 2, 5, 10, 20, 50, 100, 200];
    match len {
        0 => 1,
        1..=7 => {
            let largest = COINS[len];
            let max_n = (amount / largest) + 1;

            (0..max_n)
                .map(|x| compute(amount - x * largest, len - 1))
                .sum()
        }
        _ => unreachable!(),
    }
}

fn main() {
    project_euler::problem(|| -> String {
        compute(200, 7).to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn ways_to_get_2p_using_other_coins() {
        assert_eq!(73682, compute(200, 7));
    }
}

