const TRIANGLE: &str = "75
95 64
17 47 82
18 35 87 10
20 04 82 47 65
19 01 23 75 03 34
88 02 77 73 07 63 67
99 65 04 28 06 16 70 92
41 41 26 56 83 40 80 70 33
41 48 72 33 47 32 37 16 94 29
53 71 44 65 25 43 91 52 97 51 14
70 11 33 28 77 73 17 78 39 68 17 57
91 71 52 38 17 14 91 43 58 50 27 29 48
63 66 04 68 89 53 67 30 73 16 69 87 40 31
04 62 98 27 23 09 70 98 73 93 38 53 60 04 23";

fn compute(input: &str) -> u32 {
    let triangle = input
        .lines()
        .map(|line| line.split_ascii_whitespace().map(|x| x.parse::<u32>().unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut max = triangle.clone();

    for i in 1..triangle.len() {
        for j in 0..triangle[i].len() {
            max[i][j] = u32::MIN;

            if j > 0 {
                max[i][j] = max[i][j].max(max[i - 1][j - 1] + triangle[i][j]);
            }
            if j < max[i - 1].len() {
                max[i][j] = max[i][j].max(max[i - 1][j] + triangle[i][j]);
            }
        }
    }

    *max.last().unwrap().iter().max().unwrap()
}

fn main() {
    project_euler::problem(|| -> String {
        compute(TRIANGLE).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    const TRIANGLE_TEST: &str = "3
7 4
2 4 6
8 5 9 3";

    #[test]
    fn max_path_sum_test() {
        assert_eq!(23, compute(TRIANGLE_TEST))
    }

    #[test]
    fn max_path_sum() {
        assert_eq!(1074, compute(TRIANGLE))
    }
}

