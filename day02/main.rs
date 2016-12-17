extern crate advtools;

#[derive(Clone, Copy, Debug)]
enum Direction { U, R, D, L }
use Direction::*;

#[derive(Clone, Copy)]
struct Button(i32);

trait Keypad {
    fn next(Button, Direction) -> Button;
}

struct NormalKeypad;
struct FancyKeypad;

impl Keypad for NormalKeypad {
    // 1 2 3
    // 4 5 6
    // 7 8 9
    fn next(btn: Button, dir: Direction) -> Button {
        match (btn.0, dir) {
            (1...3, U) => btn,
            (4...9, U) => Button(btn.0 - 3),
            (7...9, D) => btn,
            (1...6, D) => Button(btn.0 + 3),
            (3, R) | (6, R) | (9, R) => btn,
            (1...2, R) | (4...5, R) | (7...8, R) => Button(btn.0 + 1),
            (1, L) | (4, L) | (7, L) => btn,
            (2...3, L) | (5...6, L) | (8...9, L) => Button(btn.0 - 1),
            _ => panic!("invalid next button: {:?}, {:?}", btn.0, dir)
        }
    }
}

impl Keypad for FancyKeypad {
    //       1
    //    2  3  4
    // 5  6  7  8  9
    //   10 11 12
    //      13
    fn next(btn: Button, dir: Direction) -> Button {
        match (btn.0, dir) {
            (5, U) | (2, U) | (1, U) | (4, U) | (9, U)    => btn,
            (6...8, U) | (10...12, U)                     => Button(btn.0 - 4),
            (3, U) | (13, U)                              => Button(btn.0 - 2),

            (5, D) | (10, D) | (13, D) | (12, D) | (9, D) => btn,
            (2...4, D) | (6...8, D)                       => Button(btn.0 + 4),
            (1, D) | (11, D)                              => Button(btn.0 + 2),

            (1, R) | (4, R) | (9, R) | (12, R) | (13, R)  => btn,
            (2...3, R) | (5...8, R) | (10...11, R)        => Button(btn.0 + 1),

            (1, L) | (2, L) | (5, L) | (10, L) | (13, L)  => btn,
            (3...4, L) | (6...9, L) | (11...12, L)        => Button(btn.0 - 1),

            _ => panic!("invalid next button: {:?}, {:?}", btn.0, dir)
        }
    }
}

fn find_code<K: Keypad>() -> String {
    let mut code = String::new();
    let mut btn = Button(5);
    for line in advtools::iter_input::<String>() {
        for ch in line.chars() {
            let dir = match ch {
                'U' => U,
                'R' => R,
                'D' => D,
                'L' => L,
                _ => panic!("invalid direction {:?}", ch)
            };
            btn = K::next(btn, dir);
        }
        code.push_str(&format!("{:X}", btn.0));
    }
    code
}


fn main() {
    println!("Code (normal keypad): {}", find_code::<NormalKeypad>());
    println!("Code (fancy keypad): {}", find_code::<FancyKeypad>());
}