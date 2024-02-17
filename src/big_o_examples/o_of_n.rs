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
