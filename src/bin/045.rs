fn compute() -> u64 {
    let mut i = 286;
    let mut tri;
    loop {
        tri = (i * i + i) >> 1;
        if is_triangular(tri * 3) && i % 2 == 1 && is_pentagon(tri) {
            return tri
        }

        i += 1;
    }
}

fn is_triangular(p: u64) -> bool {
    let x = (8.0 * p as f64 + 1.0).sqrt();
    x == x.floor()
}

fn is_pentagon(p: u64) -> bool {
    let x = (24.0 * p as f64 + 1.0).sqrt();
    if x != x.floor() {
        return false
    }

    x as u64 % 6 == 5
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
    fn triangular_pentagonal_hexagonal() {
        assert_eq!(1533776805, compute());
    }
}

