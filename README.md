# Rust Closure Demo

## 简介

演示 Rust 闭包的用法。

## 基本原理

闭包是匿名函数，可以捕获环境变量。

## 启动和使用

```bash
cargo run
```

## 教程

### 基本语法

```rust
let add = |a, b| a + b;
let multiply = |a, b| { a * b };
```

### 闭包类型

- `Fn` - 不可变借用
- `FnMut` - 可变借用
- `FnOnce` - 获取所有权

### move 关键字

```rust
let s = String::from("hello");
let c = move || println!("{}", s);
c();
// s 已无效
```
