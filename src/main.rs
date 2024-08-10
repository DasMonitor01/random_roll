use std::cmp::max;
use std::num::NonZeroUsize;
use std::thread;
use std::time::{Instant, SystemTime};

use random::{Source, Xorshift128Plus};

const AMOUNT_OF_CYCLES: usize = 1000000000;

fn main() {
    solution_v2();
}

// Advanced solution using multithreading

/// This is the function to be run by a single thread of the program
/// It takes the amount of cycles to be done by the thread as input
/// and returns the maximum amount of zeros it got during those cycles.
/// The implementation is the original implementation utilized in V1 of the program (without multithreading)
fn run_cycles(amount_of_cycles: usize) -> u64 {
    let mut sequence: Xorshift128Plus = random::default(
            // The unsafe unwrap is allowed because UNIX_EPOCH will always be less then or equal to
            // the current time
        unsafe {
            SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_unchecked().as_secs()
        }
    );


    let mut max_count_of_0: u64 = 0;

    for _ in 1..=amount_of_cycles {
        let mut count_of_0: u64 = 0;
        for _ in 1..=231 {
            // Minor note: Rust already utilizes bitwise
            // anding for calculating modulo of powers of two,
            // so x % 4 is performance-wise equivalent to x & 3
            let next = sequence.read_u64() % 4;
            if next == 0 { count_of_0 += 1 }
        }
        max_count_of_0 = max(count_of_0, max_count_of_0);
    }

    max_count_of_0
}


/// The main code of the second version of this solution
/// Uses multiple threads to speed up the process
/// It utilizes thread::available_parallelism to try to optimize the amount of threads
/// that will be used. Based upon this versions speed will vary based on the end-users architecture
/// Each thread performs the basic solution of the problem
pub fn solution_v2() {
    let start = Instant::now();

    let amount_of_threads = thread::available_parallelism()
        .unwrap_or(
            // This unchecked casting is valid because 8 is not 0
            unsafe { NonZeroUsize::new_unchecked(8) }.into()
        ).get();

    println!("Recommended amount of threads: {amount_of_threads}");

    let mut threads = Vec::with_capacity(amount_of_threads);

    // These amounts are split equally, however due to division residue, some cycles would be
    // lost, so we add these in residual_amount_of_cycles
    // In total the following statements hold:
    // (i) AMOUNT_OF_CYCLES = (amount_of_threads - 1) * regular_amount_of_cycles + residual_amount_of_cycles
    // (ii) residual_amount_of_cycles = regular_amount_of_cycles + AMOUNT_OF_CYCLES mod amount_of_threads
    // (iii) For large values of AMOUNT_OF_CYCLES residual_amount_of_cycles ≈ regular_amount_of_cycles
    let regular_amount_of_cycles = AMOUNT_OF_CYCLES / amount_of_threads;
    let residual_amount_of_cycles = AMOUNT_OF_CYCLES
        - regular_amount_of_cycles * (amount_of_threads - 1);



    let thread_closure = move || { run_cycles(regular_amount_of_cycles) };

    // we wan't all but one chunk to be handled by separate threads so in total
    // amount_of_threads - 1 threads
    for _ in 1..amount_of_threads {
        threads.push(thread::spawn(thread_closure));
    }

    // runs the last chunk on the main thread
    let own_max = run_cycles(residual_amount_of_cycles);

    let mut maxes: Vec<u64> = threads.into_iter().map(
        |t| t.join().unwrap()
    ).collect();

    maxes.push(own_max);

    let overall_max = *maxes.iter().max().unwrap();

// Note: Due to the abstract nature of the randomization, a value of zero is associated with
// getting a proper value of 1
    println!("Max amount of 1: {}", overall_max);

    let stop = start.elapsed();
    println!("Time taken: {:?}", stop)
}

// primitive solution first written
// Duration ~ 600 s
pub fn solution_v1() {
    let start = Instant::now();
    let max_count_of_0 = run_cycles(AMOUNT_OF_CYCLES);


// Note: Due to the abstract nature of the randomization, a value of zero is associated with
// getting a proper value of 1
    println!("Max amount of 1: {}", max_count_of_0);
    let stop = start.elapsed();
    println!("Time taken: {:?}", stop);
}