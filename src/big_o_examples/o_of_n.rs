// what is the Big O of find_nemo?
// how does the efficiency of find_nemo change as the size of the input grows?
// O(n) - Linear Time
pub fn _find_nemo(nemo: Vec<&str>) {
    for n in nemo {
        if n == "nemo" {
            println!("Found NEMO!");
        }
    }
}

// Simplify Big O - Rule 1: Worst Case
/// even though the function is efficient when the first element of the input is "nemo", it's still O(n) because the function has to iterate through the entire input to find "nemo".
pub fn find_nemo_break(nemo: Vec<&str>) {
    for n in nemo {
        println!("Running");
        if n == "nemo" {
            println!("Found NEMO!");
            break;
        }
    }
}
