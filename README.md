# Rust 闭包详解

## 简介

闭包是 Rust 中一种强大的语法结构，它可以捕获周围环境的变量，形成一个"闭合"的作用域。本 Demo 将通过实例讲解 Rust 闭包的各种用法。

## 快速开始

### 环境要求

- Rust 1.56+
- Cargo

### 运行

```bash
cargo run
```

## 概念讲解

### 第一部分：基础闭包

闭包使用 `|` 包裹参数，`|a, b| expression`：

```rust
let add = |a, b| a + b;
println!("add(2, 3) = {}", add(2, 3));
```

输出：`add(2, 3) = 5`

### 第二部分：捕获环境变量

闭包可以捕获定义时周围的变量：

```rust
let x = 10;
let add_x = |y| x + y;
println!("add_x(5) = {}", add_x(5));
```

输出：`add_x(5) = 15`

### 第三部分：move 关键字

默认闭包以引用方式捕获环境，如果需要所有权的转移，使用 `move`：

```rust
let s = String::from("hello");
let print_s = move || println!("{}", s);
print_s();
// 注释掉这行，因为 s 的所有权已经转移给闭包
// println!("{}", s); // 编译错误！
```

### 第四部分：闭包作为函数参数

闭包可以作为参数传递给函数：

```rust
fn apply_operation(numbers: &[i32], op: fn(i32) -> i32) -> Vec<i32> {
    numbers.iter().map(op).collect()
}

let numbers = vec![1, 2, 3, 4, 5];
let result = apply_operation(&numbers, |x| x * 2);
```

### 第五部分：闭包作为返回值

使用 `impl Fn` 或 `Box<dyn Fn>` 返回闭包：

```rust
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

let multiplier = make_multiplier(3);
println!("multiplier(4) = {}", multiplier(4));
```

输出：`multiplier(4) = 12`

### 第六部分：Iterator 常用闭包

Iterator 的 `map`、`filter`、`fold`、`sum` 等方法都使用闭包：

```rust
let numbers = vec![1, 2, 3, 4, 5];

// sum 求和
let sum: i32 = numbers.iter().sum();

// fold 折叠
let sum2: i32 = numbers.iter().fold(0, |acc, x| acc + x);

// filter 过滤
let even: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();

// map 映射
let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
```

### 第七部分：结构体中存储闭包

使用 `Box<dyn Fn>` 在结构体中存储闭包：

```rust
struct Calculator {
    result: i32,
    operation: Box<dyn Fn(i32, i32) -> i32>,
}

impl Calculator {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}

let calculator = Calculator {
    result: 0,
    operation: Box::new(|a, b| a + b),
};
println!("calculator(1, 2) = {}", calculator.calculate(1, 2));
```

## 完整示例

完整代码如下：

```rust
fn main() {
    // 基础闭包
    let add = |a, b| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // 闭包捕获环境变量
    let x = 10;
    let add_x = |y| x + y;
    println!("add_x(5) = {}", add_x(5));

    // 使用 move 强制转移所有权
    let s = String::from("hello");
    let print_s = move || println!("{}", s);
    print_s();

    // 闭包作为函数参数
    let numbers = vec![1, 2, 3, 4, 5];
    let result = apply_operation(&numbers, |x| x * 2);
    println!("result = {:?}", result);

    // 闭包作为返回值
    let multiplier = make_multiplier(3);
    println!("multiplier(4) = {}", multiplier(4));

    // 结构体中使用闭包
    let calculator = Calculator {
        result: 0,
        operation: Box::new(|a, b| a + b),
    };
    println!("calculator(1, 2) = {}", calculator.calculate(1, 2));
}

fn apply_operation(numbers: &[i32], op: fn(i32) -> i32) -> Vec<i32> {
    numbers.iter().map(op).collect()
}

fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

struct Calculator {
    result: i32,
    operation: Box<dyn Fn(i32, i32) -> i32>,
}

impl Calculator {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}
```

## 逐行解释

1. `let add = |a, b| a + b;` - 定义一个接收两个参数并返回它们之和的闭包
2. `let add_x = |y| x + y;` - 闭包可以引用定义时环境中的变量 `x`
3. `move || println!("{}", s)` - `move` 关键字强制闭包获取变量的所有权
4. `fn apply_operation(numbers: &[i32], op: fn(i32) -> i32)` - 闭包可以作为函数参数
5. `impl Fn(i32) -> i32` - 使用 `impl Fn` 返回闭包类型
6. `Box<dyn Fn(i32, i32) -> i32>` - 使用 `Box<dyn Fn>` 在堆上存储闭包

## 闭包的三个 trait

Rust 闭包自动实现以下 trait 之一：

- `Fn` - 按引用捕获环境
- `FnMut` - 按可变引用捕获环境
- `FnOnce` - 获取环境变量的所有权（只能调用一次）

大多数闭包实现 `Fn`，如果闭包获取了环境变量的所有权，则实现 `FnOnce`。

## 总结

闭包的核心要点：

1. 语法：`|params| expression` 或 `|params| { statements }`
2. 捕获方式：默认按引用，需要所有权时用 `move`
3. 作为参数：闭包可以传递给函数
4. 作为返回值：使用 `impl Fn` 或 `Box<dyn Fn>`
5. Iterator：大量使用闭包，如 `map`、`filter`、`fold`
