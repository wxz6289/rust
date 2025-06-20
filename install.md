# Rust

Rust 是一门赋予每个人构建可靠且高效软件能力的语言。

语言特性：

- Rust不支持继承，不是面向对象的，但采用了方法语法。
- 常用的流程控制机制；
- 高阶编程；
- 类型注解；
- 条件编译；
- 隐式返回；

能够保证内存安全，并且不会引入额外的运行时开销。(其他语言需要程序在执行期添加额外检查保证这种级别的安全)。

安全并高效处理并发

预编译静态语言

共享不可变，可变不共享

为什么选择 Rust?

- 高性能 速度惊人且内存利用率极高。没有运行时和垃圾回收，轻松与其他语言集成。
- 可靠性 丰富的类型系统和所有权模型保证了内存安全和线程安全，在编译期就能够消除各种各样的错误。
- 高效 出色的文档、友好的编译器和清晰的错误提示，还集成了一流的包管理工具和构建工具，智能地自动补全和类型检验的多编辑器支持，以及自动格式化代码等。

Rust 构建的应用

- 命令行
- WebAssembly
- 网络服务
- 嵌入式

特点：
快速、跨平台、低资源占有
无GC,无空值

- 高性能
- 支持并发
- 内存使用率高

安全性：

- 悬垂指针
- 数据竞争
- 缓冲区溢出
- 迭代器失效

缺点：

- 循环数据结构
- 编译速度慢
- 严格

Rust的安全性检查是基于生命周期系统的，因为生命周期系统能够验证所有对数据的访问是否有效。生命周期系统通常情况下是不需要协助的、独立进行工作。

## 安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source $HOME/.cargo/env

// or
vim ~/.bash_profile
export PATH="$HOME/.cargo/bin:$PATH"
rustc --version

# 更新rust
rustup update
# 添加工具组件
rustup component add rls
# 本地文档
rustup doc
# 卸载rust
rustup self uninstall
```

Mac安装C编译器

```bash
xcode-select --install
```

## 内置工具链

- rustup
- rustc
- cargo
- rustfmt

编译

```bash
rustc main.rs
```

## cargo

- 项目管理
- 包管理
- 构建工具

```bash
# 使用当前目录创建项目
cargo init
# 新建项目
cargo new project_name --vcs=git  --lib/bin
cargo new project --vcs none
# 编译
cargo build
cargo build --release
# 编译运行
cargo run
cargo run --release -q -v
# 语法检查
cargo check
# 执行测试用例
cargo test
# 构建项目文档
cargo doc
# 发布包
cargo publish
# 清理target目录
cargo clean
# 更新依赖
cargo update
```

换源
在`$HOME/.cargo/config`文件中编辑源地址

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'

# 科大
[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"
# 清华
[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

# 字节跳动
[source.crates-io]
replace-with = 'rsproxy'

[source.rsproxy]
registry = "https://rsproxy.cn/crates.io-index"

# 稀疏索引，要求 cargo >= 1.68
[source.rsproxy-sparse]
registry = "sparse+https://rsproxy.cn/index/"

[registries.rsproxy]
index = "https://rsproxy.cn/crates.io-index"

[net]
git-fetch-with-cli = true

```

Jupyter中运行Rust

```bash
rustup update
cargo install evcxr_jupyter
evcxr_jupyter --install
# optional
sudo xcodebuild -license
jupyter lab
```
