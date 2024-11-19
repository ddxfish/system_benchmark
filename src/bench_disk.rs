use std::fs::File;
use std::io::{Write, BufWriter, Read, Seek, SeekFrom};
use std::path::Path;
use std::time::Instant;

pub struct DiskResult {
    pub large_write_speed: f64,
    pub large_read_speed: f64,
    pub small_write_iops: f64,
    pub small_read_iops: f64,
}

pub fn run_disk_benchmark(
    large_file_gb: usize,
    small_file_iterations: usize,
) -> DiskResult {
    let path = Path::new("disktest.tmp");
    let large_chunks = large_file_gb * 1024; // Number of 1MB chunks
    let data = vec![42u8; 1024 * 1024]; // 1MB chunks
    
    println!("Running large file write test...");
    let start = Instant::now();
    {
        let file = File::create(&path).unwrap();
        let mut writer = BufWriter::new(file);
        for _ in 0..large_chunks {
            writer.write_all(&data).unwrap();
        }
        writer.flush().unwrap();
    }
    let large_write_speed = large_file_gb as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());

    println!("Running large file read test...");
    let start = Instant::now();
    {
        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 1024 * 1024];
        for _ in 0..large_chunks {
            file.read_exact(&mut buffer).unwrap();
        }
    }
    let large_read_speed = large_file_gb as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());
    
    std::fs::remove_file(path).unwrap();

    println!("Running 4K writes test...");
    let small_data = vec![42u8; 4096];
    
    let start = Instant::now();
    {
        let mut file = File::create(&path).unwrap();
        for i in 0..small_file_iterations {
            file.seek(SeekFrom::Start((i * 4096) as u64)).unwrap();
            file.write_all(&small_data).unwrap();
            file.flush().unwrap();
        }
    }
    let small_write_iops = small_file_iterations as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());

    println!("Running 4K reads test...");
    let start = Instant::now();
    {
        let mut file = File::open(&path).unwrap();
        let mut buffer = vec![0u8; 4096];
        for i in 0..small_file_iterations {
            file.seek(SeekFrom::Start((i * 4096) as u64)).unwrap();
            file.read_exact(&mut buffer).unwrap();
        }
    }
    let small_read_iops = small_file_iterations as f64 / start.elapsed().as_secs_f64();
    println!("Completed in {:.2}s", start.elapsed().as_secs_f64());
    
    std::fs::remove_file(path).unwrap();

    DiskResult {
        large_write_speed,
        large_read_speed,
        small_write_iops,
        small_read_iops,
    }
}