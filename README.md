# Rust 学习路径 (2026 最新版 - Rust 1.92.0)

欢迎开启 Rust 编程之旅！本项目采用 **Cargo Workspace (工作空间)** 模式组织，每一章都是一个独立的 Package，每一个知识点都是一个独立的二进制程序。

---

## 📂 项目目录结构

```text
learn-rust/
├── Cargo.toml               # 工作空间配置 (Workspace)
├── README.md                # 本文档
└── pXX_章节名称/             # 文件夹带编号，独立文件夹 (Package)
    ├── Cargo.toml           # 该 Package 的独立配置与依赖
    └── src/bin/             # 存放独立知识点的文件夹
        └── sXX_知识点名称.rs  # 带编号的独立程序 (每个文件都有 main 函数)
```

## 🚀 如何运行代码

你可以通过以下两种方式运行特定的知识点程序：

### 1. 在根目录下运行 (推荐)
格式：`cargo run -p <包名> --bin <文件名>`
```bash
# 例如运行“基础篇”中的“变量”知识点
cargo run -p p01_basics --bin s01_variables
```

### 2. 进入具体章节文件夹运行
```bash
cd p01_basics
cargo run --bin s01_variables
```

---

## 📅 完整学习计划 (已集成 1.92.0 新特性)

### 🟢 阶段一：基础入门 (p01_basics)
1.  **s01_variables**: 变量定义、不可变性、Shadowing。
2.  **s02_types**: 标量类型（含 1.92.0 强化的字符处理）与复合类型。
3.  **s03_control_flow**: `if` 表达式（含 `Result` 返回优化）。

### 🦀 阶段二：核心所有权系统 (p02_ownership)
1.  **所有权规则 (Ownership)**：移动 (Move) 与 克隆 (Clone)。
2.  **引用与借用 (References & Borrowing)**。
3.  **生命周期 (Lifetimes)**：理解函数间的引用流转。

### 🧬 阶段三：类型系统与高级枚举 (p03_structs_enums)
1.  **结构体 (Structs)**：方法实现 `impl`。
2.  **枚举 (Enums)**：关联数据，`Option<T>` 与 `Result<T, E>`。
3.  **Never Type (`!`) 🌟**: (Update 1.92.0) 理解 `!` 类型及其在错误处理中的严谨性（Deny-by-default lints）。

### 💡 阶段四：进阶类型与抽象 (p04_generics_traits)
1.  **特性 (Traits)**：类比 Java `Interface`，Trait Bound 约束。
2.  **Const Context 🌟**: (Update 1.92.0) 学习在 `const` 环境下使用 `rotate_left` 等切片操作。

### 🛠️ 阶段五：错误处理与模块化 (p05_errors_modules)
1.  **错误处理**：`Result<(), !>` 类型的使用。
2.  **模块系统**：`pub`, `use` 可见性管理。

### 🧠 阶段六：高级内存管理 (p06_advanced_memory)
1.  **智能指针进阶**: `Box`, `Rc`, `Arc` (含 1.92.0 稳定的 `new_zeroed` 零初始化方法)。
2.  **内部可变性**: `RefCell<T>` 与 `RwLock` (含 1.92.0 的 `downgrade` 操作)。

### ⚡ 阶段七：并发与异步 (p07_concurrency)
1.  **Fearless Concurrency**: 多线程与消息传递。
2.  **Async/Await**: Tokio 运行时。

### 🚀 阶段八：工程化与优化 (p08_engineering)
1.  **构建性能优化 🌟**: (Update 1.92.0) 学习 Cargo 的构建加速技巧。
2.  **调试进阶**: 利用 1.92.0 改进的 Linux Backtrace 进行生产环境调试。

---

## 🛠️ 学习建议 (Java 开发者视角)
-   **关注 `!` 类型**：在 Java 中，一个方法如果不返回则抛异常；在 Rust 1.92 中，你可以利用 `!` 类型在编译期就确信某条路径永远不会被进入。
-   **无畏并发**：这是 Rust 1.92 依然引以为傲的特性，它能帮你解决 Java 中头疼的各种死锁和数据竞争问题。
-   **每一个知识点文件都是独立的**：可以放心地在里面做各种实验，互不干扰。
