fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    let yo = {
        leetcode_rust::remove_element::solution(&mut vec![0,1,2,2,3,0,4,2], 2)
    };

    let elapsed: std::time::Duration = now.elapsed();
    println!("Results: {:?}, time {:.2?}", yo, elapsed);
}
