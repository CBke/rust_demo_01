use core::array::from_fn;
use std::io::{stdout, Write};

fn main() {
    const TERM_WIDTH: usize = 241;
    const WORD: &str = "hello";

    let array: [String; TERM_WIDTH - WORD.len()] =
        from_fn(|i| format!("{}{}\r\n", " ".repeat(i), WORD));

    let mut lock = stdout().lock();
    loop {
        for f in array.iter() {
            write!(lock, "{}", f).unwrap();
        }
        for f in array.iter().skip(1).rev().skip(1) {
            write!(lock, "{}", f).unwrap();
        }
    }
}
