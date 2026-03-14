fn main() {
    println!("=== Rust 闭包演示 ===\n");

    // 1. 闭包的基本语法
    let add = |a, b| a + b;
    println!("add(2, 3) = {}", add(2, 3));

    // 多行闭包
    let multiply = |a, b| {
        let result = a * b;
        result
    };
    println!("multiply(4, 5) = {}", multiply(4, 5));

    // 2. 闭包捕获环境
    let x = 10;
    let add_to_x = |y| x + y;
    println!("add_to_x(5) = {}", add_to_x(5));

    // 3. 闭包的类型推导
    // 闭包不需要显式类型标注，Rust 会自动推导
    let closure = |num| num * 2;
    let i32_result = closure(5);
    let f64_result = closure(5.0); // 这里会失败，因为类型已确定
    println!("closure(5) = {}", i32_result);

    // 4. Fn, FnMut, FnOnce
    // Fn: 不可变借用
    // FnMut: 可变借用
    // FnOnce: 获取所有权

    let fn_closure = |x| println!("Fn: {}", x);
    apply_fn(fn_closure);

    let mut count = 0;
    let mut increment = || {
        count += 1;
        println!("FnMut: {}", count);
    };
    apply_fn_mut(increment);

    let s = String::from("hello");
    let consume = || println!("FnOnce: {}", s);
    apply_fn_once(consume);

    // 5. 闭包作为函数参数
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = sum_with(numbers, |x| x * 2);
    println!("sum with closure: {}", sum);

    // 6. 捕获方式
    let s1 = String::from("hello");
    let s2 = "world";

    let c1 = || println!("{}", s1); // 不可变借用
    let c2 = || println!("{} {}", s1, s2); // 多个变量借用

    let mut s3 = String::from("hi");
    let c3 = || s3.push_str("!"); // 可变借用

    c1();
    c2();
    c3();
    println!("{}", s3);

    // move 关键字
    let s4 = String::from("moved");
    let c4 = move || println!("{}", s4); // 获取所有权
    c4();
    // println!("{}", s4); // 错误！s4 已被移动

    println!("\n=== 总结 ===");
    println!("闭包是匿名函数，可以捕获环境");
    println!("Rust 自动推导闭包类型：Fn, FnMut, FnOnce");
    println!("move 关键字强制获取变量所有权");
    println!("闭包常用于迭代器、函数参数等场景");
}

// Fn trait
fn apply_fn<F: Fn(i32)>(f: F) {
    f(42);
}

// FnMut trait
fn apply_fn_mut<F: FnMut()>(mut f: F) {
    f();
}

// FnOnce trait
fn apply_fn_once<F: FnOnce()>(f: F) {
    f();
}

// 闭包作为参数
fn sum_with<F>(nums: Vec<i32>, f: F) -> i32
where
    F: Fn(i32) -> i32,
{
    nums.iter().map(|&x| f(x)).sum()
}
