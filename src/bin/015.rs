fn compute(n: usize) -> u64 {
    let mut routes = 1_u64;
    for i in 1..=n {
        routes *= (i + n) as u64;
        routes /= i as u64;
    }

    routes
}

fn main() {
    project_euler::problem(|| -> String {
        compute(20).to_string()
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lattice_routes_on_2_x_2() {
        assert_eq!(6, compute(2))
    }
}

