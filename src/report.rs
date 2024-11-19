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
    table.add_row(row!["", "Time (s)"]);
    table.add_row(row!["Single Core", format!("{:.2}", cpu_result.single_core_time)]);
    table.add_row(row![
        format!("Multi Core ({})", cpu_result.thread_count),
        format!("{:.2}", cpu_result.multi_core_time)
    ]);
    table.printstd();

    println!("\n=== Memory Test Results ===");
    let mut table = Table::new();
    table.add_row(row!["Speed (GB/s)", "Read", "Write"]);
    table.add_row(row![
        "",
        format!("{:.2}", mem_result.read_speed),
        format!("{:.2}", mem_result.write_speed)
    ]);
    table.printstd();

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