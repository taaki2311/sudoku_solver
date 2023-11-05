fn main() {
    let mut input_string = String::new();
    let mut total_bytes = 0;
    while total_bytes < 81 {
        total_bytes += std::io::stdin().read_line(&mut input_string).unwrap();
    }
    let mut input_chars = input_string.chars();

    let mut array = [[0; 9]; 9];
    for row in 0..9 {
        for column in 0..9 {
            let mut character = input_chars.next().unwrap();
            while character.is_whitespace() {
                character = input_chars.next().unwrap();
            }
            let number = character
                .to_digit(10)
                .unwrap_or_default()
                .try_into()
                .unwrap();
            array[row][column] = number;
        }
    }

    let mut board = sudoku_solver::board::Board::from(array);
    let outcome = board.solve();
    println!("\n{board}\n{outcome}");
}
