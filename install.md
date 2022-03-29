Rust是一门赋予每个人构建可靠且高效软件能力的语言。

为什么选择Rust?  
+ 高性能 速度惊人且内存利用率极高。没有运行时和垃圾回收，轻松与其他语言集成。
+ 可靠性 丰富的类型系统和所有权模型保证了内存安全和线程安全，在编译期就能够消除各种各样的错误。
+ 高效 出色的文档、友好的编译器和清晰的错误提示，还集成了一流的包管理工具和构建工具，智能地自动补全和类型检验的多编辑器支持，以及自动格式化代码等。

Rust构建的应用
+ 命令行
+ WebAssembly
+ 网络服务
+ 嵌入式
  
特点：
快速、跨平台、低资源占有


安装Rust
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

source $HOME/.cargo/env
// or
export PATH="$HOME/.cargo/bin:$PATH"
rustc --version
```

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

```bash
cargo new project_name
cargo build
cargo build --release
cargo run 
cargo run --release
```


数据类型
+ 自动推导
+ 类型标注

标量类型 单值
+ 整数  i8/u8, i16/u16, i32/u32, i64/u64, isize/usize
+ 浮点数 f32, f64
+ 布尔值 bool 1个字节
+ 字符 char 4个字节 
  
复合类型  多值多类型
+ 元组  可以不同类型，固定长度
+ 数组  同一类型，固定长度  栈存储

标识符 snake case 蛇型命名法
函数 
```rust 
fn function_name(){} 
```

Rust不关心在那里定义函数，只要函数对于使用区域可见。
参数变量 parameter
传参 argument

表达式 执行计算并产生一个值作为结果的指令
语句 执行一些操作但不返回值的指令

函数返回值 
需要声明返回类型
可以使用return提前返回
若不指定，则隐式返回最后一个表达式的值

注释
+ 行注释 //
+ 块注释 /*  */
+ 文档注释  

控制流 
