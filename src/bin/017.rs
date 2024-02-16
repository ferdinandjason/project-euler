fn to_under_20(n: u16) -> String {
    match n {
        1 => "one",
        2 => "two",
        3 => "three",
        4 => "four",
        5 => "five",
        6 => "six",
        7 => "seven",
        8 => "eight",
        9 => "nine",
        10 => "ten",
        11 => "eleven",
        12 => "twelve",
        13 => "thirteen",
        14 => "fourteen",
        15 => "fifteen",
        16 => "sixteen",
        17 => "seventeen",
        18 => "eighteen",
        19 => "nineteen",
        _ => "",
    }.to_string()
}

fn to_under_100(n: u16) -> String {
    let tens = n / 10;
    if tens < 2 {
        return to_under_20(n);
    }
    let s = match tens {
        2 => "twenty",
        3 => "thirty",
        4 => "forty",
        5 => "fifty",
        6 => "sixty",
        7 => "seventy",
        8 => "eighty",
        9 => "ninety",
        _ => "",
    };

    let ones = n % 10;

    match ones {
        0 => format!("{}", s),
        _ => format!("{} {}", s, to_under_20(ones)),
    }
}

fn to_under_1001(n: u16) -> String {
    if n == 1000 {
        return "one thousand".to_string();
    }

    let hundreds = n / 100;
    let rest = n % 100;
    if hundreds > 0 {
        let hundred_string = to_under_20(hundreds);
        if rest > 0 {
            let rest_string = to_under_100(rest);
            return format!("{} hundred and {}", hundred_string, rest_string);
        } else {
            return format!("{} hundred", hundred_string);
        }
    } else {
        return to_under_100(rest);
    }
}

fn compute(n: u16) -> usize {
    (1..=n).map(|n| {
        to_under_1001(n)
            .chars()
            .filter(|c| c.is_alphabetic())
            .count()
    }).sum()
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
    fn number_letter_count_under_5() {
        assert_eq!(19, compute(5))
    }

    #[test]
    fn number_letter_count_under_1000() {
        assert_eq!(21124, compute(1000))
    }
}

