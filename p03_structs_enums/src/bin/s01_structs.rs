/// 结构体 (Structs)
///
/// 结构体是一种自定义数据类型，允许我们将多个相关的值组合在一起，
/// 形成一个有意义的命名组合。在 Java 中，这类似于一个只有字段（和方法）的类。

// 1. 定义结构体
// 使用 #[derive(Debug)] 可以让我们用 {:?} 打印整个结构体进行调试
#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 2. 元组结构体 (Tuple Structs)
// 当你想给整个元组起个名字，但不需要给每个字段命名时很有用
#[derive(Debug)]
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 3. 单元结构体 (Unit-Like Structs)
// 没有任何字段的结构体，常用于在某个类型上实现 Trait 但不需要存储数据
struct AlwaysAlive;

fn main() {
    // --- 实例化结构体 ---
    let mut user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };

    // 访问字段
    user1.email = String::from("anotheremail@example.com");
    println!("User1 的邮箱是: {}", user1.email);

    // --- 结构体更新语法 (Struct Update Syntax) ---
    // 根据 user1 创建 user2，只改变 email，其余字段相同
    // 注意：这里 username 是 String，发生了所有权转移！
    let user2 = User {
        email: String::from("user2@example.com"),
        ..user1 // 必须放在最后
    };

    println!("User2: {:?}", user2);
    // println!("User1: {:?}", user1); // ❌ 错误！user1.username 的所有权已经由于更新语法转移给了 user2

    // --- 元组结构体的使用 ---
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    println!("黑色 RGB: {}, {}, {}", black.0, black.1, black.2);

    // --- 结构体的方法 (impl) ---
    let rect = Rectangle {
        width: 30,
        height: 50,
    };

    println!("矩形的面积是: {}", rect.area());

    let rect2 = Rectangle {
        width: 10,
        height: 40,
    };
    println!("rect 能包含 rect2 吗? {}", rect.can_hold(&rect2));

    // 静态方法（关联函数）
    let square = Rectangle::square(20);
    println!("正方形: {:?}", square);
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 使用 impl 块为结构体定义方法
impl Rectangle {
    // &self 表示借用实例，不拿走所有权
    fn area(&self) -> u32 {
        self.width * self.height
    }

    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }

    // 关联函数 (Associated Functions)
    // 类似于 Java 的 static 方法，不带 self 参数
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
}
