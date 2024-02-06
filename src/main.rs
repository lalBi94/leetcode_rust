fn main() {
    let now: std::time::Instant = std::time::Instant::now();

    let yo = {
        let mut coucou: Vec<Vec<i32>> = vec![
            vec![1,2,3],
            vec![4,5,6],
            vec![7,8,9],
        ];

        leetcode_rust::rotate::solution(&mut coucou);

        println!("{:?}", coucou);
    };

    let elapsed: std::time::Duration = now.elapsed();
    println!("Results: {:?}, time {:.2?}", yo, elapsed);
}
