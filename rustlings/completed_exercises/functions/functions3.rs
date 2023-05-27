// functions3.rs
// Execute `rustlings hint functions3` or use the `hint` watch subcommand for a hint.

fn main() {
    nantucket(2);
}

fn nantucket(num: u8) {
    for i in 0..num {
        if i == 1 {
            println!("There once was a man from Nantucket...");
        } else {
            println!("The rest of the story, well...f@!k it.");
        }
    }
}
