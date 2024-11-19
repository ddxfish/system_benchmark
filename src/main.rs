use std::io::{self, Write};
use num_cpus;

mod bench_cpu;
mod bench_memory;
mod bench_disk;
mod report;

use bench_cpu::run_cpu_benchmark;
use bench_memory::run_memory_benchmark;
use bench_disk::run_disk_benchmark;
use report::generate_report;

fn pause_for_user() {
    print!("\nPress Enter to continue...");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
}

fn main() {
    let physical_cores = num_cpus::get_physical();

    // Configuration parameters
    let config = BenchmarkConfig {
        cpu_base_monte_carlo_iterations: 10_000_000_000,
        cpu_prime_count: 10_000_000, // 10 million primes
        cpu_physical_cores: physical_cores,
        memory_size_gb: 8,
        disk_large_file_gb: 4,
        disk_small_file_iterations: 100_000,
    };

    println!("Starting system benchmark...");
    println!("Detected {} physical cores", physical_cores);
    println!("\nWarning: This benchmark will allocate up to {}GB of RAM", config.memory_size_gb);
    pause_for_user();

    println!("\n============= Running Benchmarks =============\n");

    // Run benchmarks
    let cpu_result = run_cpu_benchmark(
        config.cpu_base_monte_carlo_iterations,
        config.cpu_prime_count,
        config.cpu_physical_cores,
    );

    let mem_result = run_memory_benchmark(config.memory_size_gb);
    
    let disk_result = run_disk_benchmark(
        config.disk_large_file_gb,
        config.disk_small_file_iterations,
    );

    println!("\n============= Benchmark Results =============\n");
    
    // Generate report
    generate_report(&cpu_result, &mem_result, &disk_result);

    println!("\n============= Benchmark Complete =============");
    pause_for_user();
}

struct BenchmarkConfig {
    cpu_base_monte_carlo_iterations: u64,
    cpu_prime_count: usize,
    cpu_physical_cores: usize,
    memory_size_gb: usize,
    disk_large_file_gb: usize,
    disk_small_file_iterations: usize,
}