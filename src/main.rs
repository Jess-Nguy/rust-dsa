// use std::thread::sleep;
// use std::time::{Duration, Instant};
use std::time::Instant;

fn main() {
    // let nemo = vec!["nemo"];
    // let everyone = vec![
    //     "dory", "bruce", "marlin", "nemo", "gill", "bloat", "nigel", "squirt", "darla", "hank",
    // ];

    let large = (0..100).map(|_| "nemo").collect::<Vec<&str>>();
    // find_nemo_measure_time(large);
    find_nemo(large);
}

/// this is not important for Big O notation because the runtime will be different for everyone
fn _find_nemo_measure_time(nemo: Vec<&str>) {
    let t0 = Instant::now();
    for n in nemo {
        if n == "nemo" {
            println!("Found NEMO!");
        }
    }
    let t1 = Instant::now();
    println!("Call to find Nemo took {:?} mircoseconds", (t1 - t0));
}

// what is the Big O of find_nemo?
// how does the efficiency of find_nemo change as the size of the input grows?
// O(n) - Linear Time
fn find_nemo(nemo: Vec<&str>) {
    for n in nemo {
        if n == "nemo" {
            println!("Found NEMO!");
        }
    }
}
