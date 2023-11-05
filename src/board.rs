use crate::position::Position;

#[derive(Clone, Copy)]
enum Line {
    Row(usize),
    Column(usize),
}

#[derive(Clone, Copy, PartialEq)]
pub struct Board([[Position; 9]; 9]);

impl Board {
    fn new() -> Board {
        let position = Position::new();
        Board([[position; 9]; 9])
    }

    pub fn from(array: [[u8; 9]; 9]) -> Board {
        let mut board = Self::new();
        for i in 0..9 {
            for ii in 0..9 {
                let value = array[i][ii];
                if value != 0 {
                    board.set_final_value(value, i, ii);
                }
            }
        }
        board
    }

    fn set_final_value(&mut self, final_value: u8, x: usize, y: usize) {
        self.0[x][y].set_final_value(final_value);
    }

    fn add_impossibilities(&mut self, impossibilities: u16, x: usize, y: usize) {
        self.0[x][y].add_impossibilities(impossibilities);
    }

    fn get_position_xy(&self, x: usize, y: usize) -> Position {
        self.0[x][y]
    }

    fn get_position_line(&self, line: Line, i: usize) -> Position {
        match line {
            Line::Row(index) => self.get_position_xy(index, i),
            Line::Column(index) => self.get_position_xy(i, index),
        }
    }

    fn get_line_final_values(&self, line: Line) -> u16 {
        let mut impossibilities = 0;
        for i in 0..9 {
            let position = self.get_position_line(line, i);
            let value = position.final_value;
            if value != 0 {
                impossibilities |= 1 << value;
            }
        }
        impossibilities
    }

    fn get_box_final_values(&self, x: usize, y: usize) -> u16 {
        let mut impossibilities = 0;
        for i in 0..3 {
            for ii in 0..3 {
                let value = self.get_position_xy(x + i, y + ii).final_value;
                if value != 0 {
                    impossibilities |= 1 << value;
                }
            }
        }
        impossibilities
    }

    fn annotate_line(&mut self, line: Line) {
        let impossibilities = self.get_line_final_values(line);
        for i in 0..9 {
            match line {
                Line::Row(index) => self.add_impossibilities(impossibilities, index, i),
                Line::Column(index) => self.add_impossibilities(impossibilities, i, index),
            }
        }
    }

    fn annotate_box(&mut self, x: usize, y: usize) {
        let impossibilities = self.get_box_final_values(x, y);
        for i in 0..3 {
            for ii in 0..3 {
                self.add_impossibilities(impossibilities, x + i, y + ii);
            }
        }
    }

    fn annotate_all_lines(&mut self) {
        for index in 0..9 {
            self.annotate_line(Line::Row(index));
            self.annotate_line(Line::Column(index));
        }
    }

    fn annotate_all_box(&mut self) {
        for i in 0..3 {
            for ii in 0..3 {
                self.annotate_box(3 * i, 3 * ii);
            }
        }
    }

    fn get_count_in_line(&self, line: Line) -> [u8; 9] {
        let mut array = [0; 9];
        for i in 0..9 {
            let position = self.get_position_line(line, i);
            if position.final_value == 0 {
                let non_possibility = position.impossibilities;
                for ii in 0..9 {
                    if ((non_possibility >> (ii + 1)) & 1) == 0 {
                        array[ii] += 1;
                    }
                }
            }
        }
        array
    }

    fn get_count_in_box(&self, x: usize, y: usize) -> [u8; 9] {
        let mut array = [0; 9];
        for i in 0..3 {
            for ii in 0..3 {
                let position = self.get_position_xy(x + i, y + ii);
                if position.final_value == 0 {
                    let impossibilities = position.impossibilities;
                    for iii in 0..9 {
                        if (impossibilities >> (iii + 1) & 1) == 0 {
                            array[iii] += 1;
                        }
                    }
                }
            }
        }
        array
    }

    fn check_line_possibilities(&mut self, line: Line) -> bool {
        let mut change_happened = false;
        let counts = self.get_count_in_line(line);
        for i in 0..9u8 {
            if counts[usize::from(i)] == 1 {
                for ii in 0..9 {
                    let position = self.get_position_line(line, i.into());
                    let non_possibility = position.impossibilities;
                    if ((non_possibility >> (i + 1)) & 1) == 0 {
                        let final_value = i + 1;
                        match line {
                            Line::Row(index) => self.set_final_value(final_value, index, ii),
                            Line::Column(index) => self.set_final_value(final_value, ii, index),
                        }
                        change_happened = true;
                    }
                }
            }
        }
        change_happened
    }

    fn check_box_possibilities(&mut self, x: usize, y: usize) -> bool {
        let mut change_happened = false;
        let counts = self.get_count_in_box(x, y);
        for i in 0..9 {
            if counts[usize::from(i)] == 1 {
                for ii in 0..3 {
                    for iii in 0..3 {
                        let impossibilities = self.get_position_xy(x + ii, y + iii).impossibilities;
                        if ((impossibilities >> (i + 1)) & 1) == 0 {
                            let final_value = i + 1;
                            self.set_final_value(final_value, x + ii, y + iii);
                            change_happened = true;
                        }
                    }
                }
            }
        }
        change_happened
    }

    fn check_all_lines_possibilities(&mut self) -> bool {
        let mut change_happened = false;
        for i in 0..9 {
            change_happened |= self.check_line_possibilities(Line::Row(i))
                | self.check_line_possibilities(Line::Column(i));
        }
        change_happened
    }

    fn check_all_box_possibilities(&mut self) -> bool {
        let mut change_happened = false;
        for i in 0..3 {
            for ii in 0..3 {
                change_happened |= self.check_box_possibilities(3 * i, 3 * ii);
            }
        }
        change_happened
    }

    fn check_possibilities(&mut self) -> bool {
        let mut change_happened = false;
        for i in 0..9 {
            for ii in 0..9 {
                change_happened |= self.0[i][ii].check_for_final_value()
            }
        }
        change_happened
    }

    fn is_solved(&self) -> bool {
        for i in 0..9 {
            for ii in 0..9 {
                if self.get_position_xy(i, ii).final_value == 0 {
                    return false;
                }
            }
        }
        true
    }

    pub fn solve(&mut self) -> bool {
        while self.is_solved() {
            self.annotate_all_lines();
            self.annotate_all_box();
            if !self.check_possibilities() {
                if !self.check_all_lines_possibilities() {
                    if !self.check_all_box_possibilities() {
                        return false;
                    }
                }
            }
        }
        true
    }
}

impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        let last_row = self.0.last().unwrap();
        for row in &self.0 {
            for position in row {
                string.push_str(&position.to_string());
            }
            if !std::ptr::eq(row, last_row) {
                string.push('\n');
            }
        }
        write!(f, "{string}")
    }
}

impl std::fmt::Debug for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        let last_row = self.0.last().unwrap();
        for row in &self.0 {
            let last_position = row.last().unwrap();
            for position in row {
                let number = position.final_value.into();
                let digit = std::char::from_digit(number, 10).unwrap();
                string.push(digit);
                string.push(':');
                let array = disp_array::DispArray(position.get_impossibilities()).to_string();
                string.push_str(array.as_str());
                if !std::ptr::eq(position, last_position) {
                    string.push('\n');
                }
            }
            if !std::ptr::eq(row, last_row) {
                string.push('\n');
                string.push('\n');
            }
        }
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_board() {
        let board = Board::new();
        for row in &board.0 {
            for position in row {
                assert_eq!(position.final_value, 0);
                assert_eq!(position.impossibilities, 0);
            }
        }
    }

    #[test]
    fn test_board_from_array() {
        let array = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9],
        ];
        let board = Board::from(array);
        for (i, row) in board.0.iter().enumerate() {
            for (j, position) in row.iter().enumerate() {
                if array[i][j] != 0 {
                    assert_eq!(position.final_value, array[i][j]);
                }
            }
        }
    }

    #[test]
    fn test_is_solved() {
        let unsolved_array = [
            [4, 5, 8, 6, 0, 0, 0, 0, 7],
            [0, 0, 0, 1, 0, 0, 0, 0, 4],
            [0, 0, 2, 4, 0, 8, 6, 0, 3],
            [0, 0, 4, 0, 1, 0, 3, 6, 2],
            [0, 0, 0, 2, 0, 5, 0, 0, 0],
            [7, 2, 9, 0, 8, 0, 5, 0, 0],
            [8, 0, 3, 7, 0, 4, 1, 0, 0],
            [2, 0, 0, 0, 0, 1, 0, 0, 0],
            [6, 0, 0, 0, 0, 2, 4, 3, 8],
        ];
        let unsolved_board = Board::from(unsolved_array);
        assert!(!unsolved_board.is_solved());

        let solved_array = [
            [5, 4, 3, 1, 2, 8, 7, 9, 6],
            [7, 8, 2, 6, 4, 9, 1, 3, 5],
            [1, 6, 9, 3, 5, 7, 2, 8, 4],
            [9, 2, 1, 5, 6, 3, 8, 4, 7],
            [3, 7, 4, 2, 8, 1, 6, 5, 9],
            [6, 5, 8, 9, 7, 4, 3, 2, 1],
            [8, 3, 5, 7, 9, 6, 4, 1, 2],
            [2, 1, 7, 4, 3, 5, 9, 6, 8],
            [4, 9, 6, 8, 1, 2, 5, 7, 3],
        ];
        let solved_board = Board::from(solved_array);
        assert!(solved_board.is_solved());
    }

    #[test]
    fn test_solve() {
        let easy_array = [
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [0, 1, 0, 2, 0, 5, 0, 9, 0],
            [0, 0, 6, 8, 0, 7, 1, 0, 0],
            [8, 0, 0, 1, 2, 3, 0, 0, 7],
            [4, 0, 0, 0, 0, 0, 0, 0, 3],
            [0, 5, 0, 7, 0, 4, 0, 6, 0],
            [0, 9, 4, 0, 1, 0, 7, 2, 0],
            [1, 0, 0, 0, 9, 0, 0, 0, 6],
            [5, 3, 0, 0, 0, 0, 0, 1, 9],
        ];
        let mut easy_board = Board::from(easy_array);
        assert!(easy_board.solve());

        let medium_array = [
            [3, 0, 0, 2, 0, 0, 1, 0, 0],
            [2, 8, 5, 0, 9, 4, 3, 0, 0],
            [0, 0, 7, 0, 0, 0, 0, 0, 0],
            [0, 3, 0, 9, 0, 0, 0, 0, 1],
            [0, 0, 9, 0, 7, 0, 0, 0, 4],
            [0, 6, 0, 8, 0, 0, 0, 0, 2],
            [0, 0, 4, 0, 0, 0, 0, 0, 0],
            [8, 5, 2, 0, 3, 6, 4, 0, 0],
            [6, 0, 0, 4, 0, 0, 2, 0, 0],
        ];
        let mut medium_board = Board::from(medium_array);
        assert!(medium_board.solve());

        let hard_array = [
            [3, 0, 0, 8, 0, 0, 0, 2, 0],
            [0, 2, 0, 3, 0, 4, 6, 0, 0],
            [4, 0, 0, 0, 7, 0, 0, 0, 0],
            [9, 1, 0, 0, 0, 0, 0, 8, 6],
            [0, 0, 0, 0, 0, 0, 0, 0, 0],
            [2, 3, 0, 0, 0, 0, 0, 1, 4],
            [0, 0, 0, 0, 4, 0, 0, 0, 3],
            [0, 0, 5, 7, 0, 9, 0, 6, 0],
            [0, 7, 0, 0, 0, 6, 0, 0, 1],
        ];
        let mut hard_board = Board::from(hard_array);
        assert!(hard_board.solve());
    }
}
