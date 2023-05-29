// vecs2.rs
// A Vec of even numbers is given. Your task is to complete the loop
// so that each number in the Vec is multiplied by 2.
//
// Make me pass the test!
//
// Execute `rustlings hint vecs2` or use the `hint` watch subcommand for a hint.

// the 2 fn's below return the same vector, but follow different methods to get there.

// this version is mutably iterating over the vector 'v' and multiplying each element by 2
fn vec_loop(mut v: Vec<i32>) -> Vec<i32> {
    for element in v.iter_mut() {
        // multiplied by 2.
        *element *= 2;
    }

    // At this point, `v` should be equal to [4, 8, 12, 16, 20].
    v
}

// this version is iterating over v and returning each value doubled to a new vector
fn vec_map(v: &Vec<i32>) -> Vec<i32> {
    v.iter()
        .map(|element| {
            // TODO: Do the same thing as above - but instead of mutating the
            // Vec, you can just return the new number!
            element * 2
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vec_loop() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_loop(v.clone());

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }

    #[test]
    fn test_vec_map() {
        let v: Vec<i32> = (1..).filter(|x| x % 2 == 0).take(5).collect();
        let ans = vec_map(&v);

        assert_eq!(ans, v.iter().map(|x| x * 2).collect::<Vec<i32>>());
    }
}
