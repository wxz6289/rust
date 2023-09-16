# Rust

Rust 是一门赋予每个人构建可靠且高效软件能力的语言。

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

安装 Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source $HOME/.cargo/env
// or
export PATH="$HOME/.cargo/bin:$PATH"
rustc --version
```

更新

```bash
rustup update
// 添加工具组件
rustup component add rls
```

Mac安装C编译器

```bash
xcode-select --install
```

内置工具链

- rustup
- rustc
- cargo
- rustfmt

更新与卸载

```bash
rustup update
rustup self uninstall
```

本地文档

```bash
rustup doc
```

编译

```bash
rustc main.rs
```

使用 cargo 进行项目管理

```bash
cargo new project_name --vcs=git
// 编译
cargo build
cargo build --release
// 编译运行
cargo run
cargo run --release
// 语法检查
cargo check
```

换源
在`$HOME/.cargo/config`文件中编辑源地址

```toml
[source.crates-io]
registry = "https://github.com/rust-lang/crates.io-index"
replace-with = 'ustc'

[source.ustc]
registry = "git://mirrors.ustc.edu.cn/crates.io-index"

[source.tuna]
registry = "https://mirrors.tuna.tsinghua.edu.cn/git/crates.io-index.git"

```

变量声明
变量默认是不可变的，声明可变变量需要使用关键字`mut`。

常量
使用`const`声明，不能改变。不能使用`mut`关键字。
常量只能被设置为常量表达式，而不可以是其他任何只能在运行时计算出的值。

隐藏
可以定义一个与之前变量同名的新变量，后声明的变量会隐藏前面声明的同名变量。
可以用相同变量名称来隐藏一个变量，以及重复使用 let 关键字来多次隐藏。

隐藏与将变量标记为 mut 的区别

- 当不小心尝试对变量重新赋值时，如果没有使用 let 关键字，就会导致编译时错误。通过使用 let，可以用这个值进行一些计算，不过计算完之后变量仍然是不可变的。
- 当再次使用 let 时，实际上创建了一个新变量，我们可以改变值的类型，并且复用这个名字。

数据类型

- 标量
- 复合

- 自动推导
- 类型标注

标量类型 单值

- 整数 i8/u8, i16/u16, i32/u32, i64/u64, i128/u128, isize/usize
- 浮点数 f32, f64
- 布尔值 bool 1 个字节
- 字符 char 4 个字节

多种数字类型的数字字面值允许使用类型后缀，也允许使用 \_ 做为分隔符以方便读数。

复合类型 多值多类型

- 元组 可以不同类型，固定长度
- 数组 同一类型，固定长度 栈存储

标识符 snake case 蛇型命名法
函数

```rust
fn function_name(){}
```

Rust 不关心在那里定义函数，只要函数对于使用区域可见。
参数变量 parameter
传参 argument

表达式 执行计算并产生一个值作为结果的指令
语句 执行一些操作但不返回值的指令

函数返回值
需要声明返回类型
可以使用 return 提前返回
若不指定，则隐式返回最后一个表达式的值

注释

- 行注释 //
- 块注释 /\* \*/
- 文档注释

控制流

宏和函数
宏是根据一系列预定规则替换一定文本模式。实现将小命令自动转化为一系列指令。

toml 文件
[] 片段
[package]
[dependencies]

基本概念

- 变量
- 基本类型
- 函数
- 注释
- 控制流

## 结构体

结构体更新语法`..`, 表示未被显示设置的字段都与给定实例拥有相同的值,并且只能位于最后。

元组结构体
不需要为字段命名，仅保留字段类型
应用场景: 需要为元组命名以区别于拥有同样定义的元组类型

空结构体
没有任何字段
应用场景：需要在某一类型上实现一个trait,却不需要在这种类型上存储任何数据

derive注解可以派生trait,为自定义类型增加许多实用的功能。

方法和函数的异同

- 都使用`fn`及一个名称来进行声明
- 都可以拥有参数和返回值
- 都包含了一段调用时执行的代码
- 方法为实例指定行为

- 方法总定义在某个结构体、枚举类型或trait对象的上下文中
- 方法的第一个参数永远是self,用来指代调用该方法的实例
- self的类型无需显示标注，能够自动推导

关联函数 不接收self作为参数的函数，不会作用于某个实例。

## 模式匹配

模式是Rust中一种用来匹配类型结构的特殊语法。

模式通常由以下组件组合而成:

- 字面量
- 解构的数组、枚举、结构体或元组
- 变量
- 通配符
- 占位符

模式用来描述数据结构(形状)，而数据结构可以用来匹配值，因而模式用来与某个特定值进行匹配。

可失败模式与不可失败模式的区别