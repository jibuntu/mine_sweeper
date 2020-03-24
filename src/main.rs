use std::env;
use std::str::FromStr;

extern crate libc;

mod screen;
mod game;

use screen::Screen;
use game::Game;
use std::os::raw::c_ushort;


#[repr(C)]
struct Winsize {
    ws_row: c_ushort,
    ws_col: c_ushort,
    ws_xpixel: c_ushort,
    ws_ypixel: c_ushort,

}

fn get_terminal_width() -> usize{
    let w = Winsize { ws_row: 0, ws_col: 0, ws_xpixel: 0, ws_ypixel: 0 };
    let r = unsafe { libc::ioctl(libc::STDOUT_FILENO, libc::TIOCGWINSZ, &w) };

    w.ws_col as usize
}

fn main() {
    let mut args = env::args().skip(1);
    let height = match args.next() {
        Some(arg) => match usize::from_str(arg.as_str()) {
            Ok(height) => height,
            Err(_) => return println!("Error: height cannot convert to usize")
        },
        None => return println!("Usage: command <height> <width> <mines>")
    };
    let width = match args.next() {
        Some(arg) => match usize::from_str(arg.as_str()) {
            Ok(width) => width,
            Err(_) => return println!("Error: width cannot convert to usize")
        },
        None => return println!("Usage: command <height> <width> <mines>")
    };
    let mines = match args.next() {
        Some(arg) => match usize::from_str(arg.as_str()) {
            Ok(mines) => mines,
            Err(_) => return println!("Error: mines cannot convert to usize")
        },
        None => return println!("Usage: command <height> <width> <mines>")
    };

    let mut game = match Game::new(height, width) {
        Ok(game) => game,
        Err(e) => return println!("Error: {}", e)
    };
    let terminal_width = get_terminal_width();
    //println!("{}", terminal_width);
    //return;
    let mut screen = Screen::new_with_terminal_width(terminal_width);
    //let mut screen = Screen::new_debug_mode();

    game.set_mines((height * width) / mines);
    screen.set_board(game.board_to_string());
    screen.set_top_bar(game.get_score().to_string());

    loop {
        screen.set_board(game.board_to_string());
        screen.print();
        match screen.read_key() {
            'n' => game.cursor_left(),
            'o' => game.cursor_right(),
            'r' => game.cursor_up(),
            'i' => game.cursor_down(),
            'N' => game.cursor_home(),
            'O' => game.cursor_end(),
            'R' => game.cursor_top(),
            'I' => game.cursor_bottom(),
            'e' => {
                game.open(); // マスを開ける
                screen.set_top_bar(game.get_score().to_string());
            },
            'E' => {
                game.open_all_squares(); // すべてのマスを開ける
                screen.set_top_bar(game.get_score().to_string());
            }
            't' => {
                game.toggle_flag(); // フラッグの付け外し
                screen.set_top_bar(game.get_score().to_string());
            },
            'b' => {
                game.back_history(); // １つ前の状態に戻す
                screen.set_top_bar(game.get_score().to_string());
            }
            'q' => break,
            _ => ()
        }
    }
}

//

#[test]
#[ignore]
fn test_main() {
    let game = Game::new(10, 10).unwrap();
    let mut screen = Screen::new();

    screen.set_board(game.board_to_string());
    screen.print();
}
