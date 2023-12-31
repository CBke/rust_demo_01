extern crate to_vec;
use crate::to_vec::ToVec;
use std::ops::Range;
fn main() {
    const TERM_WIDTH: usize = 241;
    const WORD: &str = "hello";

    let array = Range {
        start: 0,
        end: TERM_WIDTH - WORD.len(),
    }
    .map(|i| format!("{}{WORD}", " ".repeat(i)))
    .to_vec();

    loop {
        array.iter().for_each(|x| println!("{x}"));

        array
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .for_each(|x| println!("{x}"));
    }
}
