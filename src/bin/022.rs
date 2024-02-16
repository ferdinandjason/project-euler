use std::{fs::File, io::{BufRead, BufReader}};

fn compute(file: &str) -> u32 {
    let mut names = BufReader::new(File::open(file).unwrap())
        .split(b',')
        .map(|name| name.unwrap())
        .collect::<Vec<_>>();

    names.sort();

    names
        .iter()
        .enumerate()
        .map(|(i, name)| {
            name.iter().skip(1).take(name.len() - 2).map(|c| (c - b'A' + 1) as u32).sum::<u32>() * (i + 1) as u32
        })
        .sum()
}

fn main() {
    project_euler::problem(|| -> String {
        compute("./input/0022_names.txt").to_string()
    })
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn total_scores_in_file() {
        assert_eq!(871198282, compute("./input/0022_names.txt"))
    }
}

