fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    let yo = {
        leetcode_rust::fizz_buzz::solution(499979)
    };

    let elapsed: std::time::Duration = now.elapsed();
    println!("Results: {:?}, time {:.2?}", yo, elapsed);
}
