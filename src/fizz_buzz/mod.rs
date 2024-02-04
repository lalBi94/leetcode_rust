pub fn solution(n: i32) -> Vec<String> {
    let mut stock: Vec<String> = Vec::new();

    for i in 1..=n {
        if i%3 == 0 && i%5 == 0 {
            stock.push(String::from("FizzBuzz"));
        } else if i%3 == 0 {
            stock.push(String::from("Fizz"));
        } else if i%5 == 0 {
            stock.push(String::from("Buzz"));
        } else {
            stock.push(i.to_string());
        }
    }

    stock
}