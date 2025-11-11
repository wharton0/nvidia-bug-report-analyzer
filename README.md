# NVIDIA Bug Report Log Analyzer

A high-performance NVIDIA bug report log analysis tool written in Rust for quickly diagnosing GPU system issues.

[中文文档](README_CN.md)

## Features

### Core Checks
- ✅ System information summary (CPU, memory, BIOS, motherboard)
- ✅ GPU information summary (model, quantity, PCI address, UUID)
- ✅ NVIDIA driver version conflict detection
- ✅ Xid error analysis (with detailed descriptions)
- ✅ Thermal throttling detection
- ✅ Segfault detection
- ✅ CPU throttling detection
- ✅ Hardware error detection
- ✅ GPU fallen off the bus detection
- ✅ RmInit failure detection
- ✅ Power state change refusal detection
- ✅ Bad CPU error detection

### Advanced Checks 🆕
- ⭐ **NVLink Error Check** - Detects Replay, Recovery, and CRC errors
- ⭐ **GPU Temperature Monitoring** - Temperature statistics and threshold warnings
- ⭐ **ECC Memory Errors** - DRAM correctable/uncorrectable error detection
- ⭐ **PCIe Link Status** - Detects link speed and width degradation with details
- ⭐ **GPU Power and Performance** - Power statistics and Persistence Mode check

### Final Summary
- 📊 Comprehensive summary of all checks at the end of the report
- ✅/❌ Clear visual indicators for system health status
- 🔢 Issue counts for each category

## Installation

### Prerequisites
- Rust 1.70 or higher
- Cargo (Rust package manager)

### Build
```bash
# Clone or download the project
cd nvidia_log_parser

# Build release version
cargo build --release

# Executable located at
./target/release/nvidia_log_parser
```

## Usage

### Basic Usage
```bash
# Use default log file name (nvidia-bug-report.log)
./target/release/nvidia_log_parser

# Specify log file
./target/release/nvidia_log_parser /path/to/nvidia-bug-report.log

# Specify XID errors CSV file
./target/release/nvidia_log_parser -x xid-errors.csv nvidia-bug-report.log
```

### Generate NVIDIA Bug Report
```bash
# On Linux systems
sudo nvidia-bug-report.sh

# This generates a file like nvidia-bug-report.log.gz
# Extract and analyze with this tool
gunzip nvidia-bug-report.log.gz
```

## Output Example

```
==================================================
     NVIDIA Bug Report Log Summary
==================================================

Log from: Tue Oct 14 10:00:52 AM UTC 2025
NVIDIA Driver Version: 550.144.03
Chassis DMI: Dell Inc. PowerEdge XE9680
...

Summary of PCI Addresses and GPUs
00000000:19:00.0 GPU-39d52d8d-06f2-1986-31e4-9e01ffdafda3
00000000:3B:00.0 GPU-4976ff65-ea04-2402-9a7c-09817bf2e396
...

==================================================
     NVLink Status Check
==================================================

No NVLink errors detected - All links healthy

==================================================
     GPU Temperature Check
==================================================

GPU Temperatures:
   Min: 33°C, Max: 38°C, Avg: 35°C
   Temperatures are within normal range

==================================================
     PCIe Link Status Check
==================================================

** PCIe Link Degradation Detected: 11 instances
   Some PCIe links are running at reduced speed or width.

Degraded Links:
   0000:00:0c.0 - PCI bridge
      Current: 5GT/s (downgraded), x1 (ok) (Expected: 8GT/s x1)
   0000:17:03.0 - PCI bridge
      Current: 16GT/s (downgraded), x8 (downgraded) (Expected: 32GT/s x16)
...

==================================================
     FINAL SUMMARY
==================================================

  ❌ Driver/Fabric Manager Conflicts: 1 issue(s) found
  ✅ Xid Errors: OK
  ✅ NVLink Errors: OK
  ✅ GPU Temperature Issues: OK
  ✅ ECC Memory Errors: OK
  ❌ PCIe Link Degradation: 11 issue(s) found
  ❌ GPU Power/Performance Issues: 8 issue(s) found
  ✅ Thermal Slowdown: OK
  ✅ Segfaults: OK
  ✅ CPU Throttling: OK
  ✅ Hardware Errors: OK
  ✅ GPU Fallen Off Bus: OK
  ✅ RmInit Failures: OK
  ✅ Power State Refused: OK
  ✅ Bad CPU Errors: OK
```

## Supported Detection Modes

### 1. Xid Errors
Xid errors are hardware/software errors reported by the NVIDIA driver. The tool will:
- Count occurrences of each Xid error type
- Load error descriptions from CSV file
- Provide links to NVIDIA official documentation

### 2. NVLink Health Check
For multi-GPU systems (e.g., DGX, HGX), NVLink is critical:
- Detects link errors (Replay, Recovery, CRC)
- Counts total errors
- Assesses link health status

### 3. Temperature Monitoring
GPU overheating can cause:
- Performance degradation (thermal throttling)
- Reduced hardware lifespan
- System instability

The tool provides:
- Min/Max/Average temperatures
- Temperature threshold warnings (75°C, 85°C)

### 4. ECC Memory Errors
ECC errors indicate memory issues:
- **Correctable errors**: Auto-fixed, but frequent occurrences need attention
- **Uncorrectable errors**: Serious issues that may cause data corruption

### 5. PCIe Link Status
PCIe degradation severely impacts performance:
- Detects speed reduction (e.g., Gen4 to Gen3)
- Detects width reduction (e.g., x16 to x8)
- Shows detailed information for each degraded link

### 6. Power and Performance
- Monitors power consumption
- Checks if approaching power limit
- Verifies Persistence Mode configuration

## Configuration Files

### xid-errors.csv
CSV file containing Xid error codes and descriptions:
```csv
1,GPU has fallen off the bus
2,Fifo: Puller Error
3,Fifo: Pusher Error
...
```

Create this file from NVIDIA official documentation:
https://docs.nvidia.com/deploy/xid-errors/index.html

## Performance Optimizations

- Uses `once_cell::Lazy` to pre-compile regular expressions
- Single file read to reduce I/O
- Efficient pattern matching algorithms
- Release build optimizations

## Use Cases

### Data Centers
- Regular health checks
- Fault diagnosis
- Performance monitoring

### AI/ML Training Clusters
- Multi-GPU system monitoring
- NVLink health checks
- Temperature and power management

### HPC Environments
- Large-scale GPU deployments
- Automated monitoring
- Issue alerts

## Troubleshooting

### Common Issues

**Q: Log file not found**
```bash
# Ensure log file exists
ls -l nvidia-bug-report.log

# Or specify full path
./nvidia_log_parser /full/path/to/log
```

**Q: XID errors have no descriptions**
```bash
# Create or update xid-errors.csv file
# Get latest error codes from NVIDIA documentation
```

**Q: Compilation errors**
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

## Tech Stack

- **Language**: Rust 2021 Edition
- **Dependencies**:
  - `clap` - Command-line argument parsing
  - `regex` - Regular expression matching
  - `once_cell` - Lazy initialization
  - `csv` - CSV file parsing

## Contributing

Contributions welcome! You can:
1. Report bugs
2. Suggest new features
3. Submit Pull Requests
4. Improve documentation

## License

MIT License

## Related Resources

- [NVIDIA Xid Error Documentation](https://docs.nvidia.com/deploy/xid-errors/index.html)
- [NVIDIA GPU Deployment Guide](https://docs.nvidia.com/datacenter/tesla/index.html)
- [nvidia-smi Documentation](https://developer.nvidia.com/nvidia-system-management-interface)

## Changelog

### v0.2.0 (Latest)
- ✨ Added NVLink error check
- ✨ Added GPU temperature monitoring
- ✨ Added ECC memory error check
- ✨ Added PCIe link status check with detailed degradation info
- ✨ Added GPU power and performance check
- ✨ Added comprehensive final summary
- 📝 Improved documentation and examples

### v0.1.0
- 🎉 Initial release
- ✅ Basic system information parsing
- ✅ Xid error detection
- ✅ Common issue checks
