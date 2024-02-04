fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    let yo = {
        leetcode_rust::my_pow::solution(250000.5, 5)
    };

    let elapsed: std::time::Duration = now.elapsed();
    println!("Results: {:?}, time {:.2?}", yo, elapsed);
}