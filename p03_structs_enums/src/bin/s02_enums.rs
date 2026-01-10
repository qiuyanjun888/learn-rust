/// 枚举 (Enums)
///
/// 枚举允许你通过枚举一个类型的可能值来定义一个类型。
///
/// 【Java 开发者视角】：
/// - Java Enum: 主要是命名常量的集合，可以有字段和方法，但所有实例共享相同的字段结构。
/// - Rust Enum: 是“代数数据类型”(Algebraic Data Types, ADTs)。每个变体 (Variant)
///   都可以拥有完全不同的数据结构（甚至没有数据）。这类似于 Java 中的sealed class/interface 层次结构。

#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 1. 枚举变体与关联数据 (Associated Data)
// 变体可以包含各异的数据：元组形式、结构体形式、或者空。
#[derive(Debug)]
enum Message {
    Quit,                       // 类似于单元结构体，不带数据
    Move { x: i32, y: i32 },    // 类似于匿名结构体
    Write(String),              // 包含单个 String
    ChangeColor(i32, i32, i32), // 包含三个 i32
}

// 2. 为枚举定义方法
impl Message {
    fn call(&self) {
        // match 表达式必须穷举所有可能性
        match self {
            Message::Quit => println!("收到退出信号"),
            Message::Move { x, y } => println!("移动到坐标: ({}, {})", x, y),
            Message::Write(text) => println!("正在写入: {}", text),
            Message::ChangeColor(r, g, b) => {
                println!("颜色变更: R:{}, G:{}, B:{}", r, g, b)
            }
        }
    }
}

fn main() {
    // --- 基础用法 ---
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    // --- 实例化带数据的枚举 ---
    let m1 = Message::Write(String::from("Hello Rust 1.92"));
    let m2 = Message::Move { x: 10, y: 20 };
    let m3 = Message::ChangeColor(255, 0, 0);
    let m4 = Message::Quit;

    m1.call();
    m2.call();
    m3.call();
    m4.call();

    // --- Option<T> 枚举 ---
    // Rust 移除了 Null。取而代之的是标准库中的 Option<T>。
    // 这迫使开发者必须处理“无”的情况，从而避免了类似于 Java 的 NullPointerException。
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None; // 必须显式指定类型

    println!(
        "Option 示例: {:?}, {:?}, {:?}",
        some_number, some_char, absent_number
    );

    // --- 使用 match 提取 Option 中的值 ---
    let x = Some(10);
    let plus_one = match x {
        None => None,
        Some(i) => Some(i + 1),
    };
    println!("加一后的结果: {:?}", plus_one);

    // --- if let 控制流 ---
    // 如果你只想处理一种情况，match 会显得繁琐。if let 是 match 的语法糖。
    let config_max = Some(3u8);
    if let Some(max) = config_max {
        println!("最大配置为: {}", max);
    } else {
        // 可选的 else 块，相当于 match 中的 _ => ...
        println!("未设置配置");
    }

    // --- Result<T, E> 枚举 ---
    // Java 使用 Exception 处理异常；Rust 使用 Result 处理可恢复错误。
    // enum Result<T, E> { Ok(T), Err(E) }
    let success = perform_operation(true);
    let failure = perform_operation(false);

    handle_result(success);
    handle_result(failure);
}

// Result 示例函数
fn perform_operation(is_ok: bool) -> Result<i32, String> {
    if is_ok {
        Ok(42)
    } else {
        Err(String::from("操作失败：网络超时"))
    }
}

fn handle_result(res: Result<i32, String>) {
    match res {
        Ok(value) => println!("成功！返回值为: {}", value),
        Err(e) => println!("错误: {}", e),
    }
}
