use std::{fs::File, io::{BufRead, BufReader}, str};

fn is_triangle_number(n: u64) -> bool {
    let m = ((8.0 * n as f64 + 1.0).sqrt() - 1.0) / 2.0;
    m == m.floor()
}

fn compute(file: &str) -> u64 {
    BufReader::new(File::open(file).unwrap())
        .split(b',')
        .map(|word| word.unwrap())
        .map(|word| word.iter().skip(1).take(word.len() - 2).fold(0_u64, |acc, &c| acc + (c - b'A') as u64 + 1))
        .filter(|&word_value| is_triangle_number(word_value))
        .count() as u64
}

fn main() {
    project_euler::problem(|| -> String {
        compute("./input/0042_words.txt").to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn triangle_word_count() {
        assert_eq!(162, compute("./input/0042_words.txt"));
    }
}

