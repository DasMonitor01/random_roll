use std::cmp::max;
use std::num::NonZeroUsize;
use std::os;
use std::thread;
use std::thread::Thread;
use random::{Source, Xorshift128Plus};
use std::time::{Instant, SystemTime};

const AMOUNT_OF_CYCLES: usize = 1000000000;

fn main() {
    solution_v2();
}

// Advanced solution using multithreading
fn run_cycles(amount_of_cycles: usize) -> u64 {
    let mut sequence: Xorshift128Plus = random::default(
        unsafe {
            SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_unchecked().as_secs()
        }
    );

    let mut count_of_0: u64 = 0;
    let mut max_count_of_0: u64 = 0;

    for _ in 1..=amount_of_cycles {
        count_of_0 = 0;
        for _ in 1..=231 {
            let next = sequence.read_u64() % 4;
            if next == 0 { count_of_0 += 1}
        }
        max_count_of_0 = max(count_of_0, max_count_of_0);
    }

    max_count_of_0
}

 fn solution_v2(){
    let start = Instant::now();

    let amount_of_threads = thread::available_parallelism()
        .unwrap_or(
            unsafe {NonZeroUsize::new_unchecked(8)}.into()
        ).get();

    println!("Recomended amount of threads: {amount_of_threads}");

    let mut threads = Vec::with_capacity(amount_of_threads);

    let regular_amount_of_cycles = AMOUNT_OF_CYCLES/amount_of_threads;
    let residular_amount_of_cycles = regular_amount_of_cycles + AMOUNT_OF_CYCLES
        - regular_amount_of_cycles * amount_of_threads;

    let thread_closure = move || {run_cycles(regular_amount_of_cycles)};

    for _ in 1..amount_of_threads {
        threads.push(thread::spawn(thread_closure));
    }

    let own_max = run_cycles(residular_amount_of_cycles);

    let mut maxes: Vec<u64> = threads.into_iter().map(
        |t| t.join().unwrap()
    ).collect();

    maxes.push(own_max);

    let overall_max = *maxes.iter().max().unwrap();

     // Note: Due to the abstract nature of the randomization, a value of zero is associated with
     // getting a proper value of 1
     println!("Max ammount of 1: {}", overall_max);

    let stop = start.elapsed();
    println!("Time taken: {:?}", stop)
}

// primitive solution first written
// Duration ~ 600 s
fn solution_v1(){
    let start = Instant::now();
    let mut sequence: Xorshift128Plus = random::default(
        unsafe {
            SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_unchecked().as_secs()
        }
    );

    let mut count_of_0: u64 = 0;
    let mut max_count_of_0: u64 = 0;

    for _ in 1..=AMOUNT_OF_CYCLES{
        count_of_0 = 0;
        for _ in 1..=231 {
            let next = sequence.read_u64() % 4;
            if next == 0 { count_of_0 += 1}
        }
        max_count_of_0 = max(count_of_0, max_count_of_0);
    }


    // Note: Due to the abstract nature of the randomization, a value of zero is associated with
    // getting a proper value of 1
    println!("Max ammount of 1: {}", max_count_of_0);
    let stop = start.elapsed();
    println!("Time taken: {:?}", stop);
}