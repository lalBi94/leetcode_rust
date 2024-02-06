pub fn solution(matrix: &mut Vec<Vec<i32>>) -> () {
    if matrix.len() == 0 { return; }
    
    matrix.reverse();
    let mut stock: Vec<Vec<i32>> = Vec::new();
    let mut cursor_x: usize = 0;
    let mut cursor_y: usize = 0;
    let mut indexer: usize = 0;

    for _ in matrix.iter() {
        stock.push(vec![]);
    }

    for _ in 0..(matrix.len()*matrix.len()) {
        stock[indexer].push(matrix[cursor_x][cursor_y]);

        if cursor_x == matrix.len()-1 {
            cursor_x = 0;
            cursor_y += 1;
            indexer += 1;
        } else {
            cursor_x += 1;
        }
    }

    *matrix = stock.clone();
}