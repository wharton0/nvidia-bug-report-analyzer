# NVIDIA Bug Report 日志分析工具

一个用 Rust 编写的高性能 NVIDIA bug report 日志分析工具，用于快速诊断 GPU 系统问题。

## 功能特性

### 核心检查项
- ✅ 系统信息摘要（CPU、内存、BIOS、主板）
- ✅ GPU 信息摘要（型号、数量、PCI 地址、UUID）
- ✅ NVIDIA 驱动版本冲突检查
- ✅ Xid 错误分析（带详细描述）
- ✅ 热限流检测
- ✅ 段错误检测
- ✅ CPU 限流检测
- ✅ 硬件错误检测
- ✅ GPU 掉线检测（fallen off the bus）
- ✅ RmInit 失败检测
- ✅ 电源状态变更拒绝检测
- ✅ CPU 错误检测（bad cpu）

### 新增高级检查项 🆕
- ⭐ **NVLink 错误检查** - 检测 Replay、Recovery、CRC 错误
- ⭐ **GPU 温度监控** - 温度统计和阈值警告
- ⭐ **ECC 内存错误** - DRAM 可纠正/不可纠正错误检测
- ⭐ **PCIe 链路状态** - 检测链路降速和宽度问题
- ⭐ **GPU 功率和性能** - 功率统计和 Persistence Mode 检查

## 安装

### 前置要求
- Rust 1.70 或更高版本
- Cargo（Rust 包管理器）

### 编译
```bash
# 克隆或下载项目
cd nvidia_log_parser

# 编译 release 版本
cargo build --release

# 可执行文件位于
./target/release/nvidia-bug-report-parser
```

## 使用方法

### 基本用法
```bash
# 使用默认日志文件名（nvidia-bug-report.log）
./target/release/nvidia-bug-report-parser

# 指定日志文件
./target/release/nvidia-bug-report-parser /path/to/nvidia-bug-report.log

# 指定 XID 错误 CSV 文件
./target/release/nvidia-bug-report-parser -x xid-errors.csv nvidia-bug-report.log
```

### 生成 NVIDIA Bug Report
```bash
# 在 Linux 系统上生成 bug report
sudo nvidia-bug-report.sh

# 这会生成一个类似 nvidia-bug-report.log.gz 的文件
# 解压后使用本工具分析
gunzip nvidia-bug-report.log.gz
```

## 输出示例

```
==================================================
     NVIDIA Bug Report Log Summary
==================================================

Log from: Tue Oct 14 10:00:52 AM UTC 2025
NVIDIA Driver Version: 550.144.03
Chassis DMI: Dell Inc. PowerEdge R760xa
...

==================================================
     NVLink Status Check
==================================================

No NVLink errors detected - All links healthy

==================================================
     GPU Temperature Check
==================================================

GPU Temperatures:
   Min: 45°C, Max: 52°C, Avg: 48°C
   Temperatures are within normal range

==================================================
     ECC Memory Error Check
==================================================

No ECC memory errors detected

==================================================
     PCIe Link Status Check
==================================================

** PCIe Link Degradation Detected: 2 instances
   Some PCIe links are running at reduced speed or width.
   This may impact GPU performance.

==================================================
     GPU Power and Performance
==================================================

Average GPU Power Draw: 122.30 W
Average Power Limit: 700.00 W (17.5% utilized)

** RECOMMENDATION: Persistence Mode is Disabled on 8 GPU(s)
   Enable with: sudo nvidia-smi -pm 1
   This improves performance and reduces driver load time.
```

## 支持的检测模式

### 1. Xid 错误
Xid 错误是 NVIDIA 驱动报告的硬件/软件错误。工具会：
- 统计每种 Xid 错误的出现次数
- 从 CSV 文件加载错误描述
- 提供 NVIDIA 官方文档链接

### 2. NVLink 健康检查
对于多 GPU 系统（如 DGX、HGX），NVLink 是关键互联：
- 检测链路错误（Replay、Recovery、CRC）
- 统计总错误数
- 评估链路健康状况

### 3. 温度监控
GPU 过热会导致：
- 性能降低（热限流）
- 硬件寿命缩短
- 系统不稳定

工具提供：
- 最低/最高/平均温度
- 温度阈值警告（75°C、85°C）

### 4. ECC 内存错误
ECC 错误表示内存问题：
- **可纠正错误**：自动修复，但频繁出现需关注
- **不可纠正错误**：严重问题，可能导致数据损坏

### 5. PCIe 链路状态
PCIe 降速会严重影响性能：
- 检测速度降低（如 Gen4 降到 Gen3）
- 检测宽度降低（如 x16 降到 x8）

### 6. 功率和性能
- 监控功率消耗
- 检查是否接近功率上限
- 验证 Persistence Mode 配置

## 配置文件

### xid-errors.csv
包含 Xid 错误代码和描述的 CSV 文件：
```csv
1,GPU has fallen off the bus
2,Fifo: Puller Error
3,Fifo: Pusher Error
...
```

可以从 NVIDIA 官方文档创建此文件：
https://docs.nvidia.com/deploy/xid-errors/index.html

## 性能优化

- 使用 `once_cell::Lazy` 预编译正则表达式
- 单次文件读取，减少 I/O
- 高效的模式匹配算法
- Release 编译优化

## 适用场景

### 数据中心
- 定期健康检查
- 故障诊断
- 性能监控

### AI/ML 训练集群
- 多 GPU 系统监控
- NVLink 健康检查
- 温度和功率管理

### HPC 环境
- 大规模 GPU 部署
- 自动化监控
- 问题预警

## 故障排查

### 常见问题

**Q: 找不到日志文件**
```bash
# 确保日志文件存在
ls -l nvidia-bug-report.log

# 或指定完整路径
./nvidia-bug-report-parser /full/path/to/log
```

**Q: XID 错误没有描述**
```bash
# 创建或更新 xid-errors.csv 文件
# 从 NVIDIA 文档获取最新错误代码
```

**Q: 编译错误**
```bash
# 更新 Rust
rustup update

# 清理并重新编译
cargo clean
cargo build --release
```

## 技术栈

- **语言**: Rust 2021 Edition
- **依赖**:
  - `clap` - 命令行参数解析
  - `regex` - 正则表达式匹配
  - `once_cell` - 延迟初始化
  - `csv` - CSV 文件解析

## 贡献

欢迎贡献！可以：
1. 报告 bug
2. 提出新功能建议
3. 提交 Pull Request
4. 改进文档

## 许可证

MIT License

## 相关资源

- [NVIDIA Xid 错误文档](https://docs.nvidia.com/deploy/xid-errors/index.html)
- [NVIDIA GPU 部署指南](https://docs.nvidia.com/datacenter/tesla/index.html)
- [nvidia-smi 文档](https://developer.nvidia.com/nvidia-system-management-interface)

## 更新日志

### v0.2.0 (最新)
- ✨ 新增 NVLink 错误检查
- ✨ 新增 GPU 温度监控
- ✨ 新增 ECC 内存错误检查
- ✨ 新增 PCIe 链路状态检查
- ✨ 新增 GPU 功率和性能检查
- 📝 改进文档和示例

### v0.1.0
- 🎉 初始版本
- ✅ 基本系统信息解析
- ✅ Xid 错误检测
- ✅ 常见问题检查
