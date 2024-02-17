// What is the Big O of the below function? (Hint, you may want to go line by line)

/// O(n) - Linear Time
/// The loop runs as many times as the length of the input, so it's O(n).
/// The function _another_function() is called as many times as the length of the input, so it's O(n).
/// The function _another_function is a constant time operation, O(1), but it's called n times due to the loop, so it's also O(n) in the context of _fun_challenge.
/// When calculating Big O, we take the highest order of complexity. In this case, it's O(n). So, the overall time complexity of the function _fun_challenge is O(n).
pub fn _fun_challenge(input: Vec<i32>) {
    let mut _a = 10; // O(1)
    _a = 50 + 3; // O(1)

    for _ in 0..input.len() {
        // O(n)
        _another_function(); // O(n)
        let _stranger = true; // O(n)
        _a += 1; // O(n)
    }
    // O(1)
    println!("{}", _a); // O(1)
}

fn _another_function() {
    // O(1)
    println!("Hello World"); // O(1)
}
