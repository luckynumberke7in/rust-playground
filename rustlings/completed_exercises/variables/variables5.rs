// variables5.rs
// Execute `rustlings hint variables5` or use the `hint` watch subcommand for a hint.

fn main() {
    let number = "T-H-R-E-E"; // don't change this line
    println!("Spell a Number : {}", number);
    let number = 3; // don't rename this variable
    println!("The new number var is shadowing over the 1st, this is useful for re-using names inside scope. New number plus two is : {}", number + 2);
}
