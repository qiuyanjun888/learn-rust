/// 引用与借用 (References and Borrowing)
///
/// 在上一节中，我们看到将 String 传给函数会导致所有权转移。
/// 如果我们只想让函数“用一下”这个值，而不拿走所有权，该怎么办？
/// 答案就是：引用 (References)。

fn main() {
    // --- 1. 不可变引用 (&T) ---
    // 引用就像是指针，它指向一个值，但并不拥有它。
    let s1 = String::from("hello");

    // 我们使用 &s1 传递一个引用，而不是 s1 本身
    let len = calculate_length(&s1);

    // 因为只是借用，s1 在这里依然有效！
    println!("字符串 '{}' 的长度是 {}。", s1, len);

    // --- 2. 可变引用 (&mut T) ---
    // 默认情况下，引用也是不可变的。如果我们想修改借用的值，需要使用 &mut。
    let mut s2 = String::from("hello");

    change(&mut s2);
    println!("修改后的字符串: {}", s2);

    // --- 3. 引用的规则 (重要！) ---
    // 为了保证内存安全，Rust 对引用有严格的限制：

    // 规则 A: 在同一时间，你只能拥有【一个可变引用】或者【任意多个不可变引用】。
    // 这防止了“数据竞争” (Data Race)。
    let mut s3 = String::from("hello");

    // 总结，如果要可变引用，那么之前不能声明不可变引用
    let r1 = &s3; // ✅ 没问题
    let r2 = &s3; // ✅ 没问题
                  // let r3 = &mut s3; // ❌ 编译错误！不能在有不可变引用时，再创建可变引用
    println!("{} and {}", r1, r2);

    // 在 r1, r2 离开作用域（最后一次使用）后，我们可以创建可变引用
    let r3 = &mut s3; // ✅ 现在没问题了
                      // let r4 = &mut s3; // ❌ 编译错误！不能同时创建两个可变引用
    r3.push_str(" world");
    println!("{}", r3);

    // 规则 B: 引用必须始终有效 (防止悬垂引用)。
    // Rust 编译器保证引用指向的数据不会在引用失效前被释放。
    // let reference_to_nothing = dangle(); // ❌ 编译错误！
}

fn calculate_length(s: &String) -> usize {
    // s 是一个指向 String 的引用
    s.len()
} // 这里 s 离开作用域，但因为它不拥有它指向的值，所以什么也不会发生

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

/*
fn dangle() -> &String {
    // dangle 返回一个字符串的引用
    let s = String::from("hello"); // s 是一个新字符串

    &s // ❌ 返回 s 的引用。
} // 这里 s 离开作用域并被丢弃。其内存被释放。
  // 危险！这个引用指向的是无效内存。Rust 会在编译期阻止这种行为。
*/
