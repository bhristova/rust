use super::board;

use std::io::stdin;
use std::fmt;

pub struct Game4x4 {
    board: board::Board,
    human: char,
    computer: char,
    strategy: String
}

impl Game4x4 {
    pub fn new(dimensions: usize, human: char, computer: char) -> Self {
        Self { board: board::Board::new(dimensions), human: human, computer: computer, strategy: "none".to_string()}
    }

    pub fn set_board(&mut self, board: board::Board) {
        self.board = board;
    }

    pub fn get_board(self) -> board::Board {
        self.board
    }

    fn get_next_move(&mut self, iteration: i32) {
            
        let winning_move = self.get_winning_move();
        if !self.is_empty_move(winning_move) {
            self.board.set_cell_value(winning_move.0, winning_move.1, self.computer);
            println!("{}", self.board);
            return;
        }

        let trap_move = self.get_trap_move();
        if !self.is_empty_move(trap_move) {
        self.board.set_cell_value(trap_move.0, trap_move.1, self.computer);
            println!("{}", self.board);
            return;
        }

        let double_sided_trap_move = self.get_double_sided_trap_move();
        if !self.is_empty_move(double_sided_trap_move) {
        self.board.set_cell_value(double_sided_trap_move.0, double_sided_trap_move.1, self.computer);
            println!("{}", self.board);
            return;
        }

        if self.strategy == "none" {
            if self.board.is_cell_valid(2, 2) {
                self.strategy = "one".to_string();
            } else {
                self.strategy = "two".to_string();
            }
        }

        if self.strategy == "one".to_string() {
            self.strategy_one(iteration);
        } else {
            self.strategy_two(iteration);
        }
        return;
    }

    fn strategy_one(&mut self, iteration: i32) {
        let moves_map_1 = vec![(2,2)];
        let moves_map_2 = vec![(1,1), (1,3), (3,1), (3,3)];
        let moves_map_3 = vec![(1,1), (1,3), (3,1), (3,3), (3,2), (2,1), (2,3), (1,2)];
        let moves_map_rest = vec![(1,1), (1,3), (3,1), (3,3), (3,2), (2,1), (2,3), (1,2), 
        (0,0), (0,1), (0,2), (0,3), (0,4), 
        (1,0), (1,4), 
        (2,0), (2,4), 
        (3,0), (3,4), 
        (4,0), (4,1), (4,2), (4,3), (4,4)];
        
        let highest_risk_move = match iteration {
            0 => self.get_highest_risk_move(moves_map_1),
            1 => self.get_highest_risk_move(moves_map_2),
            2 => self.get_highest_risk_move(moves_map_3),
            _ => self.get_highest_risk_move(moves_map_rest),
        };

        if highest_risk_move.1 > 1 || iteration < 3 {
            self.board.set_cell_value(highest_risk_move.0.0, highest_risk_move.0.1, self.computer);
        } else {
            let attack_move = self.get_attack_move();
            if !self.is_empty_move(attack_move) {
                self.board.set_cell_value(attack_move.0, attack_move.1, self.computer);
            } else {
                self.board.set_cell_value(highest_risk_move.0.0, highest_risk_move.0.1, self.computer);
            }
        }

        println!("{}", self.board);

        return;
    }

    fn strategy_two(&mut self, iteration: i32) {
        let moves_map_1 = vec![(1,2)];
        let moves_map_2 = vec![(2,1), (2,3)];
        let moves_map_3 = vec![(1,1), (1,3), (2,1), (2,3), (3,1), (3,2), (3,3)];
        let moves_map_rest = vec![(1,1), (1,3), (3,1), (3,3), (3,2), (2,1), (2,3),
        (0,0), (0,1), (0,2), (0,3), (0,4), 
        (1,0), (1,4), 
        (2,0), (2,4), 
        (3,0), (3,4), 
        (4,0), (4,1), (4,2), (4,3), (4,4)];
        let highest_risk_move = match iteration {
            0 => self.get_highest_risk_move(moves_map_1),
            1 => self.get_highest_risk_move(moves_map_2),
            2 => self.get_highest_risk_move(moves_map_3),
            _ => self.get_highest_risk_move(moves_map_rest),
        };

        if highest_risk_move.1 > 1 || iteration < 3 {
            self.board.set_cell_value(highest_risk_move.0.0, highest_risk_move.0.1, self.computer);
        } else {
            let attack_move = self.get_attack_move();
            if !self.is_empty_move(attack_move) {
                self.board.set_cell_value(attack_move.0, attack_move.1, self.computer);
            } else {
                self.board.set_cell_value(highest_risk_move.0.0, highest_risk_move.0.1, self.computer);
            }
        }

        println!("{}", self.board);

        return;
    }

    fn get_attack_move(&mut self) -> (usize, usize) {
        for i in 0..self.board.dimensions {
            for j in 0..self.board.dimensions {
                if self.board.get_cell_value(i,j) == 'X' {
                    let adjacent_empty_cells_column = self.adjacent_empty_cells_column(i, j);
                    if !self.is_empty_move(adjacent_empty_cells_column) {
                        return adjacent_empty_cells_column; 
                    }

                    let adjacent_empty_cells_row = self.adjacent_empty_cells_row(i, j);
                    if !self.is_empty_move(adjacent_empty_cells_row) {
                        return adjacent_empty_cells_row; 
                    }
                }
            }
        }

        (10, 10)
    }

    fn adjacent_empty_cells_column(&mut self, i: usize, j: usize) -> (usize, usize) {
        let mut empty_cell = (10, 10);
        let mut empty_cell_count = 0;
        let mut computer_count = 0;
        for before_i in (0..i).rev() {
            if self.board.get_cell_value(before_i, j) == 'X' {
                computer_count = computer_count + 1;
            } else if self.board.is_cell_valid(before_i, j) {
                empty_cell_count = empty_cell_count + 1;
                if self.is_empty_move(empty_cell) {
                    empty_cell = (before_i, j);
                }
            } else {
                break;
            }
        }

        for after_i in i..self.board.dimensions {
            if self.board.get_cell_value(after_i, j) == 'X' {
                computer_count = computer_count + 1;
            } else if self.board.is_cell_valid(after_i, j) {
                empty_cell_count = empty_cell_count + 1;
                if self.is_empty_move(empty_cell) {
                    empty_cell = (after_i, j);
                }
            } else {
                break;
            }
        }


        if empty_cell_count + computer_count >= 4 { 
            return empty_cell;
        } else {
            return (10, 10);
        }
    }

    fn adjacent_empty_cells_row(&mut self, i: usize, j: usize) -> (usize, usize) {
        let mut empty_cell = (10, 10);
        let mut empty_cell_count = 0;
        let mut computer_count = 0;
        for before_j in (0..j).rev() {
            if self.board.get_cell_value(i, before_j) == 'X' {
                computer_count = computer_count + 1;
            } else if self.board.is_cell_valid(i, before_j) {
                empty_cell_count = empty_cell_count + 1;
                if self.is_empty_move(empty_cell) {
                    empty_cell = (i, before_j);
                }
            } else {
                break;
            }
        }

        for after_j in j..self.board.dimensions {
            if self.board.get_cell_value(i, after_j) == 'X' {
                computer_count = computer_count + 1;
            } else if self.board.is_cell_valid(i, after_j) {
                empty_cell_count = empty_cell_count + 1;
                if self.is_empty_move(empty_cell) {
                    empty_cell = (i, after_j);
                }
            } else {
                break;
            }
        }


        if empty_cell_count + computer_count >= 4 { 
            return empty_cell;
        } else {
            return (10, 10);
        }
    }

    fn get_winning_move(&mut self) -> (usize, usize) {
        let dimensions_minus_one = self.board.dimensions - 1;
        for i in 0..dimensions_minus_one {
            for j in 0..dimensions_minus_one {
                if self.board.is_cell_valid(i, j) {
                    if self.calculate_risk_column((i, j), self.human, self.computer) == 3 {
                        return (i, j);
                    }
                    if self.calculate_risk_row((i, j), self.human, self.computer) == 3 {
                        return (i, j);
                    }
                    if self.calculate_risk_primary_diagonal(self.human, self.computer).0 == 3 {
                        if self.board.is_cell_valid(0, 0) && (self.board.is_cell_valid(dimensions_minus_one, dimensions_minus_one) || self.board.get_cell_value(dimensions_minus_one, dimensions_minus_one) == self.human){
                            return (0, 0);
                        }
                        if self.board.is_cell_valid(dimensions_minus_one, dimensions_minus_one) && (self.board.is_cell_valid(0, 0) || self.board.get_cell_value(0, 0) == self.human){
                            return (dimensions_minus_one, dimensions_minus_one);
                        }
                    }
                    if self.calculate_risk_secondary_diagonal(self.human, self.computer).0 == 3 {
                        if self.board.is_cell_valid(0, dimensions_minus_one) && (self.board.is_cell_valid(dimensions_minus_one, 0) || self.board.get_cell_value(dimensions_minus_one, 0) == self.human){
                            return (0, dimensions_minus_one);
                        }
                        if self.board.is_cell_valid(self.board.dimensions - 1, 0) && (self.board.is_cell_valid(0, dimensions_minus_one) || self.board.get_cell_value(0, dimensions_minus_one) == self.human){
                            return (dimensions_minus_one, 0);
                        }
                    }

                    if self.calculate_risk_above_primary_diagonal(self.human, self.computer).0 == 3 {
                        if self.board.is_cell_valid(0, 1) {
                            return (0, 1);
                        }
                        if self.board.is_cell_valid(3, self.board.dimensions - 1) {
                            return (3, self.board.dimensions - 1);
                        }
                    }

                    if self.calculate_risk_below_primary_diagonal(self.human, self.computer).0 == 3 {
                        if self.board.is_cell_valid(1, 0) {
                            return (1, 0);
                        }
                        if self.board.is_cell_valid(self.board.dimensions - 1, 3) {
                            return (self.board.dimensions - 1, 3);
                        }
                    }

                    if self.calculate_risk_above_secondary_diagonal(self.human, self.computer).0 == 3 {
                        if self.board.is_cell_valid(0, 3) {
                            return (0, 3);
                        }
                        if self.board.is_cell_valid(3, 0) {
                            return (3, 0);
                        }
                    }

                    if self.calculate_risk_below_secondary_diagonal(self.human, self.computer).0 == 3 {
                        if self.board.is_cell_valid(1, dimensions_minus_one) {
                            return (0, dimensions_minus_one);
                        }
                        if self.board.is_cell_valid(dimensions_minus_one, 1) {
                            return (dimensions_minus_one, 1);
                        }
                    }
                }
            }
        }

        (10, 10)
    }

    fn get_trap_move(&mut self) -> (usize, usize) {
        let dimensions_minus_one = self.board.dimensions - 1;
        for i in 1..dimensions_minus_one {
            if self.board.equal_non_empty_cells((i, 1), (i, 3)) 
            && self.board.get_cell_value(i, 1) == self.computer 
            && self.board.is_cell_valid(i, 2) 
            && (self.board.is_cell_valid(i, 0) || self.board.is_cell_valid(i, 4)) {
                return (i, 2);
            }

            if self.board.equal_non_empty_cells((1, i), (3, i)) 
            && self.board.get_cell_value(1, i) == self.computer 
            && self.board.is_cell_valid(2, i) 
            && (self.board.is_cell_valid(0, i) || self.board.is_cell_valid(4, i)) {
                return (2, i);
            }
        }

        (10, 10)
    }

    fn get_double_sided_trap_move(&mut self) -> (usize, usize) {

        let upper_left = self.double_sided_trap((1, 2), (2, 1));
        if !self.is_empty_move(upper_left) {
            return upper_left;
        }

        let upper_right = self.double_sided_trap((1, 2), (2, 3));
        if !self.is_empty_move(upper_right) {
            return upper_right;
        }

        let lower_left = self.double_sided_trap((3, 2), (2, 1));
        if !self.is_empty_move(lower_left) {
            return lower_left;
        }

        let lower_right = self.double_sided_trap((3, 2), (2, 3));
        if !self.is_empty_move(lower_right) {
            return lower_right;
        }

        (10, 10)
    }

    fn double_sided_trap(&mut self, first_cell: (usize, usize), second_cell: (usize, usize)) -> (usize, usize) {
        if self.board.get_cell_value(first_cell.0, first_cell.1) == self.board.get_cell_value(second_cell.0, second_cell.1) && self.board.get_cell_value(second_cell.0, second_cell.1) == self.computer {
            let mut empty_row = 0;
            let mut empty_column = 0;
            for i in 1..self.board.dimensions {
                if self.board.is_cell_valid(first_cell.0, i) {
                    empty_row = empty_row + 1;
                }

                if self.board.is_cell_valid(i, second_cell.1) {
                    empty_column = empty_column + 1;
                }
            }

            if empty_row == 3 && empty_column == 3 {
                return (first_cell.0, second_cell.1);
            }
        }

        (10, 10)
    }

    fn is_empty_move(&self, curr_move: (usize, usize)) -> bool {
        curr_move.0 > self.board.dimensions && curr_move.1 > self.board.dimensions
    }

    pub fn get_highest_risk_move(&mut self, iteration_moves_available: Vec<(usize, usize)>) -> ((usize, usize), i32) {
        let mut highest_risk = 0;
        let mut highest_risk_move = iteration_moves_available[0];
            for i in 0..iteration_moves_available.len() {
            if self.board.is_cell_valid(iteration_moves_available[i].0, iteration_moves_available[i].1) {

                let current_risk = self.calculate_risk(iteration_moves_available[i]);
                if current_risk > highest_risk {
                    highest_risk = current_risk;
                    highest_risk_move = iteration_moves_available[i];
                }

                if !self.board.is_cell_valid(highest_risk_move.0, highest_risk_move.1) {
                    highest_risk_move = iteration_moves_available[i];
                }

                let risk_primary_diagonal = self.calculate_risk_primary_diagonal(self.computer, self.human);
                if risk_primary_diagonal.0 > highest_risk  {
                    highest_risk = risk_primary_diagonal.0;
                    highest_risk_move = risk_primary_diagonal.1;
                }

                let risk_secondary_diagonal = self.calculate_risk_secondary_diagonal(self.computer, self.human);
                if risk_secondary_diagonal.0 > highest_risk  {
                    highest_risk = risk_secondary_diagonal.0;
                    highest_risk_move = risk_secondary_diagonal.1;
                }

                let risk_above_primary_diagonal = self.calculate_risk_above_primary_diagonal(self.computer, self.human);
                if risk_above_primary_diagonal.0 > highest_risk  {
                    highest_risk = risk_above_primary_diagonal.0;
                    highest_risk_move = risk_above_primary_diagonal.1;
                }

                let risk_below_primary_diagonal = self.calculate_risk_below_primary_diagonal(self.computer, self.human);
                if risk_below_primary_diagonal.0 > highest_risk  {
                    highest_risk = risk_below_primary_diagonal.0;
                    highest_risk_move = risk_below_primary_diagonal.1;
                }

                let risk_above_secondary_diagonal = self.calculate_risk_above_secondary_diagonal(self.computer, self.human);
                if risk_above_secondary_diagonal.0 > highest_risk  {
                    highest_risk = risk_above_secondary_diagonal.0;
                    highest_risk_move = risk_above_secondary_diagonal.1;
                }

                let risk_below_secondary_diagonal = self.calculate_risk_below_secondary_diagonal(self.computer, self.human);
                if risk_below_secondary_diagonal.0 > highest_risk  {
                    highest_risk = risk_below_secondary_diagonal.0;
                    highest_risk_move = risk_below_secondary_diagonal.1;
                }
            }
        }
        (highest_risk_move, highest_risk)
    }

    fn calculate_risk(&mut self, current_move: (usize, usize)) -> i32 {
        self.calculate_risk_column(current_move, self.computer, self.human) + self.calculate_risk_row(current_move, self.computer, self.human)
    }

    fn calculate_risk_column(&mut self, current_move: (usize, usize), this_player: char, opponent: char) -> i32 {
        let mut risk = 0;

        if self.board.get_cell_value(1, current_move.1) == this_player 
        || self.board.get_cell_value(2, current_move.1) == this_player
        || self.board.get_cell_value(3, current_move.1) == this_player {
            return risk;
        }

        for i in 0..self.board.dimensions - 1 {
            risk = risk + if self.board.get_cell_value(i, current_move.1) == opponent { 1 } else { 0 };
        }

        risk
    }

    fn calculate_risk_row(&mut self, current_move: (usize, usize), this_player: char, opponent: char) -> i32 {
        let mut risk = 0;

        if self.board.get_cell_value(current_move.0, 1) == this_player 
        || self.board.get_cell_value(current_move.0, 2) == this_player 
        || self.board.get_cell_value(current_move.0, 3) == this_player {
            return risk;
        }

        for i in 0..self.board.dimensions - 1 {
            risk = risk + if self.board.get_cell_value(current_move.0, i) == opponent { 1 } else { 0 };
        }

        risk
    }

    fn calculate_risk_primary_diagonal(&mut self, this_player: char, opponent: char) -> (i32, (usize, usize)) {
        let mut risk = 0;
        let mut get_breaking_coords = (10, 10);

        for i in 1..self.board.dimensions - 2 {
            if self.board.get_cell_value(i, i) == this_player {
                return (0, get_breaking_coords);
            }
        }

        for i in 0..self.board.dimensions - 1 {
            if self.board.get_cell_value(i, i) == opponent { 
                risk = risk + 1;
            } else { 
                get_breaking_coords = (i, i);
            };
        }

        (risk, get_breaking_coords)
    }

    fn calculate_risk_above_primary_diagonal(&mut self, this_player: char, opponent: char) -> (i32, (usize, usize)) {
        let mut risk = 0;
        let mut get_breaking_coords = (10, 10);

        for i in 0..self.board.dimensions - 2 {
            if self.board.get_cell_value(i + 1, i) == this_player {
                return (0, get_breaking_coords);
            }

            if self.board.get_cell_value(i + 1, i) == opponent { 
                risk = risk + 1;
            } else { 
                get_breaking_coords = (i + 1, i);
            };
        }

        (risk, get_breaking_coords)
    }

    fn calculate_risk_below_primary_diagonal(&mut self, this_player: char, opponent: char) -> (i32, (usize, usize)) {
        let mut risk = 0;
        let mut get_breaking_coords = (10, 10);

        for i in 0..self.board.dimensions - 2 {
            if self.board.get_cell_value(i, i + 1) == this_player {
                return (0, get_breaking_coords);
            }
            
            if self.board.get_cell_value(i, i + 1) == opponent { 
                risk = risk + 1;
            } else { 
                get_breaking_coords = (i, i + 1);
            };
        }

        (risk, get_breaking_coords)
    }

    fn calculate_risk_secondary_diagonal(&mut self, this_player: char, opponent: char) -> (i32, (usize, usize)) {
        let mut risk = 0;
        let mut get_breaking_coords = (10, 10);

        for i in 1..self.board.dimensions - 2 {
            if self.board.get_cell_value(i, self.board.dimensions - 1 - i) == this_player {
                return (0, get_breaking_coords)
            }
        }

        for i in 0..self.board.dimensions - 1 {
            if self.board.get_cell_value(i, self.board.dimensions - 1 - i) == opponent { 
                risk = risk + 1;
            } else { 
                get_breaking_coords = (i, self.board.dimensions - 1 - i);
            };
        }

        (risk, get_breaking_coords)
    }

    fn calculate_risk_above_secondary_diagonal(&mut self, this_player: char, opponent: char) -> (i32, (usize, usize)) {
        let mut risk = 0;
        let mut get_breaking_coords = (10, 10);

        for i in 0..self.board.dimensions - 2 {
            if self.board.get_cell_value(self.board.dimensions - i - 2, i) == this_player {
                return (0, get_breaking_coords)
            }

            if self.board.get_cell_value(self.board.dimensions - i - 2, i) == opponent { 
                risk = risk + 1;
            } else { 
                get_breaking_coords = (self.board.dimensions - i - 2, i);
            };
        }

        (risk, get_breaking_coords)
    }

    fn calculate_risk_below_secondary_diagonal(&mut self, this_player: char, opponent: char) -> (i32, (usize, usize)) {
        let mut risk = 0;
        let mut get_breaking_coords = (10, 10);

        for i in 1..self.board.dimensions {
            if self.board.get_cell_value(i, self.board.dimensions - i) == this_player {
                return (0, get_breaking_coords)
            } else if self.board.get_cell_value(i, self.board.dimensions - i) == opponent { 
                risk = risk + 1;
            } else { 
                get_breaking_coords = (i, self.board.dimensions - i);
            };
        }
        (risk, get_breaking_coords)
    }

    pub fn computer_turn(&mut self, iteration: i32) {
        self.get_next_move(iteration);
    }

    pub fn human_turn(&mut self) {
        if self.board.game_over() {
            return;
        }
        
        let mut x = 0_usize;
        let mut y = 0_usize;

        loop {
            let mut buffer = String::new();

            println!("Input x coordinate, x > 0 and x < {}", self.board.dimensions);
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    match buffer.trim_end() {
                        "" => println!("Try again"),
                        value => x = value.chars().nth(0).unwrap().to_digit(10).unwrap() as usize
                    }
                },
                _ => println!("Try again")
            }

            println!("Input y coordinate, y > 0 and y < {}", self.board.dimensions);
            buffer = String::new();
            match stdin().read_line(&mut buffer) {
                Ok(_) => {
                    match buffer.trim_end() {
                        "" => println!("Try again"),
                        value => y = value.chars().nth(0).unwrap().to_digit(10).unwrap() as usize
                    }
                },
                _ => println!("Try again")
            }
            

            if self.board.is_cell_valid(x, y) {
                self.board.set_cell_value(x, y, self.human);
                println!("{}", self.board);

                return;
            } else {
                println!("invalid!");
                println!("x: {}, y: {}", x, y);
                println!("buffer {}", buffer);
                println!("{}", &self.board.get_cell_value(x, y));

            }
        }
    }

    pub fn game_over(&mut self) -> bool {
        return self.board.game_over();
    }
}

impl fmt::Display for Game4x4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.board)
    }
}

