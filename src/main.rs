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
    // println!("{}", s); // 这里再使用 s 会报错，因为所有权已经转移

    // 闭包作为函数参数
    let numbers = vec![1, 2, 3, 4, 5];
    let result = apply_operation(&numbers, |x| x * 2);
    println!("result = {:?}", result);

    // 闭包作为返回值
    let multiplier = make_multiplier(3);
    println!("multiplier(4) = {}", multiplier(4));

    // iterator 常用闭包
    let numbers = vec![1, 2, 3, 4, 5];
    let sum: i32 = numbers.iter().sum();
    println!("sum = {}", sum);

    let sum2: i32 = numbers.iter().fold(0, |acc, x| acc + x);
    println!("fold sum = {}", sum2);

    // filter 使用闭包
    let even: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("even numbers = {:?}", even);

    // map 使用闭包
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("doubled = {:?}", doubled);

    // 结构体中使用闭包
    let calculator = Calculator {
        result: 0,
        operation: Box::new(|a, b| a + b),
    };
    println!("calculator(1, 2) = {}", calculator.calculate(1, 2));

    let calculator2 = Calculator {
        result: 0,
        operation: Box::new(|a, b| a * b),
    };
    println!("calculator2(3, 4) = {}", calculator2.calculate(3, 4));
}

// 闭包作为函数参数
fn apply_operation(numbers: &[i32], op: fn(i32) -> i32) -> Vec<i32> {
    numbers.iter().map(op).collect()
}

// 闭包作为返回值
fn make_multiplier(factor: i32) -> impl Fn(i32) -> i32 {
    move |x| x * factor
}

// 结构体中使用闭包
struct Calculator {
    result: i32,
    operation: Box<dyn Fn(i32, i32) -> i32>,
}

impl Calculator {
    fn calculate(&self, a: i32, b: i32) -> i32 {
        (self.operation)(a, b)
    }
}
