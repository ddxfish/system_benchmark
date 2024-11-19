use rand::Rng;
use rayon::prelude::*;
use std::time::Instant;

pub struct CpuResult {
    pub single_core_monte_carlo_pi: f64,
    pub single_core_monte_carlo_time: f64,
    pub multi_core_monte_carlo_pi: f64,
    pub multi_core_monte_carlo_time: f64,
    pub single_core_primes_time: f64,
    pub multi_core_primes_time: f64,
    pub core_count: usize,
    pub prime_count: usize,
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

fn parallel_estimate_pi(iterations: u64, parallelism: usize) -> f64 {
    let chunk_size = iterations / parallelism as u64;
    let results: Vec<f64> = (0..parallelism)
        .into_par_iter()
        .map(|_| estimate_pi(chunk_size))
        .collect();
    
    results.iter().sum::<f64>() / parallelism as f64
}

fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    if n <= 3 {
        return true;
    }
    if n % 2 == 0 || n % 3 == 0 {
        return false;
    }
    
    let sqrt_n = (n as f64).sqrt() as u64;
    let mut i = 5;
    while i <= sqrt_n {
        if n % i == 0 || n % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}

fn calculate_primes_single(limit: usize) -> Vec<u64> {
    let mut primes = Vec::with_capacity(limit);
    let mut n = 2u64;
    
    while primes.len() < limit {
        if is_prime(n) {
            primes.push(n);
        }
        n += 1;
    }
    
    primes
}

fn calculate_primes_parallel(limit: usize, parallelism: usize) -> Vec<u64> {
    let chunk_size = (limit + parallelism - 1) / parallelism;
    let mut all_primes = Vec::with_capacity(limit);
    let mut n = 2u64;
    
    while all_primes.len() < limit {
        let candidates: Vec<u64> = (0..chunk_size as u64 * parallelism as u64)
            .into_par_iter()
            .map(|i| n + i)
            .filter(|&x| is_prime(x))
            .collect();
        
        all_primes.extend(candidates);
        n += (chunk_size * parallelism) as u64;
    }
    
    all_primes.truncate(limit);
    all_primes
}

pub fn run_cpu_benchmark(
    base_monte_carlo_iterations: u64,
    prime_count: usize,
    core_count: usize,
) -> CpuResult {
    println!("Running Monte Carlo PI single core test...");
    let start = Instant::now();
    let single_core_monte_carlo_pi = estimate_pi(base_monte_carlo_iterations);
    let single_core_monte_carlo_time = start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", single_core_monte_carlo_time);

    println!("Running Monte Carlo PI multi core test ({} cores)...", core_count);
    let start = Instant::now();
    let multi_core_monte_carlo_pi = parallel_estimate_pi(base_monte_carlo_iterations * core_count as u64 / 2, core_count);
    let multi_core_monte_carlo_time = start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", multi_core_monte_carlo_time);

    println!("Running prime calculation single core test...");
    let start = Instant::now();
    let _primes_single = calculate_primes_single(prime_count);
    let single_core_primes_time = start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", single_core_primes_time);

    println!("Running prime calculation multi core test ({} cores)...", core_count);
    let start = Instant::now();
    let _primes_multi_core = calculate_primes_parallel(prime_count * 4, core_count);
    let multi_core_primes_time = start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", multi_core_primes_time);

    CpuResult {
        single_core_monte_carlo_pi,
        single_core_monte_carlo_time,
        multi_core_monte_carlo_pi,
        multi_core_monte_carlo_time,
        single_core_primes_time,
        multi_core_primes_time,
        core_count,
        prime_count,
    }
}