/// 所有权规则 (Ownership Rules)
///
/// Rust 的核心特性是所有权。这套规则让 Rust 无需垃圾回收 (GC) 就能保证内存安全。
///
/// 所有权规则：
/// 1. Rust 中的每一个值都有一个被称为其所有者 (owner) 的变量。
/// 2. 值在任一时刻有且只有一个所有者。
/// 3. 当所有者（变量）离开作用域，这个值将被丢弃 (drop)。

fn main() {
    // --- 1. 作用域 (Scope) ---
    {
        // s 在这里无效，它尚未声明
        let _s = "hello"; // 从此处起，s 是有效的
                          // 使用 s
    } // 此作用域已结束，s 不再有效

    // --- 2. String 类型 ---
    // 前几章看到的字符串字面值是硬编码在程序里的。
    // String 类型分配在堆上，可以存储在编译时未知的文本。
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("{}", s);

    // --- 3. 移动 (Move) ---
    // 在 Java 中，s1 和 s2 会指向堆上的同一个对象（引用复制）。
    // 在 Rust 中，为了防止“二次释放” (double free)，当我们将 s1 赋值给 s2，
    // s1 会失效。这个操作被称为 移动 (Move)。
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{}, world!", s1); // ❌ 编译错误！s1 的所有权已经移动给了 s2
    println!("{}, world!", s2); // ✅ s2 现在是所有者

    // --- 4. 克隆 (Clone) ---
    // 如果我们确实需要深度复制堆上的数据，可以使用 clone 方法。
    let s3 = String::from("hello");
    let s4 = s3.clone();

    println!("s3 = {}, s4 = {}", s3, s4); // ✅ 正常运行，两者都有效

    // --- 5. 栈上数据的拷贝 (Copy Trait) ---
    // 像整型这种在编译时已知大小的类型，被完整地存储在栈上。
    // 对于这些类型，赋值操作是快速的深拷贝，不需要移动。
    let x = 5;
    let y = x; // 这里没有所有权转换，x 依然有效
    println!("x = {}, y = {}", x, y);

    // --- 6. 所有权与函数 ---
    let s_new = String::from("hello");
    takes_ownership(s_new); // s_new 的值移动到函数里 ...
                            // ... 所以这里不再有效

    // println!("{}", s_new); // ❌ 编译错误！

    let x = 5;
    makes_copy(x); // x 移动到函数里，
                   // 但 i32 是 Copy 的，所以在后面可继续使用
    println!("main 里的 x: {}", x);
}

fn takes_ownership(some_string: String) {
    println!("函数取得所有权: {}", some_string);
} // 这里 some_string 离开作用域，`drop` 被调用，内存被释放

fn makes_copy(some_integer: i32) {
    println!("函数取得拷贝: {}", some_integer);
}
