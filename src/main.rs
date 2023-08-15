use core::array::from_fn;
use std::io::{stdout, Write};

fn main() {
    const TERM_WIDTH: usize = 241;
    const WORD: &str = "hello";
    const FILL: &str = " ";

    let array: [String; TERM_WIDTH - WORD.len()] =
        from_fn(|i| format!("{}{WORD}\r\n", FILL.repeat(i)));

    let mut lock = stdout().lock();
    loop {
        array.iter()
            .for_each(|x| write!(lock, "{x}").unwrap());

        array.iter()
            .skip(1)
            .rev()
            .skip(1)
            .for_each(|x| write!(lock, "{x}").unwrap());
    }
}
