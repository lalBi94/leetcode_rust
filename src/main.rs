fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    let yo = {
        leetcode_rust::defang_i_paddr::solution("1.1.1.1".to_string())
    };

    let elapsed: std::time::Duration = now.elapsed();
    println!("Results: {:?}, time {:.2?}", yo, elapsed);
}