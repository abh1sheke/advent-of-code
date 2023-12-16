use std::time;

pub fn timed<T: core::fmt::Debug>(f: fn() -> T) {
    let start = time::Instant::now();
    let answer = f();
    let end = start.elapsed();
    println!("answer = {answer:?}");
    println!("took: {end:?}");
}
