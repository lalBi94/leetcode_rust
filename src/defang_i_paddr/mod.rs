pub fn solution(address: String) -> String {
    let mut stock: String = String::new();

    for e in address.chars() {
        if e == '.' {
            stock.push_str("[.]")
        } else {
            stock.push_str(&e.to_string());
        }
    }

    stock
}