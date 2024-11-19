use rand::Rng;
use std::time::Instant;
use rayon::prelude::*;
use std::io::{self, Read, Write, BufWriter, Seek, SeekFrom};
use std::fs::File;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use prettytable::{Table, row};

struct CpuResult {
    single_core_pi: f64,
    single_core_time: f64,
    multi_core_pi: f64,
    multi_core_time: f64,
    thread_count: usize,
}

struct MemoryResult {
    write_speed: f64,
    read_speed: f64,
}

struct DiskResult {
    large_write_speed: f64,
    large_read_speed: f64,
    small_write_iops: f64,
    small_read_iops: f64,
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

fn run_memory_test(size_gb: usize) -> MemoryResult {
    let path = Path::new("memtest.tmp");
    let file_size = size_gb * 1024 * 1024 * 1024;
    let chunk_size = 64 * 1024 * 1024; // 64MB chunks
    let data = vec![42u8; chunk_size];
    
    println!("Running memory write test...");
    let start = Instant::now();
    {
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        for _ in 0..(file_size / chunk_size) {
            writer.write_all(&data).unwrap();
        }
        writer.flush().unwrap();
    }
    let write_speed = size_gb as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());

    println!("Running memory read test...");
    let start = Instant::now();
    let total = AtomicU64::new(0);
    let chunks = file_size / chunk_size;
    
    (0..chunks).into_par_iter().for_each(|_| {
        let mut local_buf = vec![0u8; chunk_size];
        let file = File::open(&path).unwrap();
        let n = file.try_clone().unwrap().read(&mut local_buf).unwrap();
        let local_sum = local_buf[0..n].iter().map(|&x| x as u64).sum::<u64>();
        total.fetch_add(local_sum, Ordering::Relaxed);
    });
    
    let read_speed = size_gb as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());
    
    std::fs::remove_file(path).unwrap();
    
    MemoryResult {
        write_speed,
        read_speed,
    }
}

fn run_disk_test() -> DiskResult {
    // Large file test (4GB)
    println!("Running large file write test...");
    let path = Path::new("disktest.tmp");
    //let large_size = 4 * 1024 * 1024 * 1024;
    let data = vec![42u8; 1024 * 1024]; // 1MB chunks
    
    let start = Instant::now();
    {
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        for _ in 0..4096 {  // 4096 chunks of 1MB = 4GB
            writer.write_all(&data).unwrap();
        }
        writer.flush().unwrap();
    }
    let large_write_speed = 4.0 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());

    println!("Running large file read test...");
    let start = Instant::now();
    {
        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 1024 * 1024];
        for _ in 0..4096 {
            file.read_exact(&mut buffer).unwrap();
        }
    }
    let large_read_speed = 4.0 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());
    
    std::fs::remove_file(path).unwrap();

    // Small file test (4K operations)
    println!("Running 4K writes test...");
    let small_data = vec![42u8; 4096];
    let iterations = 100000;
    
    let start = Instant::now();
    {
        let mut file = File::create(&path).unwrap();
        for i in 0..iterations {
            file.seek(SeekFrom::Start((i * 4096) as u64)).unwrap();
            file.write_all(&small_data).unwrap();
            file.flush().unwrap();
        }
    }
    let small_write_iops = iterations as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());

    println!("Running 4K reads test...");
    let start = Instant::now();
    {
        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 4096];
        for i in 0..iterations {
            file.seek(SeekFrom::Start((i * 4096) as u64)).unwrap();
            file.read_exact(&mut buffer).unwrap();
        }
    }
    let small_read_iops = iterations as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());
    
    std::fs::remove_file(path).unwrap();

    DiskResult {
        large_write_speed,
        large_read_speed,
        small_write_iops,
        small_read_iops,
    }
}

fn main() {
    let single_iterations = 10_000_000_000;
    let multi_iterations = single_iterations * 20;
    let n_threads = num_cpus::get();

    // CPU Tests
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

    let cpu_result = CpuResult {
        single_core_pi: single_pi,
        single_core_time: single_time,
        multi_core_pi: multi_pi,
        multi_core_time: multi_time,
        thread_count: n_threads,
    };

    // Memory Test
    let mem_result = run_memory_test(4);
    
    // Disk Test
    let disk_result = run_disk_test();

    // Results Output
    println!("\n=== CPU Test Results ===");
    let mut table = Table::new();
    table.add_row(row!["", "Time (s)"]);
    table.add_row(row!["Single Core", format!("{:.2}", cpu_result.single_core_time)]);
    table.add_row(row![format!("Multi Core ({})", cpu_result.thread_count), 
        format!("{:.2}", cpu_result.multi_core_time)]);
    table.printstd();

    println!("\n=== Memory Test Results ===");
    let mut table = Table::new();
    table.add_row(row!["Speed (GB/s)", "Read", "Write"]);
    table.add_row(row!["", 
        format!("{:.2}", mem_result.read_speed),
        format!("{:.2}", mem_result.write_speed)]);
    table.printstd();

    println!("\n=== Disk Test Results ===");
    let mut table = Table::new();
    table.add_row(row!["", "Read", "Write"]);
    table.add_row(row!["4GB (GB/s)", 
        format!("{:.2}", disk_result.large_read_speed),
        format!("{:.2}", disk_result.large_write_speed)]);
    table.add_row(row!["4K (IOPS)", 
        format!("{:.0}", disk_result.small_read_iops),
        format!("{:.0}", disk_result.small_write_iops)]);
    table.printstd();

    println!("\nPress any key to exit...");
    io::stdin().read_exact(&mut [0u8]).unwrap();
}