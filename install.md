# Rust 编程语言指南

Rust 是一门赋予每个人构建可靠且高效软件能力的系统编程语言。

## 语言特性

### 核心特性

- **内存安全**：无垃圾回收器的情况下保证内存安全
- **零成本抽象**：高级特性不会带来运行时开销
- **所有权系统**：独特的所有权模型管理内存
- **并发安全**：防止数据竞争，安全处理并发
- **预编译静态语言**：编译后无需运行时环境

### 编程范式

- **非面向对象**：不支持传统继承，但提供方法语法
- **函数式编程**：支持高阶函数、闭包等特性
- **泛型编程**：强大的类型系统和泛型支持
- **模式匹配**：强大的 match 表达式

### 语法特点

- **类型注解**：静态类型检查
- **条件编译**：根据条件编译不同代码
- **隐式返回**：函数最后一个表达式作为返回值
- **共享不可变，可变不共享**：核心借用规则

## 为什么选择 Rust？

### 优势

- **🚀 高性能**
  - 速度惊人且内存利用率极高
  - 无运行时和垃圾回收开销
  - 轻松与其他语言集成

- **🛡️ 可靠性**
  - 丰富的类型系统保证内存安全
  - 所有权模型确保线程安全
  - 编译期消除各种错误

- **⚡ 开发效率**
  - 出色的文档和友好的编译器
  - 清晰的错误提示信息
  - 一流的包管理和构建工具
  - 智能代码补全和类型检验

### 应用领域

- **系统编程**：操作系统、驱动程序
- **网络服务**：高性能 Web 服务器
- **命令行工具**：跨平台命令行应用
- **WebAssembly**：高性能 Web 应用
- **嵌入式开发**：物联网设备
- **区块链**：加密货币和智能合约
- **游戏开发**：高性能游戏引擎

### 安全性保障

Rust 在编译期防止以下常见安全问题：

- **悬垂指针**：防止访问已释放的内存
- **数据竞争**：确保并发访问的安全性
- **缓冲区溢出**：防止越界访问
- **迭代器失效**：确保迭代过程中集合不被修改

### 挑战与限制

- **学习曲线陡峭**：所有权概念需要时间理解
- **编译速度**：相比解释型语言编译较慢
- **严格的借用检查**：某些复杂数据结构实现困难
- **生态系统**：相比成熟语言库较少（但在快速发展）

## 生命周期系统

Rust 的安全性检查基于生命周期系统，它能够验证所有对数据的访问是否有效。生命周期系统通常是自动推断的，无需手动干预，但在某些复杂情况下需要显式标注。

## 安装 Rust

### 官方安装方式

使用 rustup（推荐）：

```bash
# 下载并安装 rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 配置环境变量
source $HOME/.cargo/env

# 或者手动添加到 shell 配置文件
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.bash_profile
# 对于 zsh 用户
echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc

# 验证安装
rustc --version
cargo --version
```

### macOS 额外要求

安装 C 编译器（Xcode Command Line Tools）：

```bash
xcode-select --install
```

### Rust 工具链管理

```bash
# 更新 Rust 到最新版本
rustup update

# 查看已安装的工具链
rustup show

# 安装特定版本
rustup install stable
rustup install beta
rustup install nightly

# 设置默认工具链
rustup default stable

# 添加组件
rustup component add rustfmt    # 代码格式化工具
rustup component add clippy     # 代码检查工具
rustup component add rls        # Language Server

# 查看本地文档
rustup doc

# 完全卸载 Rust
rustup self uninstall
```

## 核心工具链

### rustc - 编译器

```bash
# 直接编译 Rust 文件
rustc main.rs

# 指定输出文件名
rustc main.rs -o my_program

# 编译时优化
rustc -O main.rs

# 查看编译选项
rustc --help
```

### cargo - 包管理器

Cargo 是 Rust 的构建系统和包管理器，提供项目管理、依赖管理、构建和发布等功能。

#### 项目创建与初始化

```bash
# 创建新的二进制项目
cargo new my_project

# 创建新的库项目
cargo new my_lib --lib

# 在当前目录初始化项目
cargo init

# 指定版本控制系统（默认 git）
cargo new my_project --vcs=git
cargo new my_project --vcs=none
```

#### 项目构建与运行

```bash
# 编译项目（调试模式）
cargo build

# 编译项目（发布模式，优化）
cargo build --release

# 编译并运行
cargo run

# 发布模式运行
cargo run --release

# 静默运行（减少输出）
cargo run -q

# 详细输出
cargo run -v

# 只检查代码，不生成可执行文件（速度更快）
cargo check
```

#### 测试与文档

```bash
# 运行所有测试
cargo test

# 运行特定测试
cargo test test_function_name

# 生成并打开项目文档
cargo doc --open

# 生成文档（不打开）
cargo doc

# 运行文档测试
cargo test --doc
```

#### 项目维护

```bash
# 清理构建产物
cargo clean

# 更新依赖到最新兼容版本
cargo update

# 格式化代码
cargo fmt

# 代码检查和建议
cargo clippy

# 审计依赖的安全漏洞
cargo audit  # 需要先安装：cargo install cargo-audit
```

#### 发布管理

```bash
# 发布到 crates.io
cargo publish

# 验证发布包
cargo package

# 检查发布包内容
cargo package --list
```

### rustfmt - 代码格式化

```bash
# 格式化当前项目所有 Rust 文件
cargo fmt

# 格式化单个文件
rustfmt src/main.rs

# 检查格式但不修改
cargo fmt -- --check

# 自定义配置文件 rustfmt.toml
echo 'max_width = 120' > rustfmt.toml
```

### clippy - 代码检查

```bash
# 运行 clippy 检查
cargo clippy

# 将警告视为错误
cargo clippy -- -D warnings

# 修复可自动修复的问题
cargo clippy --fix
```

## 配置镜像源

在中国大陆，建议配置镜像源以加速依赖下载。

创建或编辑 `~/.cargo/config.toml` 文件：

```toml
# 字节跳动源（推荐）
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

# 稀疏索引（Cargo >= 1.68）
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

# 网络配置
[net]
git-fetch-with-cli = true

# 其他镜像源选择

# 清华大学源
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 中科大源
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

# 使用时将上面的 replace-with 改为对应的源名称
```

## 开发环境配置

### VS Code 扩展推荐

```bash
# 必装扩展
code --install-extension rust-lang.rust-analyzer  # 官方语言服务器
code --install-extension vadimcn.vscode-lldb      # 调试器
```

### Jupyter Notebook 支持

在 Jupyter 中运行 Rust 代码：

```bash
# 更新 Rust
rustup update

# 安装 evcxr_jupyter
cargo install evcxr_jupyter

# 安装到 Jupyter
evcxr_jupyter --install

# 启动 Jupyter
jupyter lab
```

### Cargo.toml 配置示例

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"
authors = ["Your Name <your.email@example.com>"]
description = "A sample Rust project"
license = "MIT"
repository = "https://github.com/username/my_project"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.5"

[[bin]]
name = "my_binary"
path = "src/bin/my_binary.rs"

[[example]]
name = "example_name"
path = "examples/example_name.rs"
```

## 常用命令速查

| 命令 | 功能 |
|------|------|
| `cargo new <name>` | 创建新项目 |
| `cargo build` | 编译项目 |
| `cargo run` | 编译并运行 |
| `cargo test` | 运行测试 |
| `cargo check` | 快速语法检查 |
| `cargo fmt` | 格式化代码 |
| `cargo clippy` | 代码检查 |
| `cargo doc` | 生成文档 |
| `cargo clean` | 清理构建文件 |
| `cargo update` | 更新依赖 |
| `rustup update` | 更新 Rust |

这样你就拥有了一个完整的 Rust 开发环境！从基本概念到实际开发工具，这份指南应该能帮助你快速上手 Rust 编程。
