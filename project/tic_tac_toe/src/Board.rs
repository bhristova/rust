use std::fmt;
use std::iter;

pub struct Board {
    state: Vec<Vec<char>>,
    pub dimensions: usize
}

impl Board {
    pub fn new(dim: usize) -> Self {
        let mut state = Vec::new();

        for _ in 0..dim {
            let mut state_row = Vec::new();
            let character = '_';

            state_row.extend(iter::repeat(character).take(dim as usize));

            state.push(state_row);
        }

        Self { state: state, dimensions: dim }
    }

    pub fn get_board_state(self) -> Vec<Vec<char>> {
        self.state
    }

    pub fn set_cell_value(&mut self, x: usize, y: usize, value: char) {
        if x < self.dimensions && y < self.dimensions {
            self.state[x][y] = value;
        }
    }

    pub fn empty_cells(&mut self) -> usize {
        let mut count = 0;
        for i in 0..self.dimensions {
            for j in 0..self.dimensions {
                if self.is_cell_empty(i as usize, j as usize) {
                    count = count + 1;
                }
            }
        }
        return count;
    }

    pub fn is_cell_empty(&mut self, x: usize, y: usize) -> bool {
        self.state[x][y] == '_'
    }

    pub fn is_cell_valid(&mut self, x: usize, y: usize) -> bool {
        x > self.dimensions - 1 || y > self.dimensions - 1 || self.is_cell_empty(x,y)
    }

    pub fn get_cell_value(&self, x: usize, y: usize) -> char {
        self.state[x][y]
    }

    pub fn equal_values_row(&mut self, row_index: usize) -> bool {
        let mut equal_count = 0;
        let mut current_cell = self.state[row_index][2];
        for i in 0..self.dimensions {
            if current_cell == self.state[row_index][i] {
                equal_count = equal_count + 1;
                current_cell = self.state[row_index][i];
            }

            if equal_count == self.dimensions - 1 {
                return true;
            }
        }

        false
    }

    pub fn equal_values_column(&mut self, column_index: usize) -> bool {
        let mut equal_count = 0;
        let mut current_cell = self.state[2][column_index];
        for i in 0..self.dimensions {
            if current_cell == self.state[i][column_index] {
                equal_count = equal_count + 1;
                current_cell = self.state[i][column_index];
            }

            if equal_count == self.dimensions - 1 {
                return true;
            }
        }

        false
    }

    pub fn equal_values_primary_diagonal(&mut self) -> bool {
        let mut equal_count_primary = 0;
        let mut equal_count_above = 0;
        let mut equal_count_below = 0;

        for i in 0..self.dimensions {
            if self.equal_non_empty_cells((i, i), (self.dimensions / 2, self.dimensions / 2)) {
                equal_count_primary = equal_count_primary + 1
            }

            if self.equal_non_empty_cells((i + 1, i), (self.dimensions / 2 + 1, self.dimensions / 2)) {
                equal_count_above = equal_count_above + 1
            }

            if self.equal_non_empty_cells((i, i + 1), (self.dimensions / 2 - 1, self.dimensions / 2)) {
                equal_count_below = equal_count_below + 1
            }

            if equal_count_primary == self.dimensions - 1 || equal_count_above == self.dimensions - 1 || equal_count_below == self.dimensions - 1 {
                return true;
            }
        }

        false
    }

    pub fn equal_values_secondary_diagonal(&mut self) -> bool {
        let mut equal_count_primary = 0;
        let mut equal_count_above = 0;
        let mut equal_count_below = 0;

        for i in 0..self.dimensions {
            if self.equal_non_empty_cells((i, self.dimensions - i - 1), (self.dimensions / 2, self.dimensions / 2)) {
                equal_count_primary = equal_count_primary + 1;
            }

            if  i + 2 <= self.dimensions && self.equal_non_empty_cells((i, self.dimensions - i - 2), (0, 3)) {
                equal_count_above = equal_count_above + 1;
            }

            if self.equal_non_empty_cells((i + 1, self.dimensions - i - 1), (1, 4)) {
                equal_count_below = equal_count_below + 1;
            }

            if equal_count_primary == self.dimensions - 1 || equal_count_above == self.dimensions - 1 || equal_count_below == self.dimensions - 1 {
                return true;
            }
        }
        false
    }

    pub fn check_cell_coords(&mut self, cell: (usize, usize)) -> bool {
        cell.0 < self.dimensions && cell.1 < self.dimensions
    }

    pub fn equal_non_empty_cells(&mut self, first: (usize, usize), second: (usize, usize)) -> bool {
        if self.check_cell_coords(first) && self.check_cell_coords(second) {
            return self.state[first.0 as usize][first.1 as usize] == self.state[second.0 as usize][second.1 as usize] 
            && self.state[first.0 as usize][first.1 as usize] != '_';
        }
        false
    }

    pub fn has_winner(&mut self) -> bool {
        for i in 0..self.dimensions {
            if !self.is_cell_empty(i,2) && self.equal_values_row(i) {
                return true;
            }
            
            if !self.is_cell_empty(2,i) && self.equal_values_column(i) {
                return true;
            }
        }

        return self.equal_values_primary_diagonal() || self.equal_values_secondary_diagonal();
    }

    pub fn game_over(&mut self) -> bool {
        return self.empty_cells() == 0 || self.has_winner();
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for i in 0..self.dimensions {
            for j in 0..self.dimensions {
                write!(f, "{} ", self.state[i as usize][j as usize])?;
            }
            write!(f, "\n")?;
        }
        Ok(())
    }
}
