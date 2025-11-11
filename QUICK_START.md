# 快速开始指南

## 🚀 5 分钟上手

### 1. 克隆或下载项目
```bash
git clone https://github.com/YOUR_USERNAME/nvidia-bug-report-analyzer.git
cd nvidia-bug-report-analyzer
```

### 2. 编译项目
```bash
cargo build --release
```

### 3. 生成 NVIDIA Bug Report
```bash
# 在有 NVIDIA GPU 的 Linux 系统上
sudo nvidia-bug-report.sh

# 会生成类似 nvidia-bug-report.log.gz 的文件
gunzip nvidia-bug-report.log.gz
```

### 4. 运行分析
```bash
# Linux/Mac
./target/release/nvidia_log_parser nvidia-bug-report.log

# Windows
.\target\release\nvidia_log_parser.exe nvidia-bug-report.log
```

## 📊 输出示例

程序会输出详细的分析报告，包括：

1. **系统信息** - CPU、内存、BIOS
2. **GPU 信息** - 型号、PCI 地址、UUID
3. **错误检查** - Xid 错误、驱动冲突
4. **NVLink 状态** - 链路健康检查
5. **温度监控** - GPU 温度统计
6. **ECC 错误** - 内存错误检测
7. **PCIe 状态** - 链路降速详情
8. **功率性能** - 功率使用和配置
9. **最终摘要** - 所有检查的汇总

## 🔧 常见问题

### Q: 找不到 xid-errors.csv
**A:** 复制示例文件：
```bash
cp xid-errors.csv.example xid-errors.csv
```

### Q: 编译失败
**A:** 确保 Rust 版本 >= 1.70：
```bash
rustc --version
rustup update
```

### Q: 权限错误
**A:** 确保日志文件可读：
```bash
chmod +r nvidia-bug-report.log
```

## 📖 详细文档

- [完整 README](README.md)
- [中文文档](README_CN.md)
- [改进说明](IMPROVEMENTS.md)
- [GitHub 发布指南](GITHUB_SETUP.md)

## 💡 使用技巧

### 1. 批量分析
```bash
for log in *.log; do
    echo "Analyzing $log..."
    ./target/release/nvidia_log_parser "$log" > "${log%.log}_report.txt"
done
```

### 2. 只看摘要
```bash
./target/release/nvidia_log_parser nvidia-bug-report.log | grep -A 20 "FINAL SUMMARY"
```

### 3. 导出报告
```bash
./target/release/nvidia_log_parser nvidia-bug-report.log > report.txt
```

### 4. 检查特定问题
```bash
# 只看 PCIe 问题
./target/release/nvidia_log_parser nvidia-bug-report.log | grep -A 30 "PCIe Link"

# 只看温度
./target/release/nvidia_log_parser nvidia-bug-report.log | grep -A 10 "Temperature"
```

## 🎯 实际应用场景

### 数据中心运维
```bash
# 定期健康检查
0 2 * * * /usr/local/bin/nvidia-bug-report.sh && \
          /usr/local/bin/nvidia_log_parser nvidia-bug-report.log | \
          mail -s "GPU Health Report" admin@example.com
```

### AI 训练集群
```bash
# 训练前检查
./target/release/nvidia_log_parser nvidia-bug-report.log
if [ $? -eq 0 ]; then
    echo "GPU health check passed"
    python train.py
else
    echo "GPU issues detected, check report"
fi
```

### HPC 环境
```bash
# 节点健康监控
for node in node{1..100}; do
    ssh $node "sudo nvidia-bug-report.sh"
    scp $node:nvidia-bug-report.log ${node}.log
    ./target/release/nvidia_log_parser ${node}.log > ${node}_report.txt
done
```

## 🔗 相关资源

- [NVIDIA Xid 错误文档](https://docs.nvidia.com/deploy/xid-errors/index.html)
- [nvidia-smi 手册](https://developer.nvidia.com/nvidia-system-management-interface)
- [NVIDIA GPU 部署指南](https://docs.nvidia.com/datacenter/tesla/index.html)

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

## 📄 许可证

MIT License - 详见 [LICENSE](LICENSE) 文件
