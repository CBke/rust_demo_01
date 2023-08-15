fn main() {
    const TERM_WIDTH: usize = 241;
    const WORD: &str = "hello";
    
    let array: Vec<String> = (0..(TERM_WIDTH - WORD.len()))
        .map(|i| format!("{}{WORD}\r\n", " ".repeat(i)))
        .collect();

    loop {
        array.iter().for_each(|x| print!("{x}"));

        array
            .iter()
            .skip(1)
            .rev()
            .skip(1)
            .for_each(|x| print!("{x}"));
    }
}
