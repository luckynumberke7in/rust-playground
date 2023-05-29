// move_semantics2.rs
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand for a hint.

// Expected output:
// vec0 has length 3 content `[22, 44, 66]`
// vec1 has length 4 content `[22, 44, 66, 88]`

// Solution 1
// in this version, we are cloning vec0 immediately
// (to keep the value of vec0 accessable and avoid a move on it's data)
fn main() {
    let mut vec0 = Vec::new();

    // Do not move the following line!
    let mut vec1 = fill_vec(vec0.clone());

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
/*
    Solution 2 -- compiles
    here we are taking the reference to vec inside the function and cloning it to create an OWNED (not ref) mutable vec
    then mutating it within the function and returning that value

fn main() {
    let vec0 = Vec::new();

    let mut vec1 = fill_vec(&vec0);

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec1.push(88);

    println!("{} has length {} content `{:?}`", "vec1", vec1.len(), vec1);
}

fn fill_vec(vec: &Vec<i32>) -> Vec<i32> {
    let mut vec = vec.clone();

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
*/

/*  Solution 3 -- compiles
    here you are removing the need for vec1 and a mutable declaration inside the fucntion
fn main() {
    let mut vec0: Vec<i32> = Vec::new();

    // Do not change the following line!
    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);

    vec0.push(88);

    println!("{} has length {} content `{:?}`", "vec0", vec0.len(), vec0);
}

fn fill_vec(vec: &mut Vec<i32>) ->  &mut Vec<i32> {

    vec.push(22);
    vec.push(44);
    vec.push(66);

    vec
}
*/
