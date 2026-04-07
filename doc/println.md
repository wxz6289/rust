# println! 宏用法笔记

println! 是 Rust 中最常用的输出宏之一，用于向标准输出打印一行文本，并在末尾自动追加换行。

## 1. 最基础用法

```rust
fn main() {
	println!("hello");
	println!("{}", 42);
  println!("name = {}, age = {}", "Tom", 18);
}
```

规则：

- 第一个参数是格式字符串
- 后续参数按占位符顺序填充
- 使用一对大括号表示一个占位符

## 2. 常见变体

```rust
print!("no newline");      // 不自动换行
println!("with newline");  // 自动换行
eprintln!("error log");    // 输出到标准错误
```

## 3. 占位符与参数绑定

### 按顺序绑定

```rust
println!("{} + {} = {}", 1, 2, 3);
```

### 按位置索引

```rust
println!("{1} {0}", "world", "hello");
```

### 按名称绑定

```rust
println!("{name} is {age}", name = "Alice", age = 20);
```

### 使用捕获变量（Rust 1.58+）

```rust
let lang = "Rust";
println!("I love {lang}");
```

## 4. 输出特征：Display 与 Debug

Rust 的格式化本质是基于 trait。

- {} 使用 Display
- {:?} 使用 Debug
- {:#?} 使用美化 Debug（多行缩进）

```rust
#[derive(Debug)]
struct User {
	id: u32,
	name: String,
}

fn main() {
	let u = User { id: 1, name: "Bob".to_string() };
	println!("{}", 123);      // Display
	println!("{:?}", u);      // Debug
	println!("{:#?}", u);     // Pretty Debug
}
```

## 5. 重点：格式化微语言（format spec）

println! 中最重要的部分是格式化微语言。完整形式可理解为：

```text
{参数:填充+对齐 宽度 .精度 类型}
```

你不一定每次都写全，常见是写其中一部分。

### 5.1 对齐与填充

```rust
println!("|{:>8}|", "cat");   // 右对齐
println!("|{:<8}|", "cat");   // 左对齐
println!("|{:^8}|", "cat");   // 居中
println!("|{:*^8}|", "cat");  // 用 * 填充并居中
```

### 5.2 宽度 width

```rust
println!("|{:5}|", 12);      // 最小宽度 5
println!("|{:05}|", 12);     // 用 0 补齐
```

### 5.3 精度 precision

通常用于浮点数，表示小数位数。

```rust
let pi = 3.1415926;
println!("{:.2}", pi);       // 3.14
println!("{:.4}", pi);       // 3.1416
```

也可用于字符串截断（按字符边界处理）。

```rust
println!("{:.3}", "abcdef"); // abc
```

### 5.4 符号与进制

```rust
println!("{:b}", 10);        // 二进制: 1010
println!("{:o}", 10);        // 八进制: 12
println!("{:x}", 255);       // 十六进制小写: ff
println!("{:X}", 255);       // 十六进制大写: FF
println!("{:+}", 5);         // 强制显示正负号: +5
println!("{:+}", -5);        // -5
```

### 5.5 井号 # 的常见用法

```rust
println!("{:#x}", 255);      // 0xff
println!("{:#X}", 255);      // 0xFF
println!("{:#b}", 10);       // 0b1010
println!("{:#o}", 10);       // 0o12
```

### 5.6 动态宽度与动态精度

宽度和精度可以来自参数。

```rust
let w = 8;
let p = 3;
let n = 12.34567;

println!("|{:>width$}|", 42, width = w);
println!("{:.precision$}", n, precision = p);
```

也可用位置参数指定动态宽度：

```rust
println!("|{:>1$}|", 42, 6);   // 宽度 6
```

## 6. 大括号转义

格式字符串中如果要输出字面量大括号，要写成双大括号。

```rust
println!("{{}} => braces");
```

## 7. 与 format!、write! 的关系

- format!：返回 String，不直接打印
- println!：打印到标准输出并换行
- write! / writeln!：写入实现了 std::fmt::Write 或 std::io::Write 的目标

```rust
let s = format!("name = {}", "Tom");
println!("{s}");
```

## 8. 最常用速查

```rust
println!("{}", value);        // Display
println!("{:?}", value);      // Debug
println!("{:#?}", value);     // Pretty Debug
println!("{:>8}", value);     // 右对齐 + 宽度
println!("{:.2}", value);     // 精度
println!("{:#x}", value);     // 带前缀十六进制
println!("{name}");           // 捕获变量
```

## 9. 常见错误

- 占位符数量与参数数量不一致
- 对自定义类型使用 {} 但没有实现 Display
- 忘记为需要 Debug 输出的类型添加 derive(Debug)
- 在格式字符串中直接写 { 或 } 导致解析错误（应使用 {{ 和 }}）
