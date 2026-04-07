# Cargo 常用用法

Cargo 是 Rust 官方提供的构建系统和包管理器，负责项目创建、依赖管理、构建、测试、文档生成与发布。日常开发里，大部分和 Rust 工程相关的操作都可以通过 Cargo 完成。

## 1. Cargo 能做什么

Cargo 的核心职责主要有以下几类：

- 创建和初始化项目
- 管理依赖
- 构建和运行程序
- 执行测试
- 生成文档
- 发布 crate
- 管理多包工作区（workspace）

## 2. 常用目录与文件

一个典型的 Cargo 项目通常包含这些关键文件：

```text
my_project/
├── Cargo.toml
├── Cargo.lock
└── src/
	└── main.rs
```

### Cargo.toml

项目配置文件，主要用来描述：
- 包名
- 版本号 version
- Rust edition <YEAR>
- 依赖项
- 构建配置

示例：

```toml
[package]
name = "hello_cargo"
version = "0.1.0"
edition = "2021"

[dependencies]
rand = "0.8"
```

### Cargo.lock

锁定依赖的精确版本，保证不同环境下安装到一致的依赖。

- 应用项目一般建议提交到版本控制
- 库项目是否提交要看团队习惯和发布策略

### target

Cargo 的构建产物目录。

- 调试构建通常在 target/debug
- 发布构建通常在 target/release

## 3. 创建与初始化项目

### 创建二进制项目

```bash
cargo new hello_cargo
```

这会创建一个可执行程序项目，默认入口文件是 src/main.rs。

### 创建库项目

```bash
cargo new my_lib --lib
```

这会创建一个库项目，默认入口文件是 src/lib.rs。

### 在已有目录中初始化项目

```bash
cargo init
```

适合在现有目录中补充 Cargo 配置。

## 4. 构建与运行

### 编译项目

```bash
cargo build
```

默认使用调试模式构建，速度快，便于开发。

### 发布模式构建

```bash
cargo build --release
```

发布模式会启用优化，生成的程序性能更好，但编译时间更长。

### 运行项目

```bash
cargo run
```

如果项目尚未构建，Cargo 会先编译再执行。

### 发布模式运行

```bash
cargo run --release
```

### 只检查代码，不生成可执行文件

```bash
cargo check
```

这是开发中非常高频的命令。它只做语法和类型检查，速度通常明显快于 cargo build。

## 5. 依赖管理

Cargo 通过 Cargo.toml 中的 [dependencies] 管理依赖。

### 指定普通依赖

```toml
[dependencies]
serde = "1.0"
```

### 指定带 feature 的依赖

```toml
[dependencies]
tokio = { version = "1", features = ["full"] }
```

### 开发依赖

用于测试、基准测试或开发辅助工具：

```toml
[dev-dependencies]
pretty_assertions = "1"
```

### 本地路径依赖

```toml
[dependencies]
my_utils = { path = "../my_utils" }
```

### Git 依赖

```toml
[dependencies]
some_crate = { git = "https://github.com/user/some_crate" }
```

### 更新依赖

```bash
cargo update
```

这个命令会根据 Cargo.toml 的版本范围更新 Cargo.lock 中锁定的依赖版本。

### 查看依赖树

```bash
cargo tree
```

适合排查：

- 是否引入了重复依赖
- 某个 crate 是被谁间接依赖进来的
- feature 组合是否符合预期

## 6. 测试、格式化与静态检查

### 运行测试

```bash
cargo test
```

执行单元测试、集成测试和文档测试。

### 运行指定测试

```bash
cargo test test_name
```

### 格式化代码

```bash
cargo fmt
```

需要安装 rustfmt 组件。通常会在提交代码前执行一次。

### 静态检查

```bash
cargo clippy
```

Clippy 会给出很多风格、可读性和潜在问题方面的建议。

### 常见日常组合

```bash
cargo check
cargo fmt
cargo clippy
cargo test
```

这四个命令基本覆盖了日常开发中的主要检查流程。

## 7. 文档生成

### 生成文档

```bash
cargo doc
```

### 生成并打开文档

```bash
cargo doc --open
```

如果项目中写了文档注释，这个命令会非常有用。

## 8. 安装与发布

### 安装命令行工具

```bash
cargo install ripgrep
```

用于安装 crates.io 上发布的可执行程序。

### 安装当前项目

```bash
cargo install --path .
```

适合本地开发命令行工具时快速安装和测试。

### 发布 crate

```bash
cargo publish
```

将当前 crate 发布到 crates.io。发布前通常需要检查：

- 包名是否可用
- README、license、repository 等元信息是否完整
- 测试是否通过

## 9. 清理与排错

### 清理构建缓存

```bash
cargo clean
```

当构建缓存异常、磁盘占用过大或怀疑旧产物导致问题时，这个命令很有用。

### 查看项目元数据

```bash
cargo metadata
```

常用于工具集成、脚本分析或排查 workspace 结构。

### 定位项目清单文件

```bash
cargo locate-project
```

## 10. Workspace 常用用法

当一个仓库中包含多个 crate 时，通常会使用 workspace 统一管理。

示例：

```toml
[workspace]
members = [
	"app",
	"utils",
]
```

### 构建整个工作区

```bash
cargo build --workspace
```

### 测试整个工作区

```bash
cargo test --workspace
```

### 只操作某个包

```bash
cargo build -p utils
cargo test -p utils
cargo run -p app
```

这在多 crate 项目中非常常见。

## 11. 补充：常见扩展命令

有些命令不是 Cargo 内置，而是通过扩展工具提供，但在 Rust 社区里也很常用。

### cargo add

```bash
cargo add serde
```

用于快速添加依赖。通常来自 cargo-edit。

### cargo rm

```bash
cargo rm serde
```

用于移除依赖，同样通常来自 cargo-edit。

### cargo upgrade

```bash
cargo upgrade
```

用于升级依赖版本，通常也来自 cargo-edit 或相关扩展工具。

## 12. 最常用的一组命令

如果只记住最核心的一批，建议优先掌握下面这些：

```bash
cargo new project_name
cargo run
cargo check
# 构建发布版本
cargo build --release
cargo test
cargo fmt
cargo clippy
cargo tree
# 更新依赖
cargo update
cargo fix --edition # 迁移到新版本
```

## 13. 小结

可以把 Cargo 理解为 Rust 项目的总入口：

- 创建项目用 cargo new / cargo init
- 开发检查用 cargo check
- 运行程序用 cargo run
- 正式构建用 cargo build --release
- 测试代码用 cargo test
- 统一风格用 cargo fmt
- 做静态检查用 cargo clippy
- 管理依赖看 Cargo.toml、Cargo.lock 和 cargo tree
