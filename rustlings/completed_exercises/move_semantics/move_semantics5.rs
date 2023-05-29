// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x; // y is only a reference to mut x
    *y += 100; // so changing y here actually changes x
    let z = &mut x; // you have to finish borrowing x in regard to y before you can borrow with z
    *z += 1000; // same as y here, x itself is now 1200
    assert_eq!(x, 1200);
    println!("x = {}", x);
}
