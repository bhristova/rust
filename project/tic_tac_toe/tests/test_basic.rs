use tic_tac_toe::game_4x4::Game4x4;
use tic_tac_toe::board::Board;

#[test]
fn test_board_0() {
    let mut actual_board = Board::new(5);
    let empty_cells = 25;

    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    assert_eq!(actual_board.empty_cells(), empty_cells);

    assert_eq!(actual_board.get_board_state(), expected_board);
}

#[test]
fn test_board_1() {
    let mut actual_board = Board::new(5);
    let empty_cells = 24;

    actual_board.set_cell_value(2, 2, 'X');

    let expected_board_2 = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','X','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    assert_eq!(actual_board.empty_cells(), empty_cells);

    assert_eq!(actual_board.get_board_state(), expected_board_2);
}

#[test]
fn test_board_winner_primary_diagonal() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(0, 0, 'X');
    actual_board.set_cell_value(1, 1, 'X');
    actual_board.set_cell_value(2, 2, 'X');
    actual_board.set_cell_value(3, 3, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(0, 0, '_');
    actual_board.set_cell_value(4, 4, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(2, 2, 'O');

    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_above_primary_diagonal() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(0, 1, 'X');
    actual_board.set_cell_value(1, 2, 'X');
    actual_board.set_cell_value(2, 3, 'X');
    actual_board.set_cell_value(3, 4, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(2, 3, 'O');
    
    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_below_primary_diagonal() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(1, 0, 'X');
    actual_board.set_cell_value(2, 1, 'X');
    actual_board.set_cell_value(3, 2, 'X');
    actual_board.set_cell_value(4, 3, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(3, 2, 'O');
    
    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_secondary_diagonal() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(0, 4, 'X');
    actual_board.set_cell_value(1, 3, 'X');
    actual_board.set_cell_value(2, 2, 'X');
    actual_board.set_cell_value(3, 1, 'X');
    
    assert!(actual_board.has_winner());
    
    actual_board.set_cell_value(0, 4, '_');
    actual_board.set_cell_value(4, 0, 'X');
    assert!(actual_board.has_winner());

    actual_board.set_cell_value(2, 2, 'O');

    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_above_secondary_diagonal() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(0, 3, 'X');
    actual_board.set_cell_value(1, 2, 'X');
    actual_board.set_cell_value(2, 1, 'X');
    actual_board.set_cell_value(3, 0, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(2, 1, 'O');
    
    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_below_secondary_diagonal() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(1, 4, 'X');
    actual_board.set_cell_value(2, 3, 'X');
    actual_board.set_cell_value(3, 2, 'X');
    actual_board.set_cell_value(4, 1, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(3, 2, 'O');
    
    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_row() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(0, 0, 'X');
    actual_board.set_cell_value(0, 1, 'X');
    actual_board.set_cell_value(0, 2, 'X');
    actual_board.set_cell_value(0, 3, 'X');
    
    assert!(actual_board.has_winner());
    
    actual_board.set_cell_value(0, 0, '_');
    actual_board.set_cell_value(0, 4, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(0, 4, '_');

    assert!(!actual_board.has_winner());
}

#[test]
fn test_board_winner_column() {
    let mut actual_board = Board::new(5);

    actual_board.set_cell_value(0, 1, 'X');
    actual_board.set_cell_value(1, 1, 'X');
    actual_board.set_cell_value(2, 1, 'X');
    actual_board.set_cell_value(3, 1, 'X');
    
    assert!(actual_board.has_winner());
    
    actual_board.set_cell_value(0, 1, '_');
    actual_board.set_cell_value(4, 1, 'X');

    assert!(actual_board.has_winner());

    actual_board.set_cell_value(4, 1, '_');

    assert!(!actual_board.has_winner());
}

#[test]
fn test_game_strategy_one() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','O','_','_'),
        vec!('_','_','X','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(1, 2, 'O');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_strategy_two() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','X','_','_'),
        vec!('_','_','O','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(2, 2, 'O');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_win_column() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','X','_'),
        vec!('_','_','_','X','_'),
        vec!('_','_','_','X','_'),
        vec!('_','_','_','X','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(0, 3, 'X');
    board.set_cell_value(1, 3, 'X');
    board.set_cell_value(2, 3, 'X');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_win_row() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('X','X','X','X','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(1, 1, 'X');
    board.set_cell_value(1, 2, 'X');
    board.set_cell_value(1, 3, 'X');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_trap() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','X','X','X','_'),
        vec!('_','_','O','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(1, 1, 'X');
    board.set_cell_value(2, 2, 'O');
    board.set_cell_value(1, 3, 'X');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_double_sided_trap_upper_left() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','X','X','_','_'),
        vec!('_','X','O','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(1, 2, 'X');
    board.set_cell_value(2, 1, 'X');
    board.set_cell_value(2, 2, 'O');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_double_sided_trap_upper_right() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','X','X','_'),
        vec!('_','_','O','X','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(1, 2, 'X');
    board.set_cell_value(2, 3, 'X');
    board.set_cell_value(2, 2, 'O');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_double_sided_trap_lower_left() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','X','O','_','_'),
        vec!('_','X','X','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(3, 2, 'X');
    board.set_cell_value(2, 1, 'X');
    board.set_cell_value(2, 2, 'O');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_move_double_sided_trap_lower_right() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','O','X','_'),
        vec!('_','_','X','X','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(3, 2, 'X');
    board.set_cell_value(2, 3, 'X');
    board.set_cell_value(2, 2, 'O');
    game.set_board(board);

    game.computer_turn(0);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}

#[test]
fn test_game_higest_risk_move_0() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let moves_available = vec![(1,1), (1,3), (3,1), (3,3), (3,2), (2,1), (2,3), (1,2), (0,0), (0,1), (0,2), (0,3), (0,4), (1,0), (1,4), (2,0), (2,4), (3,0), (3,4), (4,0), (4,1), (4,2), (4,3), (4,4)];

    board.set_cell_value(1, 1, 'O');
    board.set_cell_value(1, 3, 'O');
    board.set_cell_value(2, 2, 'X');
    board.set_cell_value(3, 1, 'X');

    game.set_board(board);

    assert_eq!(game.get_highest_risk_move(moves_available).0, (1, 2));
}

#[test]
fn test_game_higest_risk_move_1() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let moves_available = vec![(1,1), (1,3), (3,1), (3,3), (3,2), (2,1), (2,3), (1,2), (0,0), (0,1), (0,2), (0,3), (0,4), (1,0), (1,4), (2,0), (2,4), (3,0), (3,4), (4,0), (4,1), (4,2), (4,3), (4,4)];
    
    board.set_cell_value(1, 1, 'O');
    board.set_cell_value(1, 2, 'O');
    board.set_cell_value(1, 4, 'O');

    game.set_board(board);

    assert_eq!(game.get_highest_risk_move(moves_available).0, (1, 3));
}

#[test]
fn test_game_attack_move() {
    let mut game = Game4x4::new(5, 'O', 'X');
    let mut board = Board::new(5);
    let expected_board = vec!(
        vec!('_','_','_','O','_'),
        vec!('_','_','_','O','_'),
        vec!('_','_','X','X','_'),
        vec!('_','_','_','_','_'),
        vec!('_','_','_','_','_')
    );

    board.set_cell_value(0, 3, 'O');
    board.set_cell_value(1, 3, 'O');
    board.set_cell_value(2, 3, 'X');
    game.set_board(board);

    game.computer_turn(4);

    assert_eq!(game.get_board().get_board_state(), expected_board);
}