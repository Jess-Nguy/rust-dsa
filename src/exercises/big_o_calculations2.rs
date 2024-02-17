// What is the Big O of the below function? (Hint, you may want to go line by line)\

/// O(n) - Linear Time
/// Big O = 4 + 7n = O(n)
/// The variable assignments a, b, c, and who_am_i outside the loops are constant time operations, O(1).
/// There are two separate loops that run as many times as the length of the input, so each loop is O(n).
/// Inside the first loop, all operations (variable assignments x, y, z) are constant time operations, but since they are in a loop that runs n times, they each become O(n).
/// Inside the second loop, all operations (variable assignments p, q) are also constant time operations, but since they are in a loop that runs n times, they each become O(n).
fn another_fun_challenge(input: Vec<i32>) {
    let a = 5; // O(1)
    let b = 10; // O(1)
    let c = 50; // O(1)

    // O(n)
    for i in 0..input.len() {
        let x = i + 1; // O(n)
        let y = i + 2; // O(n)
        let z = i + 3; // O(n)
    }
    for j in 0..input.len() {
        let p = j * 2; // O(n)
        let q = j * 2; // O(n)
    }
    let who_am_i = "I don't know"; // O(1)
}
