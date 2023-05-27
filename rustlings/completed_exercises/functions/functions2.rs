// functions2.rs
// Execute `rustlings hint functions2` or use the `hint` watch subcommand for a hint.

fn main() {
    let sparta = 300;
    this_is(sparta);
}

fn this_is(num: u16) {
    for i in 0..num {
        println!("King Leonidas and {i} of his men faught against domination.");
    }
}
