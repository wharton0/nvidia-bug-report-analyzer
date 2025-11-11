use clap::Parser;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::HashMap;
use std::error::Error;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = "Parses an nvidia-bug-report.log file to provide a summary of system info and errors.")]
struct Args {
    #[arg(default_value = "nvidia-bug-report.log")]
    log_file: PathBuf,

    #[arg(long, short = 'x', default_value = "xid-errors.csv")]
    xid_errors_csv: PathBuf,
}

static RE_DATE: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^Date:\s*(.*)$").unwrap());
static RE_NVIDIA_VERSION: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^Driver Version:\s*(.*)$").unwrap());
static RE_CHASSIS_DMI: Lazy<Regex> = Lazy::new(|| Regex::new(r"DMI:\s*(.*),").unwrap());
static RE_SYS_INFO: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?ms)System Information\s*\n(.*?)\n\n").unwrap());
static RE_BASEBOARD_INFO: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?ms)Base Board Information\s*\n(.*?)\n\n").unwrap());
static RE_BIOS_INFO: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?ms)BIOS Information\s*\n(.*?)\n\n").unwrap());
static RE_CMDLINE: Lazy<Regex> = Lazy::new(|| Regex::new(r"cmdline\s*\n\s*(.*)").unwrap());
static RE_CPU_MODEL: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^model name\s*:\s*(.*)$").unwrap());
static RE_DIMM: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?ms)Handle.*?DMI type 17, (.*?)\n\n").unwrap());
static RE_GPU_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?ms)GPU \d+:(.*?)\n\n").unwrap());
static RE_GPU_SMI_BLOCK: Lazy<Regex> = Lazy::new(|| Regex::new(r"(?m)^GPU ([0-9A-Fa-f:\.]+)\s*$").unwrap());
static RE_GPU_UUID: Lazy<Regex> = Lazy::new(|| Regex::new(r"GPU UUID\s*:\s*([^\s]+)").unwrap());
static RE_BUS_LOCATION: Lazy<Regex> = Lazy::new(|| Regex::new(r"Bus Location:\s*([^\s]+)").unwrap());
static RE_GPU_MODEL: Lazy<Regex> = Lazy::new(|| Regex::new(r"Model:\s*([^\n]+)").unwrap());
static RE_SUBSYSTEM: Lazy<Regex> = Lazy::new(|| Regex::new(r"Subsystem:\s*([^\n]+)").unwrap());
static RE_XID: Lazy<Regex> = Lazy::new(|| Regex::new(r"kernel: NVRM: Xid \(PCI: [^)]+\): (\d+), ([^\n]+)").unwrap());

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    if !args.log_file.exists() {
        eprintln!("Error: Log file not found at '{}'", args.log_file.display());
        return Ok(());
    }
    if !args.xid_errors_csv.exists() {
        eprintln!("Warning: XID errors CSV not found at '{}'. Descriptions will be unavailable.", args.xid_errors_csv.display());
    }

    let log_content = fs::read_to_string(&args.log_file)?.replace('\r', "");

    println!("\n==================================================");
    println!("     NVIDIA Bug Report Log Summary");
    println!("==================================================\n");

    print_system_summary(&log_content);
    print_gpu_summary(&log_content);
    
    // Detailed checks
    let version_conflicts = check_version_conflicts(&log_content);
    let xid_errors = check_xid_errors(&log_content, &args.xid_errors_csv);
    let nvlink_errors = check_nvlink_errors(&log_content);
    let temp_status = check_gpu_temperatures(&log_content);
    let ecc_errors = check_ecc_errors(&log_content);
    let pcie_issues = check_pcie_link_status(&log_content);
    let power_status = check_gpu_power_performance(&log_content);
    
    // Simple status checks
    let thermal_slowdown = check_thermal_slowdown(&log_content);
    let segfaults = check_segfaults(&log_content);
    let cpu_throttle = check_cpu_throttle(&log_content);
    let hardware_errors = check_hardware_errors(&log_content);
    let fallen_off_bus = check_fallen_off_bus(&log_content);
    let rminit_failed = check_rminit_failed(&log_content);
    let power_state_refused = check_power_state_refused(&log_content);
    let bad_cpu = check_bad_cpu(&log_content);

    // Final Summary
    println!("\n==================================================");
    println!("     FINAL SUMMARY");
    println!("==================================================\n");
    
    print_summary_item("Driver/Fabric Manager Conflicts", version_conflicts);
    print_summary_item("Xid Errors", xid_errors);
    print_summary_item("NVLink Errors", nvlink_errors);
    print_summary_item("GPU Temperature Issues", temp_status);
    print_summary_item("ECC Memory Errors", ecc_errors);
    print_summary_item("PCIe Link Degradation", pcie_issues);
    print_summary_item("GPU Power/Performance Issues", power_status);
    print_summary_item("Thermal Slowdown", thermal_slowdown);
    print_summary_item("Segfaults", segfaults);
    print_summary_item("CPU Throttling", cpu_throttle);
    print_summary_item("Hardware Errors", hardware_errors);
    print_summary_item("GPU Fallen Off Bus", fallen_off_bus);
    print_summary_item("RmInit Failures", rminit_failed);
    print_summary_item("Power State Refused", power_state_refused);
    print_summary_item("Bad CPU Errors", bad_cpu);

    println!("\n==================================================");
    println!("     End of Report");
    println!("==================================================\n");

    Ok(())
}

fn print_system_summary(log: &str) {
    if let Some(cap) = RE_DATE.captures(log) {
        println!("Log from: {}", &cap[1]);
    }

    if let Some(cap) = RE_NVIDIA_VERSION.captures(log) {
        println!("NVIDIA Driver Version: {}", &cap[1]);
    }

    if let Some(cap) = RE_CHASSIS_DMI.captures(log) {
        println!("Chassis DMI: {}", &cap[1]);
    }
    
    if let Some(sys_block) = RE_SYS_INFO.captures(log) {
        let manu = extract_field(&sys_block[1], "Manufacturer:");
        let prod = extract_field(&sys_block[1], "Product Name:");
        let serial = extract_field(&sys_block[1], "Serial Number:");
        println!("Chassis: {} {} {}", manu, prod, serial);
    }
    
    if let Some(bb_block) = RE_BASEBOARD_INFO.captures(log) {
         let prod = extract_field(&bb_block[1], "Product Name:");
         println!("BaseBoard info: {}", prod);
    }

    if let Some(bios_block) = RE_BIOS_INFO.captures(log) {
         let ver = extract_field(&bios_block[1], "Version:");
         let date = extract_field(&bios_block[1], "Release Date:");
         println!("  BIOS: {} {}", ver, date);
    }
    
    let cpu_counts = count_occurrences(RE_CPU_MODEL.captures_iter(log).map(|cap| cap[1].to_string()));
    for (model, count) in cpu_counts {
        println!("  CPUs # threads: {} of CPU: {}", count, model);
    }

    if let Some(cap) = RE_CMDLINE.captures(log) {
        println!("  BOOT Line: {}", &cap[1]);
    }

    println!("  Memory:");
    println!("      Count     DIMM Information");
    
    let dimm_counts = count_occurrences(
        RE_DIMM.captures_iter(log)
            .filter_map(|block| {
                let dimm_info_lines: Vec<String> = block[1].lines()
                    .filter(|l| l.contains("Size:") || l.contains("Speed:") || l.contains("Manufacturer:") || l.contains("Part Number:"))
                    .filter(|l| !["Configured", "Logical", "Non-Volatile", "Cache", "None", "Unknown", "NO DIMM", "No Module"]
                        .iter().any(|&exclude| l.contains(exclude)))
                    .map(|l| l.trim().to_string())
                    .collect();
                if dimm_info_lines.is_empty() {
                    None
                } else {
                    Some(dimm_info_lines.join(" | "))
                }
            })
    );
    
    for (info, count) in dimm_counts {
        println!("    {:>7}     {}", count, info);
    }
    println!();
}

fn print_gpu_summary(log: &str) {
    println!("Summary of PCI Addresses and GPUs");
    
    // Try to parse nvidia-smi format first (GPU 00000000:19:00.0)
    let smi_gpus: Vec<_> = RE_GPU_SMI_BLOCK.captures_iter(log).collect();
    
    if !smi_gpus.is_empty() {
        // Parse nvidia-smi output format
        let lines: Vec<&str> = log.lines().collect();
        for cap in smi_gpus {
            let bus_addr = &cap[1];
            let gpu_line_idx = lines.iter().position(|&l| l.contains(&format!("GPU {}", bus_addr)));
            
            if let Some(idx) = gpu_line_idx {
                // Look for GPU UUID in the next ~20 lines
                let mut uuid = "N/A";
                for i in idx..std::cmp::min(idx + 30, lines.len()) {
                    if let Some(uuid_cap) = RE_GPU_UUID.captures(lines[i]) {
                        uuid = uuid_cap.get(1).unwrap().as_str();
                        break;
                    }
                }
                println!("{} {}", bus_addr, uuid);
            }
        }
    } else {
        // Fallback to old format (GPU 0:, GPU 1:, etc.)
        for block in RE_GPU_BLOCK.captures_iter(log) {
            let uuid = RE_GPU_UUID.captures(&block[1]).map_or("N/A", |c| c.get(1).unwrap().as_str());
            let bus = RE_BUS_LOCATION.captures(&block[1]).map_or("N/A", |c| c.get(1).unwrap().as_str());
            println!("{} {}", bus, uuid);
        }
    }
    println!();

    println!("GPUs:");
    let model_counts = count_occurrences(RE_GPU_MODEL.captures_iter(log).map(|cap| cap[1].to_string()));
    for (model, count) in model_counts {
        println!("    {} {}", count, model);
    }
    
    let subsystem_counts = count_occurrences(RE_SUBSYSTEM.captures_iter(log).map(|cap| cap[1].to_string()));
    for (subsystem, count) in subsystem_counts {
        println!("    {}     Subsystem: {}", count, subsystem);
    }
    println!();
}

fn check_pattern(log: &str, pattern: &str, header: &str, not_found_msg: &str, advice: Option<&str>) -> usize {
    let re = Regex::new(pattern).unwrap();
    let matches: Vec<_> = re.find_iter(log).map(|m| m.as_str().to_string()).collect();
    
    if matches.is_empty() {
        println!("{}", not_found_msg);
        0
    } else {
        println!("\n** {}: {}", header, matches.len());
        if let Some(adv) = advice {
            println!("{}", adv);
        }
        
        let counts = count_occurrences(matches.iter().map(|s| s.trim()));
        for (line, count) in counts {
             println!("    ({}x) {}", count, line);
        }
        matches.len()
    }
}

fn check_version_conflicts(log: &str) -> usize {
    let pattern = r"Please update with matching NVIDIA driver";
    let advice = "     ** This is important for any SXM chassis **\n     Check with:\n        'nvidia-smi topo -m'\n         python -c \"import torch ; print('Is available: ', torch.cuda.is_available())\"";
    check_pattern(log, pattern, "GPU Driver and Fabric Manager Conflicts", "No version conflicts found", Some(advice))
}

fn check_xid_errors(log: &str, csv_path: &Path) -> usize {
    let matches: Vec<_> = RE_XID.captures_iter(log).collect();

    if matches.is_empty() {
        println!("No Xid errors found");
        return 0;
    }

    println!("\nSummary of Xid errors: {}", matches.len());
    println!(" Definitions: https://docs.nvidia.com/deploy/xid-errors/index.html");
    
    let counts = count_occurrences(matches.iter().map(|cap| cap.get(0).unwrap().as_str().trim()));
    for (line, count) in counts {
        println!("    ({}x) {}", count, line);
    }
    
    println!("\nSummary of error descriptions:");
    let xid_map = load_xid_descriptions(csv_path);

    let unique_xids: std::collections::HashSet<_> = matches.iter()
        .map(|cap| cap[1].to_string())
        .collect();
    
    for xid in unique_xids {
        if let Some(desc) = xid_map.get(&xid) {
            println!("   {} - {}", xid, desc);
        } else {
            println!("   {} - No description found in CSV.", xid);
        }
    }
    
    matches.len()
}

fn check_thermal_slowdown(log: &str) -> usize {
    check_pattern(log, r"(?m).*(?:SW|HW) Thermal Slowdown.*: Active$", "Thermal Slow down", "No thermal slowdown messages found", None)
}

fn check_segfaults(log: &str) -> usize {
    check_pattern(log, r"(?m).*segfault.*$", "Segfaults", "No segfaults found", None)
}

fn check_cpu_throttle(log: &str) -> usize {
    check_pattern(log, r"(?m).*cpu clock throttled.*$", "CPU throttling", "No CPU throttling", None)
}

fn check_hardware_errors(log: &str) -> usize {
    let advice = "  To find specific errors:\n      `grep -E 'Hardware Error' <logfile>` on Linux, or search in a text editor on Windows.";
    check_pattern(log, r"Hardware Error", "Hardware Errors", "No Hardware Errors found", Some(advice))
}

fn check_fallen_off_bus(log: &str) -> usize {
    check_pattern(log, r"(?m)kernel: NVRM:.*GPU has fallen off the bus.*$", "Fallen off the bus Errors", "No 'fallen off the bus' errors", None)
}

fn check_rminit_failed(log: &str) -> usize {
    check_pattern(log, r"(?m).*(?:RmInitAdapter failed|rm_init_adapter failed).*", "GPU RmInitAdapter Failed", "No 'RmInit failures'", None)
}

fn check_power_state_refused(log: &str) -> usize {
     check_pattern(log, r"kernel: nvidia-gpu.*Refused to change power state,", "Power State Change Refused", "No 'Refused to change power state' messages", None)
}

fn check_bad_cpu(log: &str) -> usize {
    let advice = "Commonly due to only 255 of 256+ threads seen.\nQuick fix: disable SMT in BIOS.\nReal fix depends on Motherboard/BIOS (e.g., enabling X2APIC and IOMMU).";
    check_pattern(log, r"bad cpu", "Bad CPU Error", "No 'bad cpu' Errors found", Some(advice))
}

fn check_nvlink_errors(log: &str) -> usize {
    println!("\n==================================================");
    println!("     NVLink Status Check");
    println!("==================================================\n");
    
    let replay_re = Regex::new(r"Link \d+: Replay Errors: (\d+)").unwrap();
    let recovery_re = Regex::new(r"Link \d+: Recovery Errors: (\d+)").unwrap();
    let crc_re = Regex::new(r"Link \d+: CRC Errors: (\d+)").unwrap();
    
    let mut total_replay = 0;
    let mut total_recovery = 0;
    let mut total_crc = 0;
    
    for cap in replay_re.captures_iter(log) {
        if let Ok(count) = cap[1].parse::<i32>() {
            total_replay += count;
        }
    }
    for cap in recovery_re.captures_iter(log) {
        if let Ok(count) = cap[1].parse::<i32>() {
            total_recovery += count;
        }
    }
    for cap in crc_re.captures_iter(log) {
        if let Ok(count) = cap[1].parse::<i32>() {
            total_crc += count;
        }
    }
    
    let total_errors = (total_replay + total_recovery + total_crc) as usize;
    
    if total_errors > 0 {
        println!("** NVLink Errors Detected:");
        println!("   Total Replay Errors: {}", total_replay);
        println!("   Total Recovery Errors: {}", total_recovery);
        println!("   Total CRC Errors: {}", total_crc);
        println!("   ** These errors may indicate NVLink connectivity issues.");
    } else {
        println!("No NVLink errors detected - All links healthy");
    }
    
    total_errors
}

fn check_gpu_temperatures(log: &str) -> usize {
    println!("\n==================================================");
    println!("     GPU Temperature Check");
    println!("==================================================\n");
    
    let temp_re = Regex::new(r"GPU Current Temp\s*:\s*(\d+)\s*C").unwrap();
    let mut temps = Vec::new();
    
    for cap in temp_re.captures_iter(log) {
        if let Ok(temp) = cap[1].parse::<i32>() {
            temps.push(temp);
        }
    }
    
    if temps.is_empty() {
        println!("No temperature data found");
        return 0;
    }
    
    let max_temp = temps.iter().max().unwrap();
    let min_temp = temps.iter().min().unwrap();
    let avg_temp = temps.iter().sum::<i32>() / temps.len() as i32;
    
    println!("GPU Temperatures:");
    println!("   Min: {}°C, Max: {}°C, Avg: {}°C", min_temp, max_temp, avg_temp);
    
    let issues = if *max_temp > 85 {
        println!("   ** WARNING: High temperature detected (>85°C)!");
        temps.iter().filter(|&&t| t > 85).count()
    } else if *max_temp > 75 {
        println!("   ** CAUTION: Elevated temperature (>75°C)");
        temps.iter().filter(|&&t| t > 75).count()
    } else {
        println!("   Temperatures are within normal range");
        0
    };
    
    issues
}

fn check_ecc_errors(log: &str) -> usize {
    println!("\n==================================================");
    println!("     ECC Memory Error Check");
    println!("==================================================\n");
    
    let dram_corr_re = Regex::new(r"DRAM Correctable\s*:\s*(\d+)").unwrap();
    let dram_uncorr_re = Regex::new(r"DRAM Uncorrectable\s*:\s*(\d+)").unwrap();
    
    let mut total_correctable = 0;
    let mut total_uncorrectable = 0;
    
    for cap in dram_corr_re.captures_iter(log) {
        if let Ok(count) = cap[1].parse::<i32>() {
            total_correctable += count;
        }
    }
    
    for cap in dram_uncorr_re.captures_iter(log) {
        if let Ok(count) = cap[1].parse::<i32>() {
            total_uncorrectable += count;
        }
    }
    
    let total_errors = (total_correctable + total_uncorrectable) as usize;
    
    if total_errors > 0 {
        println!("** ECC Errors Detected:");
        println!("   DRAM Correctable Errors: {}", total_correctable);
        println!("   DRAM Uncorrectable Errors: {}", total_uncorrectable);
        if total_uncorrectable > 0 {
            println!("   ** CRITICAL: Uncorrectable errors indicate potential hardware failure!");
        }
    } else {
        println!("No ECC memory errors detected");
    }
    
    if log.contains("Retired Pages") {
        println!("\nRetired Pages information available in log");
    }
    
    total_errors
}

fn check_pcie_link_status(log: &str) -> usize {
    println!("\n==================================================");
    println!("     PCIe Link Status Check");
    println!("==================================================\n");
    
    // Find all PCI devices with downgraded links
    let pci_device_re = Regex::new(r"(?m)^([0-9a-f]{4}:[0-9a-f]{2}:[0-9a-f]{2}\.[0-9])\s+(.+?)\[([0-9a-f]{4})\]").unwrap();
    let lnksta_re = Regex::new(r"LnkSta:\s*Speed\s+([^\s]+)\s*(\([^)]*\))?,\s*Width\s+([^\s]+)\s*(\([^)]*\))?").unwrap();
    let lnkcap_re = Regex::new(r"LnkCap:.*?Speed\s+([^\s,]+).*?Width\s+([^\s,]+)").unwrap();
    
    let lines: Vec<&str> = log.lines().collect();
    let mut degraded_links = Vec::new();
    
    for (i, line) in lines.iter().enumerate() {
        if line.contains("LnkSta:") && line.contains("downgraded") {
            // Find the PCI device address by looking backwards
            let mut pci_addr = "Unknown";
            let mut device_desc = "Unknown Device";
            let mut lnkcap_info = String::new();
            
            for j in (0..i).rev().take(50) {
                if let Some(cap) = pci_device_re.captures(lines[j]) {
                    pci_addr = cap.get(1).unwrap().as_str();
                    device_desc = cap.get(2).unwrap().as_str().trim();
                    break;
                }
            }
            
            // Find LnkCap (expected capability) by looking backwards
            for j in (0..i).rev().take(20) {
                if let Some(cap) = lnkcap_re.captures(lines[j]) {
                    let expected_speed = cap.get(1).unwrap().as_str();
                    let expected_width = cap.get(2).unwrap().as_str();
                    lnkcap_info = format!(" (Expected: {} {})", expected_speed, expected_width);
                    break;
                }
            }
            
            // Parse current LnkSta
            if let Some(cap) = lnksta_re.captures(line) {
                let current_speed = cap.get(1).unwrap().as_str();
                let speed_status = cap.get(2).map_or("", |m| m.as_str());
                let current_width = cap.get(3).unwrap().as_str();
                let width_status = cap.get(4).map_or("", |m| m.as_str());
                
                degraded_links.push(format!(
                    "   {} - {}\n      Current: {} {}, {} {}{}",
                    pci_addr, device_desc, current_speed, speed_status, 
                    current_width, width_status, lnkcap_info
                ));
            }
        }
    }
    
    if !degraded_links.is_empty() {
        println!("** PCIe Link Degradation Detected: {} instances", degraded_links.len());
        println!("   Some PCIe links are running at reduced speed or width.");
        println!("   This may impact GPU performance.\n");
        println!("Degraded Links:");
        for link in &degraded_links {
            println!("{}", link);
        }
    } else {
        println!("All PCIe links appear to be running at expected speeds");
    }
    
    degraded_links.len()
}

fn check_gpu_power_performance(log: &str) -> usize {
    println!("\n==================================================");
    println!("     GPU Power and Performance");
    println!("==================================================\n");
    
    let power_draw_re = Regex::new(r"GPU Power Readings\s*\n\s*Power Draw\s*:\s*([\d.]+)\s*W").unwrap();
    let power_limit_re = Regex::new(r"Current Power Limit\s*:\s*([\d.]+)\s*W").unwrap();
    let persistence_re = Regex::new(r"Persistence Mode\s*:\s*(\w+)").unwrap();
    
    let mut power_draws = Vec::new();
    let mut power_limits = Vec::new();
    
    for cap in power_draw_re.captures_iter(log) {
        if let Ok(power) = cap[1].parse::<f32>() {
            power_draws.push(power);
        }
    }
    
    for cap in power_limit_re.captures_iter(log) {
        if let Ok(limit) = cap[1].parse::<f32>() {
            power_limits.push(limit);
        }
    }
    
    if !power_draws.is_empty() {
        let avg_power = power_draws.iter().sum::<f32>() / power_draws.len() as f32;
        println!("Average GPU Power Draw: {:.2} W", avg_power);
        
        if !power_limits.is_empty() {
            let avg_limit = power_limits.iter().sum::<f32>() / power_limits.len() as f32;
            let usage_pct = (avg_power / avg_limit) * 100.0;
            println!("Average Power Limit: {:.2} W ({:.1}% utilized)", avg_limit, usage_pct);
        }
    }
    
    let persistence_disabled = persistence_re.captures_iter(log)
        .filter(|cap| &cap[1] == "Disabled")
        .count();
    
    if persistence_disabled > 0 {
        println!("\n** RECOMMENDATION: Persistence Mode is Disabled on {} GPU(s)", persistence_disabled);
        println!("   Enable with: sudo nvidia-smi -pm 1");
        println!("   This improves performance and reduces driver load time.");
    }
    
    persistence_disabled
}

fn extract_field<'a>(block: &'a str, field: &str) -> &'a str {
    block.lines()
        .find(|l| l.contains(field))
        .and_then(|l| l.split(':').nth(1))
        .map(|s| s.trim())
        .unwrap_or("")
}

fn print_summary_item(name: &str, count: usize) {
    if count > 0 {
        println!("  ❌ {}: {} issue(s) found", name, count);
    } else {
        println!("  ✅ {}: OK", name);
    }
}

fn count_occurrences<I, T>(iter: I) -> HashMap<String, usize>
where
    I: Iterator<Item = T>,
    T: AsRef<str>,
{
    let mut counts = HashMap::new();
    for item in iter {
        *counts.entry(item.as_ref().to_string()).or_insert(0) += 1;
    }
    counts
}

fn load_xid_descriptions(csv_path: &Path) -> HashMap<String, String> {
    let mut xid_map = HashMap::new();
    if let Ok(mut reader) = csv::ReaderBuilder::new().has_headers(false).from_path(csv_path) {
        for result in reader.records() {
            if let Ok(record) = result {
                if record.len() >= 2 {
                    xid_map.insert(record.get(0).unwrap().to_string(), record.get(1).unwrap().to_string());
                }
            }
        }
    }
    xid_map
}
