/// 生命周期 (Lifetimes)
///
/// 生命周期是 Rust 编译器用来确保所有引用都有效的一种方式。
/// 它并不改变引用的存活时间，而是描述了引用之间的关系。

fn main() {
    // --- 1. 为什么需要生命周期？ ---
    // 考察下面这个例子：
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("最长的字符串是: {}", result);
    } // string2 在这里被销毁

    // println!("{}", result); // ❌ 如果这一行取消注释，编译会报错！
    // 因为 result 引用的可能是 string2，而 string2 已经死了。这就是悬垂引用。

    // --- 2. 如何在作用域外使用结果？ ---

    // 方案 2.1: 延长原始数据的生命周期
    let s1 = String::from("long");
    let s2 = String::from("short");
    let res;
    {
        res = longest(s1.as_str(), s2.as_str());
    }
    println!("方案 2.1 (延长寿命): {}", res);

    // 方案 2.2: 克隆数据，从“引用”变为“所有”
    let s3 = String::from("long");
    let res_owned: String;
    {
        let s4 = String::from("short");
        let temp_res = longest(s3.as_str(), s4.as_str());
        res_owned = temp_res.to_string(); // 这里发生了拷贝，创建了新的所有权
    } // s4 在这里死了，但 res_owned 依然活着
    println!("方案 2.2 (克隆副本): {}", res_owned);

    // --- 3. 解决你之前的疑问：如何正确从函数返回？ ---
    let returned_string = return_owned_string(); // 方案 A：移交所有权
    println!("从函数拿回了所有权: {}", returned_string);
}

/// 方案 A：返回所有权 (Owned Value)
/// 这种方式最简单，不需要考虑生命周期，因为所有权转移了。
fn return_owned_string() -> String {
    let s = String::from("I am owned by the caller now");
    s // 所有权移动
}

/// 方案 B：生命周期标注 (&'a str)
/// 当函数接受多个引用并返回一个引用时，Rust 有时不知道返回的引用到底
/// 指向哪一个输入。这时需要“生命周期标注”。
///
/// 这里 <'a> 告诉编译器：返回的引用，其寿命至少和输入参数里的 'a 一样长。
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

/*
// ❌ 错误做法（也就是你之前问的 dangle 问题）
fn return_reference() -> &String {
    let s = String::from("hello");
    &s // 错误！s 会在函数结束时被销毁，返回的引用将指向无效内容
}
*/
