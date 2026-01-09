fn main() {
    println!("--- p01_basics: s03_control_flow ---");

    // ==========================================
    // 1. If 表达式 (If is an Expression)
    // ==========================================
    let number = 7;

    // Rust 的 if 是一个表达式，这意味着它可以返回值！
    // 类似于 Java 的三元运算符 condition ? a : b，但功能更强
    let condition = true;
    let result = if condition { 5 } else { 6 };
    println!("Result from if expression: {}", result);

    // 注意：Rust 强制要求 if 的条件必须是 bool 类型，不会自动把 0 或 null 转为 false
    if number < 5 {
        println!("Condition was true");
    } else {
        println!("Condition was false");
    }

    // ==========================================
    // 2. 循环 (Loops)
    // ==========================================

    // (A) loop: 真正的死循环
    let mut count = 0;
    let loop_result = loop {
        count += 1;
        if count == 10 {
            // loop 也可以返回值！通过 break 传出
            break count * 2;
        }
    };
    println!("Loop result: {}", loop_result);

    // 如果不返回数据，那么不用赋值且break 里不返回
    println!("=======");
    let mut count1 = 0;
    loop {
        count1 += 1;
        if count1 == 10 {
            // 直接跳出循环
            break;
        }
    }
    println!("Loop result: {}", count1);

    // (B) while: 标准的条件循环
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");

    // (C) for: 极其常用的遍历
    // 遍历集合
    let a = [10, 20, 30, 40, 50];
    print!("Array traversal: ");
    for element in a {
        print!("{} ", element);
    }
    println!();

    // 遍历范围 (Range)
    // (1..4) 是左闭右开 [1, 4)
    // (1..=4) 是全闭 [1, 4]
    print!("Range 1 to 3: ");
    for i in 1..4 {
        print!("{} ", i);
    }
    println!();

    // 逆向遍历
    print!("Reverse range: ");
    for i in (1..4).rev() {
        print!("{} ", i);
    }
    println!();

    // ==========================================
    // 3. 循环标签 (Loop Labels)
    // ==========================================
    // Java 也有 Label，但 Rust 的更现代
    'outer: loop {
        println!("Entered the outer loop");
        loop {
            println!("Entered the inner loop");
            // 跳出指定的循环
            break 'outer;
        }
    }
}
