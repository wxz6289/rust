# WebAssembly

JavaScript的动态类型和垃圾收集机制会使JavaScript很难保证可靠的性能，即使使用JIT优化,也会因类型的变化导致性能急剧下降。

1. 更小的`.wasm`文件 无GC
2. 无需重写全部,只需迁移性能敏感的部分
3. 与现有的工具链结合很容易

可移植、紧凑、执行效率高，线性内存模型

两种格式

- wat S表达式
- wasm 二进制可执行文件

## wesm-pack

```bash
curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh
cargo install wasm-pack
npm install -g wasm-pack

```

## cargo-generate

```bash
cargo install cargo-generate
cargo generate --git https://github.com/rustwasm/wasm-pack-template

```
