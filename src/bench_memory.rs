use std::fs::File;
use std::io::{Write, BufWriter, Read};
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Instant;
use rayon::prelude::*;

pub struct MemoryResult {
    pub write_speed: f64,
    pub read_speed: f64,
}

pub fn run_memory_benchmark(size_gb: usize) -> MemoryResult {
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