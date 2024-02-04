fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    let yo = {
        leet_ex::arrange_coin::solution(1804289383)
    };

    let elapsed: std::time::Duration = now.elapsed();
    println!("Results: {:?}, time {:.2?}", yo, elapsed);
}