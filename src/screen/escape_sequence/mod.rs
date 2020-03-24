macro_rules! EscapeSequenceBool {
    ($macro_name:ident, $enable:expr, $disable:expr) => {
        #[macro_export]
        macro_rules! $macro_name {
            (enable) => {
                $enable
            };
            (disable) => {
                $disable
            };
        }
    };
}

macro_rules! EscapeSequence {
    ($macro_name:ident, $escape_sequence:expr) => {
        #[macro_export]
        macro_rules! $macro_name {
            () => {
                $escape_sequence
            };
        }
    };
}

#[macro_export]
macro_rules! move_cursor {
    ($row:expr, $col:expr) => {
        format!("\x1b[{};{}H", $row, $col)
    };
}


#[macro_export]
macro_rules! foreground {
    (black) =>   { "\x1b[30m" };
    (red) =>     { "\x1b[31m" };
    (green) =>   { "\x1b[32m" };
    (yellow) =>  { "\x1b[33m" };
    (blue) =>    { "\x1b[34m" };
    (magenta) => { "\x1b[35m" };
    (cyan) =>    { "\x1b[36m" };
    (white) =>   { "\x1b[37m" };
    (default) => { "\x1b[39m" };
}

#[macro_export]
macro_rules! background {
    (black) =>   { "\x1b[40m" };
    (red) =>     { "\x1b[41m" };
    (green) =>   { "\x1b[42m" };
    (yellow) =>  { "\x1b[43m" };
    (blue) =>    { "\x1b[44m" };
    (magenta) => { "\x1b[45m" };
    (cyan) =>    { "\x1b[46m" };
    (white) =>   { "\x1b[47m" };
    (default) => { "\x1b[49m" };
}

EscapeSequenceBool!(alternate_screen, "\x1b[?1049h", "\x1b[?1049l");
EscapeSequenceBool!(hide_cursor, "\x1b[?25l", "\x1b[?25h");
EscapeSequenceBool!(color_reverse, "\x1b[7m", "\x1b[27m");
EscapeSequence!(home_cursor, "\x1b[H");
EscapeSequence!(clear, "\x1b[2J");


#[test]
#[ignore]
fn test_escape_sequence() {
    hide_cursor!(enable);
    println!("{}aiueo{}", background!(red), background!(default));
}
