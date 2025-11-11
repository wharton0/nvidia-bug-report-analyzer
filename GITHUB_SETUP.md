# GitHub 发布指南

## 方法 1: 使用 GitHub CLI (推荐)

### 1. 安装 GitHub CLI
```bash
# Windows (使用 winget)
winget install --id GitHub.cli

# 或下载安装包
# https://cli.github.com/
```

### 2. 登录 GitHub
```bash
gh auth login
```

### 3. 创建仓库并推送
```bash
# 创建公开仓库
gh repo create nvidia-bug-report-analyzer --public --source=. --remote=origin --push

# 或创建私有仓库
gh repo create nvidia-bug-report-analyzer --private --source=. --remote=origin --push
```

## 方法 2: 使用 GitHub 网页界面

### 1. 在 GitHub 上创建新仓库
1. 访问 https://github.com/new
2. 仓库名称: `nvidia-bug-report-analyzer`
3. 描述: `A high-performance NVIDIA bug report log analysis tool written in Rust`
4. 选择 Public 或 Private
5. **不要**勾选 "Initialize this repository with a README"
6. 点击 "Create repository"

### 2. 推送本地代码
```bash
# 添加远程仓库（替换 YOUR_USERNAME）
git remote add origin https://github.com/YOUR_USERNAME/nvidia-bug-report-analyzer.git

# 推送代码
git branch -M main
git push -u origin main
```

## 方法 3: 使用 SSH

### 1. 设置 SSH 密钥（如果还没有）
```bash
# 生成 SSH 密钥
ssh-keygen -t ed25519 -C "your_email@example.com"

# 复制公钥
cat ~/.ssh/id_ed25519.pub

# 在 GitHub 添加 SSH 密钥
# Settings -> SSH and GPG keys -> New SSH key
```

### 2. 创建仓库并推送
```bash
# 在 GitHub 网页创建仓库后
git remote add origin git@github.com:YOUR_USERNAME/nvidia-bug-report-analyzer.git
git branch -M main
git push -u origin main
```

## 发布 Release

### 使用 GitHub CLI
```bash
# 创建 tag
git tag -a v0.2.0 -m "Release v0.2.0"
git push origin v0.2.0

# 创建 release
gh release create v0.2.0 \
  --title "v0.2.0 - Enhanced Analysis Features" \
  --notes "See CHANGELOG in README.md for details" \
  ./target/release/nvidia_log_parser.exe#nvidia_log_parser-v0.2.0-windows-x64.exe
```

### 使用 GitHub 网页
1. 进入仓库页面
2. 点击 "Releases" -> "Create a new release"
3. Tag version: `v0.2.0`
4. Release title: `v0.2.0 - Enhanced Analysis Features`
5. 描述中粘贴 CHANGELOG
6. 上传编译好的可执行文件
7. 点击 "Publish release"

## 推荐的仓库设置

### Topics (标签)
在仓库页面添加以下 topics:
- `rust`
- `nvidia`
- `gpu`
- `log-analyzer`
- `diagnostics`
- `monitoring`
- `hpc`
- `machine-learning`
- `datacenter`

### About 部分
- Description: `A high-performance NVIDIA bug report log analysis tool written in Rust for quickly diagnosing GPU system issues`
- Website: 可以添加文档链接
- Topics: 如上所述

### Branch Protection (可选)
Settings -> Branches -> Add rule:
- Branch name pattern: `main`
- ✅ Require pull request reviews before merging
- ✅ Require status checks to pass before merging

## 持续集成 (可选)

创建 `.github/workflows/rust.yml`:
```yaml
name: Rust

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

env:
  CARGO_TERM_COLOR: always

jobs:
  build:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    
    steps:
    - uses: actions/checkout@v3
    - name: Build
      run: cargo build --release --verbose
    - name: Run tests
      run: cargo test --verbose
    - name: Upload artifacts
      uses: actions/upload-artifact@v3
      with:
        name: nvidia_log_parser-${{ matrix.os }}
        path: target/release/nvidia_log_parser*
```

## 后续维护

### 更新代码
```bash
git add .
git commit -m "描述你的更改"
git push
```

### 创建新版本
```bash
# 更新版本号
# 编辑 Cargo.toml 中的 version

# 提交更改
git add Cargo.toml
git commit -m "Bump version to v0.3.0"

# 创建 tag
git tag -a v0.3.0 -m "Release v0.3.0"
git push origin v0.3.0

# 创建 release
gh release create v0.3.0
```

## 推荐的 README 徽章

在 README.md 顶部添加：
```markdown
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg)](https://www.rust-lang.org/)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![GitHub release](https://img.shields.io/github/release/YOUR_USERNAME/nvidia-bug-report-analyzer.svg)](https://github.com/YOUR_USERNAME/nvidia-bug-report-analyzer/releases)
```
