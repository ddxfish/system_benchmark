use std::time::Instant;
use rayon::prelude::*;

pub struct MemoryResult {
    pub sequential_write_speed: f64,  // GB/s
    pub sequential_read_speed: f64,   // GB/s
    pub random_write_speed: f64,      // GB/s
    pub random_read_speed: f64,       // GB/s
    pub latency_ns: f64,             // Average latency in nanoseconds
}

const CHUNK_SIZE: usize = 1024 * 1024;  // 1MB chunks
const LATENCY_ARRAY_SIZE: usize = 1024 * 1024;  // Size for pointer chasing
const LATENCY_ITERATIONS: usize = 10_000_000;   // Number of random accesses

fn measure_sequential_write(size_gb: usize) -> f64 {
    let total_size = size_gb * 1024 * 1024 * 1024;
    let mut data = vec![0u8; total_size];
    let test_value = 42u8;
    
    let start = Instant::now();
    for chunk in data.chunks_mut(CHUNK_SIZE) {
        chunk.fill(test_value);
    }
    let duration = start.elapsed().as_secs_f64();
    
    size_gb as f64 / duration
}

fn measure_sequential_read(size_gb: usize) -> f64 {
    let total_size = size_gb * 1024 * 1024 * 1024;
    let data = vec![42u8; total_size];
    let mut sum = 0u64;
    
    let start = Instant::now();
    for chunk in data.chunks(CHUNK_SIZE) {
        sum += chunk.iter().map(|&x| x as u64).sum::<u64>();
    }
    let duration = start.elapsed().as_secs_f64();
    
    // Prevent compiler from optimizing away the read
    if sum == 0 {
        println!("Unexpected sum");
    }
    
    size_gb as f64 / duration
}

fn measure_random_write(size_gb: usize) -> f64 {
    let total_size = size_gb * 1024 * 1024 * 1024;
    let mut data = vec![0u8; total_size];
    let mut rng = fastrand::Rng::new();
    let test_value = 42u8;
    
    let start = Instant::now();
    for _ in 0..total_size / CHUNK_SIZE {
        let offset = (rng.usize(..total_size / CHUNK_SIZE)) * CHUNK_SIZE;
        data[offset..offset + CHUNK_SIZE].fill(test_value);
    }
    let duration = start.elapsed().as_secs_f64();
    
    size_gb as f64 / duration
}

fn measure_random_read(size_gb: usize) -> f64 {
    let total_size = size_gb * 1024 * 1024 * 1024;
    let data = vec![42u8; total_size];
    let mut rng = fastrand::Rng::new();
    let mut sum = 0u64;
    
    let start = Instant::now();
    for _ in 0..total_size / CHUNK_SIZE {
        let offset = (rng.usize(..total_size / CHUNK_SIZE)) * CHUNK_SIZE;
        sum += data[offset..offset + CHUNK_SIZE].iter().map(|&x| x as u64).sum::<u64>();
    }
    let duration = start.elapsed().as_secs_f64();
    
    if sum == 0 {
        println!("Unexpected sum");
    }
    
    size_gb as f64 / duration
}

fn measure_memory_latency() -> f64 {
    // Create indices for pointer chasing
    let mut rng = fastrand::Rng::new();
    let mut next_indices = vec![0; LATENCY_ARRAY_SIZE];
    
    // Create random permutation for pointer chasing
    let mut available: Vec<usize> = (0..LATENCY_ARRAY_SIZE).collect();
    let mut current = 0;
    
    // Build the circular linked list
    for i in 0..LATENCY_ARRAY_SIZE - 1 {
        let idx = rng.usize(..available.len());
        let next = available.swap_remove(idx);
        next_indices[current] = next;
        current = next;
    }
    // Complete the circle
    next_indices[current] = next_indices[0];
    
    // Chase pointers and measure time
    let start = Instant::now();
    let mut current = 0;
    for _ in 0..LATENCY_ITERATIONS {
        current = next_indices[current];
    }
    let duration = start.elapsed();
    
    // Prevent compiler optimization
    if current == usize::MAX {
        println!("Unexpected index");
    }
    
    // Calculate average latency in nanoseconds
    (duration.as_nanos() as f64) / (LATENCY_ITERATIONS as f64)
}

pub fn run_memory_benchmark(size_gb: usize) -> MemoryResult {
    println!("Running sequential write test...");
    let seq_write = measure_sequential_write(size_gb);
    println!("Sequential write: {:.2} GB/s", seq_write);
    
    println!("Running sequential read test...");
    let seq_read = measure_sequential_read(size_gb);
    println!("Sequential read: {:.2} GB/s", seq_read);
    
    println!("Running random write test...");
    let rand_write = measure_random_write(size_gb);
    println!("Random write: {:.2} GB/s", rand_write);
    
    println!("Running random read test...");
    let rand_read = measure_random_read(size_gb);
    println!("Random read: {:.2} GB/s", rand_read);
    
    println!("Running memory latency test...");
    let latency = measure_memory_latency();
    println!("Memory latency: {:.2} ns", latency);
    
    MemoryResult {
        sequential_write_speed: seq_write,
        sequential_read_speed: seq_read,
        random_write_speed: rand_write,
        random_read_speed: rand_read,
        latency_ns: latency,
    }
}