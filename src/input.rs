use std::io;

pub fn get_input() -> String {
    let mut out = String::new();

    io::stdin().read_line(&mut out).expect("Error reading input");

    out
}