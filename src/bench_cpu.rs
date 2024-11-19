use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

pub struct CpuResult {
    pub single_core_pi: f64,
    pub single_core_time: f64,
    pub multi_core_pi: f64,
    pub multi_core_time: f64,
    pub thread_count: usize,
}

fn estimate_pi(iterations: u64) -> f64 {
    let mut rng = rand::thread_rng();
    let mut inside = 0;

    for _ in 0..iterations {
        let x: f64 = rng.gen();
        let y: f64 = rng.gen();
        if x * x + y * y <= 1.0 {
            inside += 1;
        }
    }

    4.0 * (inside as f64) / (iterations as f64)
}

fn parallel_estimate_pi(iterations: u64, n_threads: usize) -> f64 {
    let chunk_size = iterations / n_threads as u64;
    let results: Vec<f64> = (0..n_threads)
        .into_par_iter()
        .map(|_| estimate_pi(chunk_size))
        .collect();
    
    results.iter().sum::<f64>() / n_threads as f64
}

pub fn run_cpu_benchmark(single_iterations: u64, multi_iterations: u64, n_threads: usize) -> CpuResult {
    println!("Running single core test...");
    let start = Instant::now();
    let single_pi = estimate_pi(single_iterations);
    let single_time = start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", single_time);

    println!("Running multi core test...");
    let start = Instant::now();
    let multi_pi = parallel_estimate_pi(multi_iterations, n_threads);
    let multi_time = start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", multi_time);

    CpuResult {
        single_core_pi: single_pi,
        single_core_time: single_time,
        multi_core_pi: multi_pi,
        multi_core_time: multi_time,
        thread_count: n_threads,
    }
}