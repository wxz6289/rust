# Rust 语言基础

这是一份面向入门到进阶前阶段的 Rust 基础笔记，目标是：

- 梳理核心知识点
- 突出高频重点
- 配合最小可运行示例
- 补充常见易错点

## 目录

1. [学习地图](#学习地图)
2. [重点速记](#重点速记)
3. [变量常量与作用域](#变量常量与作用域)
4. [所有权借用与生命周期](#所有权借用与生命周期)
5. [常用数据类型](#常用数据类型)
6. [函数与表达式](#函数与表达式)
7. [控制流](#控制流)
8. [结构体与方法](#结构体与方法)
9. [枚举与模式匹配](#枚举与模式匹配)
10. [集合与字符串](#集合与字符串)
11. [错误处理](#错误处理)
12. [模块包与可见性](#模块包与可见性)
13. [trait 与泛型基础](#trait-与泛型基础)
14. [并发基础](#并发基础)
15. [宏与文档注释](#宏与文档注释)
16. [工程实践清单](#工程实践清单)

## 学习地图

可以把 Rust 基础按 3 层理解：

- 语法层：变量、函数、类型、控制流
- 语义层：所有权、借用、生命周期、模式匹配
- 工程层：模块组织、错误处理、并发、文档与测试

如果你觉得 Rust 难，通常不是语法难，而是语义层没有完全建立。

## 重点速记

- 变量默认不可变，变更要用 mut
- String、Vec 等默认在堆上，赋值常发生移动
- 同一时刻要么多个不可变借用，要么一个可变借用
- 悬垂引用在编译期被禁止
- Option 用于“可能为空”，Result 用于“可能失败”
- match 必须穷尽所有分支
- if 是表达式，函数可用最后一个表达式隐式返回
- for 优先于手写索引循环
- 先写正确性，再谈 unsafe

## 变量常量与作用域

### 变量与可变性

```rust
let x = 5;
let mut y = 10;
y += 1;
```

### 常量与静态变量

```rust
const MAX_RETRY: u32 = 3;

static APP_NAME: &str = "learn-rust";
```

说明：

- const 在编译期内联，必须显式类型
- static 拥有固定内存地址，生命周期贯穿程序运行期
- static mut 需要 unsafe，日常优先使用原子类型或锁

### 变量遮蔽

```rust
let spaces = "   ";
let spaces = spaces.len();
```

适合“同名不同语义”的阶段性转换。

## 所有权借用与生命周期

这是 Rust 最核心的部分。

### 所有权三规则

- 每个值有且仅有一个所有者
- 同一时刻所有者唯一
- 所有者离开作用域时值被释放

```rust
let s1 = String::from("hello");
let s2 = s1; // move
// println!("{}", s1); // 编译错误
```

### 借用

```rust
fn len_of(s: &String) -> usize {
    s.len()
}

let s = String::from("rust");
let n = len_of(&s);
```

### 可变借用规则

```rust
let mut s = String::from("abc");
let r1 = &mut s;
r1.push('d');
// 同一作用域内不能再创建其他借用直到 r1 不再使用
```

### 切片

```rust
let s = String::from("hello");
let h = &s[0..2];
println!("{}", h);
```

### 生命周期的直觉

生命周期不是让对象活更久，而是描述“引用有效区间”。

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

## 常用数据类型

### 标量类型

- 整数：i8..i128, u8..u128, isize, usize
- 浮点：f32, f64
- 布尔：bool
- 字符：char（4 字节 Unicode 标量值）

### 复合类型

#### 元组

```rust
let tup: (i32, f64, &str) = (1, 2.0, "ok");
let (a, b, c) = tup;
```

#### 数组

```rust
let arr = [1, 2, 3, 4];
let repeat = [0; 5];
```

#### 向量 Vec

```rust
let mut v = vec![1, 2, 3];
v.push(4);
```

## 函数与表达式

### 函数定义

```rust
fn add(x: i32, y: i32) -> i32 {
    x + y
}
```

### 语句与表达式

```rust
let x = {
    let a = 1;
    a + 2 // 表达式，无分号
};
```

要点：

- 有分号通常是语句，返回 ()
- 无分号通常是表达式，返回该值

## 控制流

### if 表达式

```rust
let n = 7;
let kind = if n % 2 == 0 { "even" } else { "odd" };
```

### 循环

```rust
for i in 0..3 {
    println!("{}", i);
}

let mut n = 3;
while n > 0 {
    n -= 1;
}

let x = loop {
    break 42;
};
```

## 结构体与方法

```rust
#[derive(Debug, Clone)]
struct User {
    name: String,
    active: bool,
}

impl User {
    fn new(name: String) -> Self {
        Self { name, active: true }
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}
```

重点：

- 方法第一个参数通常是 self, &self, &mut self
- 关联函数不含 self，常作构造器

## 枚举与模式匹配

### 枚举

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
}
```

### match

```rust
fn handle(m: Message) {
    match m {
        Message::Quit => println!("quit"),
        Message::Move { x, y } => println!("move to {}, {}", x, y),
        Message::Write(s) => println!("{}", s),
    }
}
```

### Option 与 if let

```rust
let maybe = Some(3);
if let Some(v) = maybe {
    println!("{}", v);
}
```

## 集合与字符串

### String

```rust
let mut s = String::from("hello");
s.push_str(" rust");
let t = format!("{}!", s);
```

### HashMap

```rust
use std::collections::HashMap;

let mut map = HashMap::new();
map.insert("blue", 10);
*map.entry("blue").or_insert(0) += 1;
```

## 错误处理

### 不可恢复错误

```rust
panic!("unexpected state");
```

### 可恢复错误

```rust
use std::fs::File;

fn open_file() -> Result<File, std::io::Error> {
    File::open("hello.txt")
}
```

### ? 运算符

```rust
use std::fs;

fn read_name() -> Result<String, std::io::Error> {
    let s = fs::read_to_string("name.txt")?;
    Ok(s)
}
```

建议：

- 库代码优先返回 Result
- 应用入口可统一处理错误并输出上下文

## 模块包与可见性

### crate / module / path

- crate：编译单元（bin 或 lib）
- mod：模块
- use：引入路径
- pub：导出可见性

```rust
mod math {
    pub fn add(x: i32, y: i32) -> i32 {
        x + y
    }
}

fn main() {
    let v = math::add(1, 2);
    println!("{}", v);
}
```

## trait 与泛型基础

### 泛型函数

```rust
fn largest<T: PartialOrd + Copy>(arr: &[T]) -> T {
    let mut max = arr[0];
    for &item in arr {
        if item > max {
            max = item;
        }
    }
    max
}
```

### trait 定义与实现

```rust
trait Summary {
    fn summary(&self) -> String;
}

struct News {
    title: String,
}

impl Summary for News {
    fn summary(&self) -> String {
        format!("news: {}", self.title)
    }
}
```

## 并发基础

Rust 并发重点是“通过类型系统预防数据竞争”。

### 线程

```rust
use std::thread;

let handle = thread::spawn(|| {
    println!("from thread");
});

handle.join().unwrap();
```

### 消息传递

```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();
thread::spawn(move || {
    tx.send(String::from("hi")).unwrap();
});

println!("{}", rx.recv().unwrap());
```

### 共享状态

```rust
use std::sync::{Arc, Mutex};

let n = Arc::new(Mutex::new(0));
{
    let mut g = n.lock().unwrap();
    *g += 1;
}
```

## 宏与文档注释

### 常用宏

- println!
- format!
- vec!
- dbg!
- panic!

### 文档注释

```rust
/// 计算和
///
/// # Examples
///
/// ```
/// assert_eq!(add(1, 2), 3);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}
```


建议习惯：

- 先建模再写实现
- 先用 Result 传递错误，少用 unwrap
- 出现借用报错时，先缩小引用作用域
- 在接口边界写注释和测试

---

如果你已经掌握本文内容，下一步可进入：

- 生命周期进阶（结构体生命周期参数）
- trait object 与动态分发
- async/await 与 tokio
- unsafe Rust 边界设计
