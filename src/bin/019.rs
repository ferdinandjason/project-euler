fn is_leap_year(y: u32) -> bool {
    if y % 400 == 0 {
        true
    } else if y % 100 == 0 {
        false
    } else if y % 4 == 0 {
        true
    } else {
        false
    }
}

fn day_of_year(y: u32) -> u32 {
    if is_leap_year(y) {
        366
    } else {
        365
    }
}

fn day_of_month(y: u32) -> [u32; 12] {
    [
        31,                                    // Jan
        if is_leap_year(y) { 29 } else { 28 }, // Feb
        31,                                    // Mar
        30,                                    // Apr
        31,                                    // May
        30,                                    // Jun
        31,                                    // Jul
        31,                                    // Aug
        30,                                    // Sep
        31,                                    // Oct
        30,                                    // Nov
        31,                                    /* Dec */
    ]
}

fn compute() -> u32 {
    let mut week = [0; 7];
    let mut day = 1;            // Monday
    day = (day + day_of_year(1900)) % 7;
    for year in 1901..2001 {
        for offset in day_of_month(year) {
            week[day as usize] += 1;
            day = (day + offset) % 7;
        }
    }

    week[0]
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
    fn sunday_on_first_day_of_the_month() {
        assert_eq!(171, compute())
    }
}

