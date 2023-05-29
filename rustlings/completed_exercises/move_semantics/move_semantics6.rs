// move_semantics6.rs
// Execute `rustlings hint move_semantics6` or use the `hint` watch subcommand for a hint.
// You can't change anything except adding or removing references.

fn main() {
    let data = "Rust is great!".to_string();

    get_char(&data);

    string_uppercase(data);
}

// Created references here so value is borrowed instead of moved
fn get_char(data: &String) -> char {
    data.chars().last().unwrap()
}

// This takes ownership instead of referencing
fn string_uppercase(mut data: String) {
    data = data.to_uppercase();

    println!("{}", data);
}
