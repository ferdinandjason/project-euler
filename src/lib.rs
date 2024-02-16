pub mod seq;
pub mod prime;
pub mod num;

use std::time::Instant;

pub fn problem(solver: impl Fn () -> String) {
    let time = Instant::now();
    let answer = solver();
    let duration = time.elapsed().as_micros();

    println!("{answer} in {duration} μs");
}
