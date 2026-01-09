fn main() {
    println!("--- p01_basics: s02_types ---");

    // ==========================================
    // 1. 标量类型 (Scalar Types)
    // ==========================================

    // 整数 (Integers): i8, u8, i32, u32, i128, isize...
    // Java 只有有符号整数 (byte, short, int, long)
    // Rust 区分有符号 (i) 和 无符号 (u)
    let a: i32 = -10;
    let b: u32 = 10;
    let c: isize = 100; // 根据你的 CPU 架构决定是 32 位还是 64 位 (类似 Java 的指针长度)
    println!("Integers: {}, {}, {}", a, b, c);

    // 浮点数 (Floating-Point): f32 (float), f64 (double)
    let f1: f32 = 3.14;
    let f2 = 2.0; // 默认是 f64
    println!("Floats: {}, {}", f1, f2);

    // 布尔 (Boolean): bool
    let is_active = true;
    println!("Boolean: {}", is_active);

    // 字符 (Character): char
    // 注意：Rust 的 char 是 Unicode 标量值，占 4 字节，不仅是 ASCII
    let heart_eye_emoji = '😻';
    println!("Character: {}", heart_eye_emoji);

    // ==========================================
    // 2. 复合类型 (Compound Types)
    // ==========================================

    // 元组 (Tuple)
    // Java 需要自定义 Class 来返回多个值，Rust 可以直接用元组
    let person: (&str, i32, bool) = ("Bruce", 30, true);

    // 解构元组 (Destructuring)
    let (name, age, _) = person;
    println!("Tuple - Name: {}, Age: {}", name, age);

    // 数组 (Array)
    // 注意：Rust 数组长度是固定的 (Fixed size)，且分配在栈上
    // 这点类似于 Java 的 int[] arr = new int[5]，但长度是类型的一部分
    let months = ["Jan", "Feb", "Mar"];
    let numbers: [i32; 5] = [1, 2, 3, 4, 5]; // [类型; 长度]

    println!("Array first element: {}", months[0]);
    println!("Array length: {}", numbers.len());

    // 错误示范：Rust 的数组越界会在编译期或运行时非常安全地检查，不会像 C 那样读取非法内存
    // println!("{}", numbers[10]); // 这行如果取消注释，编译会报错
}
