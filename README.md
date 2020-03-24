# mine_sweeper

マインスイーパー

## [src/main.rs](https://github.com/jibuntu/mine_sweeper/blob/559ddc6215a81a9cd69129f13d0b734acdeaf227/src/main.rs#L71-L98)の71行目から98行目のコード
```Rust
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
```
