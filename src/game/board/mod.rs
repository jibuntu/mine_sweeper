#![allow(dead_code)]

use std::cell::RefCell;

#[derive(Debug, PartialEq, Clone)]
struct Position {
    x: usize,
    y: usize
}

// マスのクラス
#[derive(Debug, PartialEq, Clone)]
pub struct Square {
    pub is_mine: RefCell<bool>,
    pub is_open: RefCell<bool>,
    pub is_flag: RefCell<bool>,
    pub number: RefCell<usize>,
    pos: Position,
}


use std::mem;

#[test]
fn test_square() {
    assert_eq!(mem::size_of::<Square>(), 80);
}

fn number_to_zenkaku_string(number: usize) -> String {
    match number {
        0 => "０".to_string(),
        1 => "１".to_string(),
        2 => "２".to_string(),
        3 => "３".to_string(),
        4 => "４".to_string(),
        5 => "５".to_string(),
        6 => "６".to_string(),
        7 => "７".to_string(),
        8 => "８".to_string(),
        9 => "９".to_string(),
        _ => "０".to_string(),
    }
}

impl Square {
    fn new(x: usize, y: usize) -> Square {
        Square {
            is_mine: RefCell::new(false),
            is_open: RefCell::new(false),
            is_flag: RefCell::new(false),
            number: RefCell::new(0),
            pos: Position {
                x,
                y
            }
        }
    }

    fn to_string(&self) -> String {
        if *self.is_open.borrow() == true {
            if *self.is_mine.borrow() == true {
                return "\x1b[91m<>\x1b[0m".to_string();
            }
            if *self.number.borrow() == 0 {
                return "  ".to_string();
            }
            //return self.number.borrow().to_string() +
            //       &self.number.borrow().to_string()
            return  number_to_zenkaku_string(*self.number.borrow())
        }
        if *self.is_flag.borrow() == true {
            return "\x1b[93m/>\x1b[0m".to_string();
        }
        "[]".to_string()
    }
}

#[test]
fn test_square_to_string() {
    let square = Square::new(0, 0);
    assert_eq!(square.to_string().as_str(), "[]");
    *square.is_flag.borrow_mut() = true;
    assert_eq!(square.to_string().as_str(), "\x1b[93m/>\x1b[0m");
    *square.is_open.borrow_mut() = true;
    assert_eq!(square.to_string().as_str(), "  ");
    *square.number.borrow_mut() = 1;
    assert_eq!(square.to_string().as_str(), "１");
    *square.number.borrow_mut() = 8;
    assert_eq!(square.to_string().as_str(), "８");
    *square.is_mine.borrow_mut() = true;
    assert_eq!(square.to_string().as_str(), "\x1b[91m<>\x1b[0m");
}


// マスを持つクラス
#[derive(Debug, PartialEq)]
pub struct Board {
    // (height, width)
    size: (usize, usize),
    // マスの２次元配列
    squares: Vec<Vec<Square>>,
    // 過去のsquaresの配列
    squares_history: Vec<Vec<Square>>
}

impl Board {
    pub fn new(x: usize, y: usize) -> Result<Board, String> {
        if x == 0 || y == 0 {
            return Err("x or y is 0.".to_string());
        }

        let mut board = Board {
            size: (y, x),
            squares: Vec::new(),
            squares_history: Vec::new(),
        };

        for h in 0..y {
            board.squares.push(Vec::new());
            for w in 0..x {
                board.squares[h].push(Square::new(w, h));
            }
        }

        return Ok(board);
    }

    pub fn get_size(&self) -> (usize, usize) {
        self.size
    }

    pub fn get_square(&self, pos: (usize, usize)) -> Option<&Square> {
        let line = match self.squares.get(pos.1) {
            Some(line) => line,
            None => return None
        };
        let square = match line.get(pos.0) {
            Some(square) => square,
            None => return None
        };

        Some(square)
    }

    // 上下左右
    pub fn get_right_square_of(&self, square: &Square) -> Option<&Square> {
        self.get_square((square.pos.x+1, square.pos.y))
    }
    pub fn get_left_square_of(&self, square: &Square) -> Option<&Square> {
        if square.pos.x == 0 { return None }
        self.get_square((square.pos.x-1, square.pos.y))
    }
    pub fn get_upper_square_of(&self, square: &Square) -> Option<&Square> {
        if square.pos.y == 0 { return None }
        self.get_square((square.pos.x, square.pos.y-1))
    }
    pub fn get_lower_square_of(&self, square: &Square) -> Option<&Square> {
        self.get_square((square.pos.x, square.pos.y+1))
    }
    // 斜めの四方
    pub fn get_upper_right_square_of(&self, square: &Square) -> Option<&Square> {
        if square.pos.y == 0 { return None }
        self.get_square((square.pos.x+1, square.pos.y-1))
    }
    pub fn get_lower_right_square_of(&self, square: &Square) -> Option<&Square> {
        self.get_square((square.pos.x+1, square.pos.y+1))
    }
    pub fn get_upper_left_square_of(&self, square: &Square) -> Option<&Square> {
        if square.pos.x == 0 || square.pos.y == 0 { return None }
        self.get_square((square.pos.x-1, square.pos.y-1))
    }
    pub fn get_lower_left_square_of(&self, square: &Square) -> Option<&Square> {
        if square.pos.x == 0 { return  None }
        self.get_square((square.pos.x-1, square.pos.y+1))
    }

    pub fn get_around_squares_of(&self, square: &Square) -> Vec<(String, Option<&Square>)> {
        let mut squares = Vec::new();
        squares.push(("right".to_string(), self.get_right_square_of(square)));
        squares.push(("left".to_string(), self.get_left_square_of(square)));
        squares.push(("upper".to_string(), self.get_upper_square_of(square)));
        squares.push(("lower".to_string(), self.get_lower_square_of(square)));

        squares.push(("upper_right".to_string(), self.get_upper_right_square_of(square)));
        squares.push(("lower_right".to_string(), self.get_lower_right_square_of(square)));
        squares.push(("upper_left".to_string(), self.get_upper_left_square_of(square)));
        squares.push(("lower_left".to_string(), self.get_lower_left_square_of(square)));

        squares
    }

    pub fn add_squares_history(&mut self, squares: Vec<Square>) {
        self.squares_history.push(squares);
    }

    pub fn back_squares_history(&mut self) {
        if self.squares_history.len() == 0 {
            return;
        }
        let squares = self.squares_history.pop().unwrap();

        for square in squares {
            let x = square.pos.x;
            let y = square.pos.y;
            self.squares[y][x] = square;
        }
    }

    pub fn to_string(&self) -> String {
        let mut board_string = String::new();
        for (y, line) in self.squares.iter().enumerate() {
            for (x, square) in line.iter().enumerate() {
                board_string += &square.to_string();
            }
            board_string += "\n";
        }

        board_string.trim_end_matches("\n").to_string()
    }

    // カーソルの部分は色が反転する
    pub fn to_string_with_cursor(&self, cursor: (usize, usize)) -> String {
        let mut board_string = String::new();
        for (y, line) in self.squares.iter().enumerate() {
            for (x, square) in line.iter().enumerate() {
                if cursor == (x, y) {
                    board_string += &"\x1b[7m";
                    board_string += &square.to_string();
                    board_string += &"\x1b[27m";
                } else {
                    board_string += &square.to_string();
                }
            }
            board_string += "\n";
        }

        board_string.trim_end_matches("\n").to_string()
    }
}
#[test]
fn test_board_new() {
    assert_eq!(Board::new(0, 0), Err("x or y is 0.".to_string()));
    assert!(Board::new(1, 1).is_ok());
    assert!(Board::new(10, 10).is_ok());
    assert!(Board::new(100, 100).is_ok());
}
#[test]
fn test_board_get_size() {
    assert_eq!(Board::new(1, 1).unwrap().get_size(), (1, 1));
    assert_eq!(Board::new(3, 3).unwrap().get_size(), (3, 3));
}
#[test]
fn test_board_get_square() {
    let board = Board::new(10, 10).unwrap();
    assert_eq!(board.get_square((10, 10)),
               None);
    assert_eq!(board.get_square((20, 20)),
               None);
    assert_eq!(board.get_square((0, 0)).unwrap(),
               &Square::new(0, 0));
    assert_eq!(board.get_square((9, 9)).unwrap(),
               &Square::new(9, 9));
    assert_eq!(board.get_square((3, 6)).unwrap(),
               &Square::new(3, 6));
}
#[test]
fn test_board_borrow() {
    let board = Board::new(10, 10).unwrap();
    let square = board.get_square((5, 5)).unwrap();
    *square.is_open.borrow_mut() = true;
    *square.is_mine.borrow_mut() = true;

    let square = board.get_square((6, 6)).unwrap();
    *square.is_open.borrow_mut() = true;
    *square.is_mine.borrow_mut() = true;
}
// get_{directon}_square_ofのテストも兼ねる
#[test]
fn test_board_get_around_squares_of() {
    let board = Board::new(10, 10).unwrap();
    let center_square = board.get_square((0, 0)).unwrap();
    let around_squares = board.get_around_squares_of(center_square);
    // for文で回して、ちゃんとnoneになっているか確認
    for (direction, square) in around_squares {
        match direction.as_str() {
            "lower_left" | "left" | "upper_left" |
            "upper" | "upper_right" => assert_eq!(square, None),
            _ => ()
        }
    }
    let center_square = board.get_square((9, 9)).unwrap();
    let around_squares = board.get_around_squares_of(center_square);
    // for文で回して、ちゃんとnoneになっているか確認
    for (direction, square) in around_squares {
        match direction.as_str() {
            "upper_right" | "right" | "lower_right" |
            "lower" | "lower_left" => assert_eq!(square, None),
            _ => ()
        }
    }
    let board = Board::new(3, 3).unwrap();
    let square = board.get_square((1, 1)).unwrap();
    let around_squares = board.get_around_squares_of(square);
    for (i, (direction, around_square)) in around_squares.iter().enumerate() {
        let number = match direction.as_str() {
            "right" => 1,
            "left" => 2,
            "upper" => 3,
            "lower" => 4,
            "upper_right" => 5,
            "lower_right" => 6,
            "upper_left" => 7,
            "lower_left" => 8,
            _ => 0
        };
        *around_square.unwrap().number.borrow_mut() = number;
        *around_square.unwrap().is_open.borrow_mut() = true;
    }
    assert_eq!(board.to_string().as_str(),
               "７３５\n\
                ２[]１\n\
                ８４６");
}
#[test]
fn test_board_to_string() {
    let board = Board::new(1, 1).unwrap();
    assert_eq!(board.to_string().as_str(), "[]");
    let board = Board::new(1, 2).unwrap();
    assert_eq!(board.to_string().as_str(),
               "[]\n[]");
    let board = Board::new(3, 3).unwrap();
    assert_eq!(board.to_string().as_str(),
               "[][][]\n\
                [][][]\n\
                [][][]");
    *board.get_square((0, 0)).unwrap().is_flag.borrow_mut() = true;
    *board.get_square((0, 2)).unwrap().is_flag.borrow_mut() = true;
    *board.get_square((1, 1)).unwrap().is_open.borrow_mut() = true;
    *board.get_square((2, 2)).unwrap().is_open.borrow_mut() = true;
    *board.get_square((2, 2)).unwrap().number.borrow_mut() = 2;
    assert_eq!(board.to_string().as_str(),
               "\x1b[93m/>\x1b[0m[][]\n\
                []  []\n\
                \x1b[93m/>\x1b[0m[]２");
}
#[test]
fn test_board_to_string_with_cursor() {
    let board = Board::new(1, 1).unwrap();
    assert_eq!(board.to_string_with_cursor((0, 0)).as_str(),
               "\x1b[7m[]\x1b[27m");
    let board = Board::new(3, 3).unwrap();
    assert_eq!(board.to_string_with_cursor((1, 1)).as_str(),
               "[][][]\n\
                []\x1b[7m[]\x1b[27m[]\n\
                [][][]");
    assert_eq!(board.to_string_with_cursor((2, 2)).as_str(),
               "[][][]\n\
                [][][]\n\
                [][]\x1b[7m[]\x1b[27m");
}