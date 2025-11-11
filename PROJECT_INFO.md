# 项目信息

## 📦 项目名称
**NVIDIA Bug Report Log Analyzer**

## 🎯 项目简介
一个用 Rust 编写的高性能 NVIDIA bug report 日志分析工具，用于快速诊断 GPU 系统问题。

## 🏷️ 版本信息
- **当前版本**: v0.2.0
- **发布日期**: 2025-11-11
- **Rust 版本要求**: 1.70+

## 📁 项目结构
```
nvidia_log_parser/
├── src/
│   └── main.rs              # 主程序源代码
├── Cargo.toml               # Rust 项目配置
├── .gitignore              # Git 忽略文件
├── LICENSE                 # MIT 许可证
├── README.md               # 英文文档
├── README_CN.md            # 中文文档
├── IMPROVEMENTS.md         # 改进说明
├── GITHUB_SETUP.md         # GitHub 发布指南
├── QUICK_START.md          # 快速开始指南
├── PROJECT_INFO.md         # 项目信息（本文件）
├── xid-errors.csv          # XID 错误描述（空文件）
└── xid-errors.csv.example  # XID 错误描述示例
```

## 🚀 核心功能

### 基础检查
1. 系统信息解析（CPU、内存、BIOS、主板）
2. GPU 信息摘要（型号、数量、PCI 地址、UUID）
3. NVIDIA 驱动版本冲突检测
4. Xid 错误分析（带详细描述）
5. 热限流检测
6. 段错误检测
7. CPU 限流检测
8. 硬件错误检测
9. GPU 掉线检测
10. RmInit 失败检测
11. 电源状态拒绝检测
12. CPU 错误检测

### 高级检查
1. **NVLink 错误检查** - 检测 Replay、Recovery、CRC 错误
2. **GPU 温度监控** - 温度统计和阈值警告
3. **ECC 内存错误** - DRAM 可纠正/不可纠正错误检测
4. **PCIe 链路状态** - 检测链路降速和宽度问题，显示详细信息
5. **GPU 功率和性能** - 功率统计和 Persistence Mode 检查

### 最终摘要
- 所有检查项的综合摘要
- ✅/❌ 清晰的视觉指示器
- 每个类别的问题计数

## 🛠️ 技术栈

### 语言和工具
- **Rust**: 2021 Edition
- **Cargo**: Rust 包管理器

### 依赖库
```toml
[dependencies]
clap = { version = "4.5", features = ["derive"] }
regex = "1.11"
once_cell = "1.20"
csv = "1.3"
```

### 性能优化
- 使用 `once_cell::Lazy` 预编译正则表达式
- 单次文件读取，减少 I/O
- 高效的模式匹配算法
- Release 编译优化

## 📊 支持的日志格式

### nvidia-smi 输出格式
```
GPU 00000000:19:00.0
    Product Name                          : NVIDIA H200
    GPU UUID                              : GPU-39d52d8d-06f2-1986-31e4-9e01ffdafda3
```

### 旧格式
```
GPU 0:
    Model:                                NVIDIA H200
    GPU UUID:                             GPU-39d52d8d-06f2-1986-31e4-9e01ffdafda3
```

## 🎯 适用场景

### 数据中心
- 定期健康检查
- 故障诊断
- 性能监控
- 自动化报告

### AI/ML 训练集群
- 多 GPU 系统监控
- NVLink 健康检查
- 温度和功率管理
- 训练前系统验证

### HPC 环境
- 大规模 GPU 部署
- 节点健康监控
- 问题预警
- 批量分析

## 📈 测试覆盖

### 测试系统
- **服务器**: Dell PowerEdge XE9680
- **GPU**: 8x NVIDIA H200 (141GB HBM3)
- **CPU**: 2x Intel Xeon Platinum 8558 (192 线程)
- **内存**: 32x 64GB DDR5-5600
- **驱动**: NVIDIA 550.144.03
- **操作系统**: Ubuntu 22.04.5 LTS

### 测试结果
✅ 成功检测：
- 8 个 GPU 的 PCI 地址和 UUID
- 驱动版本冲突（1 个）
- PCIe 链路降速（11 个）
- Persistence Mode 未启用（8 个 GPU）
- 温度正常（33-38°C）
- 无 NVLink 错误
- 无 ECC 错误

## 🔄 版本历史

### v0.2.0 (2025-11-11)
- ✨ 新增 NVLink 错误检查
- ✨ 新增 GPU 温度监控
- ✨ 新增 ECC 内存错误检查
- ✨ 新增 PCIe 链路状态检查（带详细降速信息）
- ✨ 新增 GPU 功率和性能检查
- ✨ 新增综合最终摘要
- 🐛 修复 GPU 信息解析（支持 nvidia-smi 格式）
- 📝 改进文档和示例

### v0.1.0 (初始版本)
- 🎉 初始发布
- ✅ 基本系统信息解析
- ✅ Xid 错误检测
- ✅ 常见问题检查

## 🤝 贡献指南

### 如何贡献
1. Fork 项目
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

### 代码规范
- 遵循 Rust 官方代码风格
- 使用 `cargo fmt` 格式化代码
- 使用 `cargo clippy` 检查代码质量
- 添加必要的注释和文档

### 提交信息规范
- `feat:` 新功能
- `fix:` 修复 bug
- `docs:` 文档更新
- `style:` 代码格式调整
- `refactor:` 代码重构
- `test:` 测试相关
- `chore:` 构建/工具相关

## 📞 联系方式

### 问题反馈
- GitHub Issues: 提交 bug 报告或功能请求
- Pull Requests: 贡献代码

### 相关资源
- [NVIDIA 官方文档](https://docs.nvidia.com/)
- [Rust 官方网站](https://www.rust-lang.org/)
- [Cargo 文档](https://doc.rust-lang.org/cargo/)

## 📄 许可证
MIT License - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢
- NVIDIA 提供的 GPU 技术和文档
- Rust 社区提供的优秀工具和库
- 所有贡献者和用户的支持

## 🔮 未来计划

### 短期计划 (v0.3.0)
- [ ] 添加 JSON 输出格式
- [ ] 支持多日志文件对比
- [ ] 添加性能基准测试
- [ ] 改进错误消息

### 中期计划 (v0.4.0)
- [ ] Web UI 界面
- [ ] 实时监控模式
- [ ] 邮件/Slack 通知
- [ ] 数据库存储历史记录

### 长期计划 (v1.0.0)
- [ ] 机器学习预测故障
- [ ] 集群级别分析
- [ ] 自动修复建议
- [ ] 多语言支持

## 📊 项目统计
- **代码行数**: ~600 行 Rust 代码
- **依赖数量**: 4 个核心依赖
- **编译时间**: ~4 秒（release 模式）
- **可执行文件大小**: ~2.4 MB
- **支持平台**: Windows, Linux, macOS
