# System Benchmark Tool

A comprehensive system benchmarking utility that measures CPU, memory, and disk performance across multiple dimensions. This tool is designed to provide detailed insights into your system's computational capabilities and I/O performance.

## Features

### CPU Performance Testing
- Single-core performance evaluation using:
  - Monte Carlo PI calculation
  - Prime number generation
- Multi-core performance testing utilizing all physical CPU cores
- Automatic core detection and scaling

### Memory Testing
- Sequential read/write speed measurements
- Random access read/write performance
- Memory latency testing through pointer chasing
- Tests performed with configurable memory sizes (default 8GB)

### Disk I/O Testing
- Large file operations (default 4GB):
  - Sequential read/write speeds
  - Throughput measurements in GB/s
- Small file operations (4KB):
  - Random read/write performance
  - IOPS (Input/Output Operations Per Second)
  - Performance testing with configurable iteration counts

## Output

Results are presented in a clear, tabulated format showing:
- Detailed CPU performance metrics across single and multi-core operations
- Memory bandwidth in GB/s and latency in nanoseconds
- Disk performance metrics for both large sequential operations and small random operations

## Use Cases

- System performance evaluation
- Hardware comparison
- Performance regression testing
- System capacity planning
- Hardware troubleshooting
- Performance optimization baseline measurements

## Notes

- The benchmark requires sufficient free memory and disk space to run effectively
- Results may vary based on system load and background processes
- For most accurate results, close other applications before running the benchmark
- The tool automatically scales tests based on the number of CPU cores available