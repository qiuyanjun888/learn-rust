fn main() {
    println!("--- p01_basics: s01_variables ---");
    println!("知识点：变量、可变性与类型推断");

    // ==========================================
    // 1. 变量与可变性 (Variables & Mutability)
    // ==========================================

    // 默认不可变 (Immutable by default)
    let name = "Antigravity";
    // name = "New Name"; // 这行会报错，因为 name 是不可变的

    // 显式声明可变 (Explicitly mutable)
    let mut study_days = 0;
    study_days += 1;

    println!("Welcome {}, day {} of Rust.", name, study_days);

    // ==========================================
    // 2. 类型推断 (Type Inference)
    // ==========================================

    // Rust 编译器非常聪明，可以根据初始值自动推断类型
    let is_rust_fun = true; // 推断为 bool
    let logical_score = 95; // 推断为 i32 (整数默认类型)
    let price = 10.5; // 推断为 f64 (浮点数默认类型)

    println!(
        "Inferred types: fun={}, score={}, price={}",
        is_rust_fun, logical_score, price
    );

    // ==========================================
    // 3. 显式类型标注 (Explicit Type Annotation)
    // ==========================================

    // 有时为了清晰，或者默认类型不满足需求时，可以手动标注
    let age: u32 = 18; // 显式指定为无符号 32 位整数
    let bonus: f32 = 100.25; // 显式指定为单精度浮点数

    println!("Explicit types: age={}, bonus={}", age, bonus);

    // ==========================================
    // 4. 必须手动指定类型的情况
    // ==========================================

    // (A) 常量 (Constants): 必须指定类型，且不能用 mut
    const MAX_POINTS: u32 = 100_000;
    println!("Constant MAX_POINTS: {}", MAX_POINTS);

    // (B) 无法推断的场景 (比如解析字符串)
    // 如果不写 : u32，编译会报错，因为 parse() 不知道你想转成什么数字类型
    let guessed_number: u32 = "42".parse().expect("Not a number!");
    println!("Parsed number: {}", guessed_number);

    // (C) 函数签名: 参数和返回值必须写类型 (见下方的 add 函数)
    let sum = add(5, 10);
    println!("Sum from function: {}", sum);

    // (D) 变量遮蔽
    let x = 5;
    let x = x + 1;
    {
        let x = x * 2;
        println!("x in inner scope: {}", x);
    }
    println!("x in outer scope: {}", x);
}

/// 函数定义必须包含完整的类型信息
fn add(a: i32, b: i32) -> i32 {
    a + b
}
