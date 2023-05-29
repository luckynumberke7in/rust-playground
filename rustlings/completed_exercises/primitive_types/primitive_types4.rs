// primitive_types4.rs
// Get a slice out of Array a where the ??? is so that the test passes.
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand for a hint.

#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    // make nice_slice an array referencing array a @ indexes 1-3
    let nice_slice = &a[1..4];

    // with a more readable syntax imo
    let with_equals = &a[1..=3];

    fn test_slice(slice: &[u8]) {
        assert_eq!([2, 3, 4], slice)
    }
    test_slice(nice_slice);
    test_slice(with_equals);
}
