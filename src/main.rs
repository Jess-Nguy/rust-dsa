// use std::thread::sleep;
// use std::time::{Duration, Instant};
use std::time::Instant;
mod big_o_examples;
mod exercises;
use big_o_examples::o_of_1::*;
use big_o_examples::o_of_n::*;

fn main() {
    // let nemo = vec!["nemo"];
    let everyone = vec![
        "dory", "bruce", "marlin", "nemo", "gill", "bloat", "nigel", "squirt", "darla", "hank",
    ];

    // let large = (0..100).map(|_| "nemo").collect::<Vec<&str>>();
    // find_nemo_measure_time(large);
    // find_nemo(large);

    let boxes = vec![0, 1, 2, 3, 4, 5];
    log_first_two_boxes(boxes);

    find_nemo_break(everyone)
}

/// We initially think efficiency is about speed, but it's really about scale
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
