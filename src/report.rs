use prettytable::{Table, row};
use crate::bench_cpu::CpuResult;
use crate::bench_memory::MemoryResult;
use crate::bench_disk::DiskResult;

pub fn generate_report(
    cpu_result: &CpuResult,
    mem_result: &MemoryResult,
    disk_result: &DiskResult,
) {
    println!("\n=== CPU Test Results ===");
    let mut table = Table::new();
    
    // Header row with test types
    table.add_row(row![
        "Test Type",
        "Single Core (s)",
        format!("Multi Core ({}) (s)", cpu_result.thread_count)
    ]);

    // Monte Carlo PI test results
    table.add_row(row![
        "Monte Carlo PI",
        format!("{:.2}", cpu_result.single_core_monte_carlo_time),
        format!("{:.2}", cpu_result.multi_core_monte_carlo_time)
    ]);

    // Prime calculation test results
    table.add_row(row![
        format!("Prime Numbers ({})", cpu_result.prime_count),
        format!("{:.2}", cpu_result.single_core_primes_time),
        format!("{:.2}", cpu_result.multi_core_primes_time)
    ]);

    table.printstd();

    // Additional PI calculation results
    println!("\nPI Calculation Results:");
    println!("Single Core π ≈ {:.6}", cpu_result.single_core_monte_carlo_pi);
    println!("Multi Core π ≈ {:.6}", cpu_result.multi_core_monte_carlo_pi);

    println!("\n=== Memory Test Results ===");
    let mut table = Table::new();
    table.add_row(row!["Test Type", "Speed (GB/s)"]);
    table.add_row(row!["Sequential Read", format!("{:.2}", mem_result.sequential_read_speed)]);
    table.add_row(row!["Sequential Write", format!("{:.2}", mem_result.sequential_write_speed)]);
    table.add_row(row!["Random Read", format!("{:.2}", mem_result.random_read_speed)]);
    table.add_row(row!["Random Write", format!("{:.2}", mem_result.random_write_speed)]);
    table.printstd();
    
    println!("\nMemory Latency: {:.1} ns", mem_result.latency_ns);

    println!("\n=== Disk Test Results ===");
    let mut table = Table::new();
    table.add_row(row!["", "Read", "Write"]);
    table.add_row(row![
        "4GB (GB/s)",
        format!("{:.2}", disk_result.large_read_speed),
        format!("{:.2}", disk_result.large_write_speed)
    ]);
    table.add_row(row![
        "4K (IOPS)",
        format!("{:.0}", disk_result.small_read_iops),
        format!("{:.0}", disk_result.small_write_iops)
    ]);
    table.printstd();
}