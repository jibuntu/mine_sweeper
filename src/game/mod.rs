#![allow(dead_code)]

extern crate rand;
use rand::Rng;

mod board;
use crate::game::board::Board;
use crate::game::board::Square;

pub struct Score {
    number_of_flags: usize,
    number_of_unopened_mines: usize,
    number_of_unopened_squares: usize,
    number_of_opened_mines: usize,
    number_of_opened_squares: usize,
}

impl Score {
    pub fn to_string(&self) -> String {
        format!("[] {}  |  [\x1b[91m<>\x1b[0m] {}  |  \x1b[93m/>\x1b[0m {}  |  \
                \x1b[91m<>\x1b[0m {}  |  [\x1b[91m<>\x1b[0m] - \x1b[93m/>\x1b[0m = {}",
                self.number_of_unopened_squares,
                self.number_of_unopened_mines,
                self.number_of_flags,
                self.number_of_opened_mines,
                self.number_of_unopened_mines as isize - self.number_of_flags as isize)
    }
}

pub struct Game {
    // (height, width)
    board_size: (usize, usize),
    // (x, y)
    cursor: (usize, usize),
    board: Board,
}

impl Game {
    pub fn new(height: usize, width: usize) -> Result<Game, String> {
        let board = match Board::new(width, height) {
            Ok(board) => board,
            Err(e) => return Err(e)
        };

        let game = Game {
            cursor: (0, 0),
            board_size: board.get_size(),
            board
        };

        Ok(game)
    }

    pub fn set_mines(&mut self, times: usize) {
        self.set_mines_to_squares(times);
        self.remove_mine_and_open();
        self.set_numbers_to_squares();
    }

    fn set_mines_to_squares(&self, times: usize) {
        let mut rng = rand::thread_rng();

        for i in 0..times {
            let y = rng.gen_range(0, self.board_size.0);
            let x = rng.gen_range(0, self.board_size.1);
            let square = self.board.get_square((x, y)).unwrap();

            *square.is_mine.borrow_mut() = true;
        }
    }

    fn set_numbers_to_squares(&self) {
        for y in 0..self.board_size.0 {
            for x in 0..self.board_size.1 {
                let square = self.board.get_square((x,y)).unwrap();
                let around_squares = self.board.get_around_squares_of(square);
                let mut count_of_mines = 0;
                for (direction, around_square) in around_squares {
                    if let Some(s) = around_square {
                        if *s.is_mine.borrow() == true {
                            count_of_mines += 1;
                        }
                    }
                }
                *square.number.borrow_mut() = count_of_mines;
            }
        }
    }

    // 辺にあるマスの地雷を削除して、マスを開く。
    fn remove_mine_and_open(&self) {
        for h in 0..self.board_size.0 {
            let square = self.board.get_square((0, h)).unwrap();
            *square.is_mine.borrow_mut() = false;
            *square.is_open.borrow_mut() = true;

            let square = self.board.get_square((self.board_size.1-1, h)).unwrap();
            *square.is_mine.borrow_mut() = false;
            *square.is_open.borrow_mut() = true;
        }

        for w in 0..self.board_size.1 {
            let square = self.board.get_square((w, 0)).unwrap();
            *square.is_mine.borrow_mut() = false;
            *square.is_open.borrow_mut() = true;

            let square = self.board.get_square((w, self.board_size.0-1)).unwrap();
            *square.is_mine.borrow_mut() = false;
            *square.is_open.borrow_mut() = true;
        }
    }

    pub fn board_to_string(&self) -> String {
        self.board.to_string_with_cursor(self.cursor)
    }

    pub fn get_score(&self) -> Score {
        let mut number_of_flags = 0;
        let mut number_of_unopened_mines = 0;
        let mut number_of_unopened_squares = 0;
        let mut number_of_opened_mines = 0;
        let mut number_of_opened_squares = 0;

        for y in 0..self.board_size.0 {
            for x in 0..self.board_size.1 {
                let square = self.board.get_square((x,y)).unwrap();

                if *square.is_open.borrow() == false {
                    if *square.is_flag.borrow() {
                        number_of_flags += 1;
                    }

                    if *square.is_mine.borrow() {
                        number_of_unopened_mines += 1;
                    }

                    if *square.is_mine.borrow() == false {
                        number_of_unopened_squares += 1;
                    }
                } else {
                    if *square.is_mine.borrow() {
                        number_of_opened_mines += 1;
                    }

                    if *square.is_mine.borrow() == false {
                        number_of_opened_squares += 1;
                    }
                }

            }
        }

        Score {
            number_of_flags,
            number_of_unopened_mines,
            number_of_unopened_squares,
            number_of_opened_mines,
            number_of_opened_squares,
        }
    }

    pub fn back_history(&mut self) {
        self.board.back_squares_history();
    }

    // is_open, is_mine, is_flagがtrueのマスは伝播しない
    // numberが0のマスは自身のマスを開けた後、四方のマスに伝播する
    // numberが0でないマスは自身を開けるが伝播しない
    // 自身のマスを変更したら、before_squaresに追加する
    fn open_adjacent_squares(&self,
                             center_square: &Square,
                             before_squares: &mut Vec<Square>) {
        let around_squares = self.board.get_around_squares_of(center_square);
        for (direction, square) in &around_squares[0..4] {
            let square = match square {
                Some(square) => *square,
                None => continue
            };

            if *square.is_mine.borrow() || *square.is_open.borrow()
                || *square.is_flag.borrow(){
                continue
            }
            if *square.number.borrow() != 0 {
                before_squares.push(square.clone());
                *square.is_open.borrow_mut() = true;
                continue
            }

            before_squares.push(square.clone());
            *square.is_open.borrow_mut() = true;
            self.open_adjacent_squares(square, before_squares);
        }
    }

    pub fn open(&mut self) {
        let square = self.board.get_square(self.cursor).unwrap();

        if *square.is_open.borrow() || *square.is_flag.borrow() == true {
            return;
        }

        let mut before_squares = Vec::new();
        before_squares.push(square.clone());

        *square.is_open.borrow_mut() = true;


        if *square.number.borrow() == 0 {
            self.open_adjacent_squares(square, &mut before_squares);
        }

        self.board.add_squares_history(before_squares);
    }

    pub fn open_all_squares(&mut self) {
        let mut before_squares = Vec::new();
        for y in 0..self.board_size.0 {
            for x in 0..self.board_size.1 {
                let square = self.board.get_square((x,y)).unwrap();
                before_squares.push(square.clone());
                *square.is_open.borrow_mut() = true;
            }
        }

        self.board.add_squares_history(before_squares);
    }

    pub fn toggle_flag(&mut self) {
        let square = self.board.get_square(self.cursor).unwrap();

        if *square.is_open.borrow() {
            return;
        }

        let before_square = square.clone();

        let is_flag = *square.is_flag.borrow_mut();
        *square.is_flag.borrow_mut() = !is_flag;

        self.board.add_squares_history(vec![before_square]);
    }

    pub fn cursor(&mut self, cursor: (usize, usize)) {
        if 0 <= cursor.0 || cursor.0 < self.board_size.1 {
            if 0 <= cursor.1 || cursor.1 < self.board_size.0 {
                self.cursor = cursor;
            }
        }
    }

    pub fn cursor_left(&mut self) {
        if 0 < self.cursor.0 {
            self.cursor.0 -= 1
        }
    }

    pub fn cursor_right(&mut self) {
        if &self.cursor.0 + 1 < self.board_size.1 {
            self.cursor.0 += 1
        }
    }

    pub fn cursor_up(&mut self) {
        if 0 < self.cursor.1 {
            self.cursor.1 -= 1
        }
    }

    pub fn cursor_down(&mut self) {
        if &self.cursor.1 + 1 < self.board_size.0 {
            self.cursor.1 += 1
        }
    }

    pub fn cursor_home(&mut self) {
        self.cursor.0 = 0;
    }

    pub fn cursor_end(&mut self) {
        self.cursor.0 = &self.board_size.1 - 1;
    }

    pub fn cursor_top(&mut self) {
        self.cursor.1 = 0;
    }

    pub fn cursor_bottom(&mut self) {
        self.cursor.1 = &self.board_size.0 - 1;
    }

}

#[test]
fn test_cursor() {
    let mut game = Game::new(3, 3).unwrap();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m[][]\n\
                [][][]\n\
                [][][]");
    game.cursor_right();
    assert_eq!(game.board_to_string().as_str(),
               "[]\x1b[7m[]\x1b[27m[]\n\
                [][][]\n\
                [][][]");
    game.cursor_left();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m[][]\n\
                [][][]\n\
                [][][]");
    game.cursor_down();
    assert_eq!(game.board_to_string().as_str(),
               "[][][]\n\
                \x1b[7m[]\x1b[27m[][]\n\
                [][][]");
    game.cursor_up();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m[][]\n\
                [][][]\n\
                [][][]");
    game.cursor((1,1));
    assert_eq!(game.board_to_string().as_str(),
               "[][][]\n\
                []\x1b[7m[]\x1b[27m[]\n\
                [][][]");
    let mut game = Game::new(1, 1).unwrap();
    game.cursor_left();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m");
    game.cursor_right();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m");
    game.cursor_up();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m");
    game.cursor_down();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m");

}
#[test]
fn test_game_open() {
    let mut game = Game::new(1, 1).unwrap();
    game.open();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m  \x1b[27m");
}
#[test]
fn test_game_open_all_squares() {
    let mut game = Game::new(1, 1).unwrap();
    game.open_all_squares();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m  \x1b[27m");
    let mut game = Game::new(3, 3).unwrap();
    game.open_all_squares();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m  \x1b[27m    \n      \n      ");
}
#[test]
fn test_game_toggle_flag() {
    let mut game = Game::new(1, 1).unwrap();
    game.toggle_flag();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m\x1b[93m/>\x1b[0m\x1b[27m");
    game.toggle_flag();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m");
}
#[test]
fn test_game_set_mines() {
    let mut game = Game::new(3, 3).unwrap();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m[]\x1b[27m[][]\n\
                [][][]\n\
                [][][]");
    let square = game.board.get_square((1, 1)).unwrap();
    *square.is_mine.borrow_mut() = true;
    game.set_numbers_to_squares();
    game.open_all_squares();
    assert_eq!(game.board_to_string().as_str(),
               "\x1b[7m１\x1b[27m１１\n\
                １\x1b[91m<>\x1b[0m１\n\
                １１１");
}
