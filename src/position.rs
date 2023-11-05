#[derive(Clone, Copy, PartialEq)]
pub struct Position {
    pub final_value: u8, // The final value determined to be in this position, 0 for unknown
    pub impossibilities: u16,
}

impl Position {
    pub fn new() -> Position {
        Position {
            final_value: 0,
            impossibilities: 0,
        }
    }

    fn bits_to_array(bits: u16) -> [u8; 9] {
        let mut array = [0; 9];
        for i in 1..=9 {
            if ((bits >> i) & 1) == 1 {
                array[usize::from(i - 1)] = i
            }
        }
        array
    }

    pub fn get_impossibilities(&self) -> [u8; 9] {
        Self::bits_to_array(self.impossibilities)
    }

    pub fn set_final_value(&mut self, final_value: u8) {
        debug_assert_ne!(final_value, 0);
        debug_assert!(final_value < 10);
        self.final_value = final_value;
        self.impossibilities = 0b0000001111111110;
    }

    pub fn add_impossibilities(&mut self, impossibilities: u16) {
        debug_assert!(impossibilities <= 0b0000001111111110);
        self.impossibilities |= impossibilities;
    }

    pub fn check_for_final_value(&mut self) -> bool {
        if self.final_value != 0 {
            return false;
        }
        let mut possibility = 0;
        let mut change_happened = false;
        for i in 1..=9 {
            if ((self.impossibilities >> i) & 1) == 0 {
                if possibility != 0 {
                    return false;
                }
                possibility = i;
                change_happened = true;
            }
        }
        self.final_value = possibility;
        change_happened
    }
}

impl std::fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.final_value)
    }
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut string = String::new();
        string.push_str(&self.final_value.to_string());
        string.push(':');
        string.push_str(
            &disp_array::DispArray(Position::bits_to_array(self.impossibilities)).to_string(),
        );
        write!(f, "{string}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_position() {
        let position = Position::new();
        assert_eq!(position.final_value, 0);
        assert_eq!(position.impossibilities, 0);
    }

    #[test]
    fn test_set_final_value() {
        let mut position = Position::new();
        position.set_final_value(5);
        assert_eq!(position.final_value, 5);
        assert_eq!(position.impossibilities, 0b0000001111111110);
    }

    #[test]
    fn test_add_impossibilities() {
        let mut position = Position::new();
        position.add_impossibilities(0b0000000010000000);
        assert_eq!(position.impossibilities, 0b0000000010000000);

        position.add_impossibilities(0b0000000000010000);
        assert_eq!(position.impossibilities, 0b0000000010010000);
    }

    #[test]
    fn test_check_for_final_value() {
        let mut position = Position::new();
        position.add_impossibilities(0b0000000010000000);
        assert_eq!(position.check_for_final_value(), false);

        position.add_impossibilities(0b0000001110111110);
        assert_eq!(position.check_for_final_value(), true);
        assert_eq!(position.final_value, 6);
    }
}
