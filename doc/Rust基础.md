# Rust 语言基础

本文档涵盖 Rust 编程语言的核心概念，包括变量、数据类型、函数、控制流和重要的语言特性。

## 目录

1. [变量与常量](#变量与常量)
2. [数据类型](#数据类型)
3. [函数](#函数)
4. [控制流](#控制流)
5. [结构体](#结构体)
6. [枚举](#枚举)
7. [模式匹配](#模式匹配)
8. [集合类型](#集合类型)

## 变量与常量

### 变量声明

在 Rust 中，变量默认是**不可变的**（immutable）。如需声明可变变量，需要使用 `mut` 关键字。

```rust
// 不可变变量
let x = 5;
// x = 10; // 编译错误！不能重新赋值

// 可变变量
let mut y = 5;
y = 10; // ✅ 正确
```

### 常量

使用 `const` 关键字声明常量，常量永远不可变，不能使用 `mut` 修饰。

```rust
const MAX_POINTS: u32 = 100_000;
const PI: f64 = 3.14159;
```

**常量的特点：**

- 必须显式标注类型
- 只能绑定到常量表达式，不能是运行时计算的值
- 可以在任何作用域中声明，包括全局作用域
- 按照约定使用全大写字母和下划线命名

### 变量隐藏（Shadowing）

可以使用 `let` 关键字重复声明同名变量，新变量会隐藏之前的变量。

```rust
let x = 5;
let x = x + 1;      // x = 6
let x = x * 2;      // x = 12
let x = "hello";    // 类型也可以改变
```

**隐藏 vs 可变变量的区别：**

| 特性       | 变量隐藏 | 可变变量  |
|----------|----------|-----------|
| 关键字     | `let`    | `let mut` |
| 类型改变   | ✅ 允许   | ❌ 不允许  |
| 创建新变量 | ✅ 是     | ❌ 否      |
| 最终可变性 | 可选择   | 始终可变  |

## static

在 Rust 中，使用 `static` 关键字可以声明静态变量。静态变量在整个程序运行期间都存在，并且只能在声明时初始化一次。 静态变量默认是不可变的，但可以使用 `mut` 关键字声明可变的静态变量。需要注意的是，访问可变静态变量时必须使用 `unsafe` 块，因为它们可能会引发数据竞争。

```rust
static mut MAX_POINTS: u32 = 100_000;
```

## 数据类型

Rust 是**静态类型语言**，在编译时需要知道所有变量的类型。编译器通常可以自动推导类型，但在无法推导时需要手动标注。

### 类型系统特点

- **自动类型推导**：编译器根据值和使用方式推导类型
- **类型标注**：在需要时显式指定类型
- **类型安全**：编译期保证类型正确性

```rust
// 自动推导
let x = 42;           // i32
let y = 3.14;         // f64

// 显式标注
let z: u32 = 100;
let name: &str = "Rust";
```

### 标量类型（Scalar Types）

标量类型表示单个值，Rust 有四种基本标量类型：

#### 1. 整数类型

| 有符号  | 无符号  | 字节数   |
|---------|---------|----------|
| `i8`    | `u8`    | 1        |
| `i16`   | `u16`   | 2        |
| `i32`   | `u32`   | 4        |
| `i64`   | `u64`   | 8        |
| `i128`  | `u128`  | 16       |
| `isize` | `usize` | 平台相关 |

```rust
// 数字字面值可以使用类型后缀和分隔符
let million = 1_000_000u32;
let hex = 0xff;
let octal = 0o77;
let binary = 0b1111_0000;
let byte = b'A';  // 只能用于 u8
```

**整数溢出处理：**

- **Debug 模式**：触发 panic
- **Release 模式**：执行二进制补码环绕

#### 2. 浮点数类型

```rust
let x = 2.0;      // f64（默认）
let y: f32 = 3.0; // f32（显式标注）
```

- `f32`：单精度浮点数
- `f64`：双精度浮点数（默认，更精确）
- 都遵循 IEEE-754 标准

#### 3. 布尔类型

```rust
let t = true;
let f: bool = false;
```

- 占用 1 个字节
- 只有 `true` 和 `false` 两个值

#### 4. 字符类型

```rust
let c = 'z';
let z = 'ℤ';
let heart_eyed_cat = '😻';
```

- 使用单引号声明
- 占用 4 个字节
- 表示 Unicode 标量值（U+0000 到 U+D7FF 和 U+E000 到 U+10FFFF）

### 复合类型（Compound Types）

复合类型可以将多个值组合成一个类型。

#### 1. 元组（Tuple）

```rust
// 声明
let tup: (i32, f64, u8) = (500, 6.4, 1);

// 解构
let (x, y, z) = tup;

// 索引访问
let five_hundred = tup.0;
let six_point_four = tup.1;

// 空元组（单元类型）
let unit = ();
```

**特点：**

- 固定长度
- 可以包含不同类型
- 使用圆括号

#### 2. 数组（Array）

```rust
// 声明
let a = [1, 2, 3, 4, 5];
let a: [i32; 5] = [1, 2, 3, 4, 5];

// 重复值初始化
let a = [3; 5];  // [3, 3, 3, 3, 3]

// 访问元素
let first = a[0];
let second = a[1];
```

**特点：**

- 固定长度
- 相同类型元素
- 存储在栈上
- 使用方括号

#### 3. 动态数组（Vector）

```rust
// 创建
let mut v = Vec::new();
v.push(1);
v.push(2);

// 宏创建
let v = vec![1, 2, 3];

// 访问
let third = &v[2];
let third = v.get(2);  // 返回 Option<&T>
```

**特点：**

- 可变长度
- 相同类型元素
- 存储在堆上

### 类型别名（Type Alias）

使用 `type` 关键字可以为已有类型创建别名。

```rust
type Kilometers = i32;
let x: Kilometers = 5;
```

### 命名约定

```rust
// 变量和函数：snake_case
let my_variable = 42;
fn calculate_sum() {}

// 常量：SCREAMING_SNAKE_CASE
const MAX_VALUE: u32 = 100;

// 类型：PascalCase
struct MyStruct {}
enum MyEnum {}
```

## 函数

Rust 使用 `fn` 关键字声明函数，采用 snake_case 命名规范。

### 函数定义

```rust
fn function_name() {
    println!("Hello, world!");
}

// 带参数的函数
fn greet(name: &str) {
    println!("Hello, {}!", name);
}

// 带返回值的函数
fn add(x: i32, y: i32) -> i32 {
    x + y  // 隐式返回（最后一个表达式）
}

// 显式返回
fn multiply(x: i32, y: i32) -> i32 {
    return x * y;  // 提前返回
}
```

### 函数参数

- **参数**（parameters）：函数定义中的变量
- **实参**（arguments）：调用函数时传入的具体值

```rust
fn calculate(x: i32, y: i32) -> i32 {
    //       ↑参数
    x + y
}

let result = calculate(5, 3);  // 5, 3 是实参
```

### 语句与表达式

理解语句和表达式的区别是掌握 Rust 的关键：

#### 语句（Statements）

执行操作但不返回值的指令

```rust
let x = 5;           // 变量绑定语句
fn foo() {}          // 函数定义语句
let y = (let x = 6); // 错误！let 是语句，不返回值
```

#### 表达式（Expressions）

执行计算并产生一个值作为结果的指令

```rust
5                    // 字面值表达式
5 + 6               // 数学运算表达式
{                   // 代码块表达式
    let x = 3;
    x + 1           // 返回 4
}
```

### 返回值

```rust
// 方式1：隐式返回（推荐）
fn add_one(x: i32) -> i32 {
    x + 1  // 注意：没有分号
}

// 方式2：显式返回
fn add_two(x: i32) -> i32 {
    return x + 2;
}

// 方式3：混合使用
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // 提前返回
    }
    x  // 隐式返回
}
```

**注意事项：**

- 函数的返回类型必须在 `->` 后声明
- 没有分号的表达式作为返回值
- 有分号的表达式变成语句，返回 `()`

### 函数调用规则

```rust
fn main() {
    greet();        // ✅ 函数可以在任何地方定义

    fn greet() {    // ✅ 嵌套函数定义
        println!("Hello!");
    }
}

fn another_function() {
    greet();        // ✅ 只要在作用域内可见即可
}
```

Rust 不关心函数定义的位置，只要函数对使用区域可见即可。

## 控制流

### 条件语句（if 表达式）

```rust
let number = 6;

// 基本 if 表达式
if number % 4 == 0 {
    println!("number is divisible by 4");
} else if number % 3 == 0 {
    println!("number is divisible by 3");
} else if number % 2 == 0 {
    println!("number is divisible by 2");
} else {
    println!("number is not divisible by 4, 3, or 2");
}
```

**重要特点：**

- 条件表达式必须是 `bool` 类型
- 不需要用括号包围条件
- 只执行第一个为真的分支
- `if` 是表达式，可以返回值

#### if 表达式的返回值

```rust
let condition = true;
let number = if condition { 5 } else { 6 };

// 错误示例：分支返回不同类型
// let number = if condition { 5 } else { "six" }; // 编译错误
```

### 循环

#### 1. loop - 无限循环

```rust
loop {
    println!("again!");
    break;  // 退出循环
}

// 从循环返回值
let result = loop {
    counter += 1;

    if counter == 10 {
        break counter * 2;  // 返回值
    }
};
```

#### 2. while - 条件循环

```rust
let mut number = 3;

while number != 0 {
    println!("{}!", number);
    number -= 1;
}

println!("LIFTOFF!!!");
```

#### 3. for - 迭代循环

```rust
// 迭代数组
let a = [10, 20, 30, 40, 50];
for element in a.iter() {
    println!("the value is: {}", element);
}

// 范围循环
for number in (1..4).rev() {  // 3, 2, 1
    println!("{}!", number);
}

// 带索引的迭代
for (index, value) in a.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
}
```

### 循环标签

```rust
'outer: loop {
    println!("Entered the outer loop");

    'inner: loop {
        println!("Entered the inner loop");
        break 'outer;  // 跳出外层循环
    }

    println!("This point will never be reached");
}
```

## 结构体

结构体是自定义数据类型，可以将相关的数据组合在一起。

### 定义和实例化

```rust
// 定义结构体
struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}

// 创建实例
let user1 = User {
    email: String::from("someone@example.com"),
    username: String::from("someusername123"),
    active: true,
    sign_in_count: 1,
};

// 访问字段
let email = user1.email;

// 可变实例
let mut user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername"),
    active: true,
    sign_in_count: 1,
};
user2.email = String::from("newemail@example.com");
```

### 结构体更新语法

使用 `..` 语法从其他实例复制字段值：

```rust
let user2 = User {
    email: String::from("another@example.com"),
    username: String::from("anotherusername"),
    ..user1  // 从 user1 复制其余字段
};
```

**注意：** `..` 必须位于最后，表示未显式设置的字段都与给定实例拥有相同的值。

### 结构体的类型

#### 1. 具名字段结构体（上面的例子）

#### 2. 元组结构体

不需要为字段命名，仅保留字段类型：

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

let black = Color(0, 0, 0);
let origin = Point(0, 0, 0);

// 访问字段
let red_component = black.0;
```

**应用场景：** 需要为元组命名以区别于拥有相同定义的其他元组类型。

#### 3. 单元结构体

没有任何字段的结构体：

```rust
struct AlwaysEqual;

let subject = AlwaysEqual;
```

**应用场景：** 需要在某个类型上实现 trait，但不需要存储任何数据。

### 方法

方法是与结构体关联的函数，第一个参数总是 `self`：

```rust
impl User {
    // 方法
    fn is_active(&self) -> bool {
        self.active
    }

    // 可变方法
    fn deactivate(&mut self) {
        self.active = false;
    }

    // 获取所有权的方法（较少使用）
    fn into_username(self) -> String {
        self.username
    }
}

// 使用方法
let is_active = user1.is_active();
```

### 关联函数

不接收 `self` 参数的函数，类似其他语言的静态方法：

```rust
impl User {
    // 关联函数（构造器）
    fn new(email: String, username: String) -> User {
        User {
            email,
            username,
            active: true,
            sign_in_count: 1,
        }
    }
}

// 使用关联函数
let user = User::new(
    String::from("test@example.com"),
    String::from("testuser")
);
```

### 派生 trait

使用 `#[derive]` 注解为结构体自动实现常用 trait：

```rust
#[derive(Debug, Clone, PartialEq)]
struct Rectangle {
    width: u32,
    height: u32,
}

let rect = Rectangle { width: 30, height: 50 };
println!("{:?}", rect);  // 需要 Debug trait
```

常用的可派生 trait：

- `Debug`：调试输出
- `Clone`：克隆
- `Copy`：复制语义
- `PartialEq`：相等比较
- `Eq`：完全相等
- `PartialOrd`：部分排序
- `Ord`：完全排序
- `Hash`：哈希

## 枚举

枚举允许定义一个类型，它的值只能是几个可能的变体之一。

### 基本枚举

```rust
enum IpAddrKind {
    V4,
    V6,
}

let four = IpAddrKind::V4;
let six = IpAddrKind::V6;
```

### 带数据的枚举

每个枚举成员可以拥有不同类型和数量的关联数据：

```rust
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

let home = IpAddr::V4(127, 0, 0, 1);
let loopback = IpAddr::V6(String::from("::1"));

// 更复杂的例子
enum Message {
    Quit,                       // 无关联数据
    Move { x: i32, y: i32 },   // 匿名结构体
    Write(String),              // 字符串
    ChangeColor(i32, i32, i32), // 三个 i32
}
```

### Option 枚举

`Option<T>` 是 Rust 标准库中最重要的枚举之一，用于处理可能不存在的值：

```rust
enum Option<T> {
    None,
    Some(T),
}

// 使用示例
let some_number = Some(5);
let some_string = Some("a string");
let absent_number: Option<i32> = None;

// 安全地处理可能不存在的值
fn find_first_a(s: &str) -> Option<usize> {
    for (index, character) in s.chars().enumerate() {
        if character == 'a' {
            return Some(index);
        }
    }
    None
}
```

`Option<T>` 类型包含在预导入模块中，无需显式引入作用域。

### 枚举方法

```rust
impl Message {
    fn call(&self) {
        match self {
            Message::Quit => println!("Quit"),
            Message::Move { x, y } => println!("Move to ({}, {})", x, y),
            Message::Write(text) => println!("Text: {}", text),
            Message::ChangeColor(r, g, b) => println!("RGB({}, {}, {})", r, g, b),
        }
    }
}

let m = Message::Write(String::from("hello"));
m.call();
```

## 模式匹配

模式匹配是 Rust 中处理枚举和数据结构的强大工具。

### match 表达式

`match` 是 Rust 中最强大的控制流构造，允许将值与一系列模式比较：

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

### 绑定值的模式

可以在匹配时提取枚举成员的关联数据：

```rust
enum UsState {
    Alabama,
    Alaska,
    // ...
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("Lucky penny!");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

### 匹配 Option\<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

let five = Some(5);
let six = plus_one(five);
let none = plus_one(None);
```

### 穷尽性检查

Rust 的 match 必须覆盖所有可能的情况：

```rust
// 编译错误：缺少 None 分支
fn plus_one_bad(x: Option<i32>) -> Option<i32> {
    match x {
        Some(i) => Some(i + 1),
        // 缺少 None => ... 分支
    }
}
```

### 通配符 `_`

用于匹配不需要处理的值：

```rust
let dice_roll = 9;
match dice_roll {
    3 => add_fancy_hat(),
    7 => remove_fancy_hat(),
    _ => (), // 忽略其他所有值
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
```

### if let 语法

当只关心一个匹配分支时，可以使用 `if let` 简化代码：

```rust
// 使用 match
let some_u8_value = Some(0u8);
match some_u8_value {
    Some(3) => println!("three"),
    _ => (),
}

// 使用 if let（更简洁）
let some_u8_value = Some(0u8);
if let Some(3) = some_u8_value {
    println!("three");
}

// 带 else 分支
if let Some(3) = some_u8_value {
    println!("three");
} else {
    println!("not three");
}
```

### 模式的组成

模式通常由以下组件组合而成：

- **字面量**：`1`, `"hello"`, `true`
- **解构**：数组、枚举、结构体或元组的解构
- **变量**：捕获匹配的值
- **通配符**：`_` 忽略值
- **占位符**：`..` 忽略部分值

```rust
// 解构结构体
struct Point {
    x: i32,
    y: i32,
}

let p = Point { x: 0, y: 7 };
match p {
    Point { x, y: 0 } => println!("On the x axis at {}", x),
    Point { x: 0, y } => println!("On the y axis at {}", y),
    Point { x, y } => println!("On neither axis: ({}, {})", x, y),
}

// 解构元组
let tuple = (5, 10, 15);
match tuple {
    (x, 10, z) => println!("Matched: x={}, z={}", x, z),
    _ => println!("Not matched"),
}
```

### 可失败模式 vs 不可失败模式

- **不可失败模式**：总是会匹配成功（如 `let` 语句）
- **可失败模式**：可能匹配失败（如 `if let`, `match` 的某些分支）

```rust
// 不可失败模式
let x = 5;           // 总是成功
let (a, b) = (1, 2); // 总是成功

// 可失败模式
if let Some(x) = Some(5) {  // 可能失败
    println!("{}", x);
}
```

### match vs if let 的选择

| 特性       | match      | if let         |
|----------|------------|----------------|
| 穷尽性检查 | ✅ 必须     | ❌ 无           |
| 多分支     | ✅ 支持     | ❌ 单分支       |
| 简洁性     | ❌ 相对复杂 | ✅ 简洁         |
| 适用场景   | 完整匹配   | 单一关心的情况 |

## 集合类型

Rust 标准库提供了多种集合类型，数据存储在堆上，运行时可以动态调整大小。

### 动态数组（Vector）

`Vec<T>` 是最常用的集合类型：

```rust
// 创建 Vector
let v: Vec<i32> = Vec::new();
let v = vec![1, 2, 3]; // 使用宏创建

// 添加元素
let mut v = Vec::new();
v.push(5);
v.push(6);
v.push(7);

// 访问元素
let third: &i32 = &v[2];           // 可能 panic
let third: Option<&i32> = v.get(2); // 安全访问

// 遍历
for i in &v {
    println!("{}", i);
}

// 可变遍历
for i in &mut v {
    *i += 50;
}
```

### 字符串（String）

```rust
// 创建字符串
let mut s = String::new();
let s = String::from("hello");
let s = "hello".to_string();

// 更新字符串
s.push_str(" world");
s.push('!');

// 连接字符串
let s1 = String::from("Hello, ");
let s2 = String::from("world!");
let s3 = s1 + &s2; // s1 被移动，不能再使用

// 使用 format! 宏
let s = format!("{}-{}", "Hello", "world");
```

### 哈希映射（HashMap）

```rust
use std::collections::HashMap;

// 创建
let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

// 从向量创建
let teams = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];
let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();

// 访问值
let team_name = String::from("Blue");
let score = scores.get(&team_name);

// 遍历
for (key, value) in &scores {
    println!("{}: {}", key, value);
}

// 更新
scores.insert(String::from("Blue"), 25); // 覆盖

// 只在不存在时插入
scores.entry(String::from("Red")).or_insert(50);

// 基于旧值更新
let count = map.entry(word).or_insert(0);
*count += 1;
```

## 注释和文档

### 普通注释

```rust
// 行注释
/* 块注释 */

/*
 * 多行块注释
 * 第二行
 */
```

### 文档注释

```rust
/// 计算两个数的和
///
/// # Examples
///
/// ```
/// let result = add(2, 3);
/// assert_eq!(result, 5);
/// ```
fn add(a: i32, b: i32) -> i32 {
    a + b
}

//! 这是模块级别的文档注释
//! 用于描述整个模块或 crate
```

## 宏

宏是根据预定义规则进行文本替换的强大工具：

```rust
// 常用宏
println!("Hello, {}!", name);    // 格式化输出
vec![1, 2, 3];                   // 创建向量
panic!("Something went wrong!"); // 触发 panic

// 自定义宏示例
macro_rules! say_hello {
    () => (
        println!("Hello!");
    );
}

say_hello!();
```

## 项目配置文件

### Cargo.toml 基本结构

```toml
[package]
name = "my_project"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = "1.0"
tokio = { version = "1.0", features = ["full"] }

[dev-dependencies]
criterion = "0.3"
```

### 常用片段说明

- `[package]`：项目元信息
- `[dependencies]`：运行时依赖
- `[dev-dependencies]`：开发和测试依赖
- `[build-dependencies]`：构建脚本依赖

---
