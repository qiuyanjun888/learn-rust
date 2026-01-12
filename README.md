# Rust 学习路径 (2026 最新版 - Rust 1.92.0)

欢迎开启 Rust 编程之旅！本项目采用 **Cargo Workspace (工作空间)** 模式组织，每一章都是一个独立的 Package，每一个知识点都是一个独立的二进制程序。

---

## 📂 项目目录结构

```text
learn-rust/
├── Cargo.toml               # 工作空间配置 (Workspace)
├── README.md                # 本文档
├── pXX_章节名称/             # 文件夹带编号，独立文件夹 (Package)
│   ├── Cargo.toml           # 该 Package 的独立配置与依赖
│   └── src/bin/             # 存放独立知识点的文件夹
│       └── sXX_知识点名称.rs  # 带编号的独立程序 (每个文件都有 main 函数)
└── output_lab/              # 🎯 实战产出实验室 (各个阶段的大型输出任务)
```

## 🚀 如何运行代码

你可以通过以下方式运行特定的知识点程序：
`cargo run -p <包名> --bin <文件名>`
例如：`cargo run -p p01_basics --bin s01_variables`

---

## � 完整学习计划 (含 1.92.0 进阶内容)

### 🟢 阶段一：基础入门 (p01_basics)
1.  **s01_variables**: 变量定义、不可变性、Shadowing。
2.  **s02_types**: 标量类型（含 1.92.0 字符处理）与复合类型。
3.  **s03_control_flow**: `if` 表达式（含 `Result` 返回优化）。

### 🦀 阶段二：核心所有权系统 (p02_ownership)
1.  **s01_ownership**: 移动 (Move) 与 克隆 (Clone) 的底层内存逻辑。
2.  **s02_references**: 引用与不变量借用规则。
3.  **s03_lifetimes**: 理解引用流转与生命周期标注。

### 🧬 阶段三：类型系统与高级枚举 (p03_structs_enums)
1.  **s01_structs**: 结构体定义与 `impl` 方法块。
2.  **s02_enums**: 关联数据枚举、`Option` 与 `Result` 的深度用法。
3.  **s03_never_type**: **(🌟 1.92.0)** 理解 `!` 类型在不可到达路径中的应用。

### 💡 阶段四：进阶类型与抽象 (p04_generics_traits)
1.  **s01_generics**: 泛型函数与泛型结构体。
2.  **s02_traits**: 特性定义、Trait Bound 约束（Java Interface 对比）。
3.  **s03_const_context**: **(🌟 1.92.0)** 在 `const` 环境下的切片操作。

### 🛠️ 阶段五：错误处理与模块化 (p05_errors_modules)
1.  **s01_errors**: 自定义错误、`?` 运算符链式调用。
2.  **s02_modules**: `pub`, `use` 作用域与 Crate 层级管理。

### 🧠 阶段六：高级内存管理 (p06_advanced_memory)
1.  **s01_smart_pointers**: `Box`, `Rc`, `Arc` 与 `new_zeroed` 零初始化。
2.  **s02_interior_mutability**: `RefCell` 与 `RwLock` 的动态借用检查。

### ⚡ 阶段七：并发与异步 (p07_concurrency)
1.  **s01_threads**: 多线程协作、消息传递 (Channel)。
2.  **s02_async**: `async/await` 异步流、Tokio 基础应用。

### 🚀 阶段八：工程化与优化 (p08_engineering)
1.  **s01_optimization**: **(🌟 1.92.0)** Cargo 构建加速与 Profile 配置。
2.  **s02_debugging**: 利用 1.92.0 改进的 Backtrace 进行调试。

### 🧬 阶段九：深度生命周期与内存模型 (p09_lifetimes_mastery)
1.  **s01_struct_references**: 结构体持有引用时的 `'a` 约束。
2.  **s02_reborrowing**: 借用穿透与多层引用传递逻辑。
3.  **s03_hrtb**: 高阶 Trait 约束 (`for<'a>`) 与闭包。
4.  **s04_pin_unpin**: 理解自引用结构与 `Pin` 指针。

### 🛠️ 阶段十：元编程与黑魔法 (p10_macros_unsafe)
1.  **s01_decl_macros**: `macro_rules!` 声明式宏实战。
2.  **s02_proc_macros**: `Derive` 与 `Attribute` 过程宏开发。
3.  **s03_unsafe**: 裸指针操作、内存 Layout 与 FFI。

### ⚡ 阶段十一：高性能异步 (p11_advanced_async)
1.  **s01_future_trait**: 手写 `Future` 与状态机理解。
2.  **s02_executor**: 手撸简易异步运行时。

---

## 🎯 每个阶段的实战任务 (Practical Output)

每当学习完一个阶段的所有知识点（sXX）后，请到 `output_lab/` 目录下完成对应的综合练习：

- **阶段一 (入门)**：`r-grep-lite` —— 实现一个支持参数解析的基础搜索工具。
- **阶段二 (所有权)**：`my-vec` —— 手写一个能自动扩容的动态数组容器（理解 Move）。
- **阶段三 (类型)**：`ast-interpreter` —— 利用枚举支持简单的数学表达式递归求值。
- **阶段四 (抽象)**：`generic-serializer` —— 编写一个能将不同结构体序列化为 CSV 的 Trait。
- **阶段九 (生命周期)**：`zero-copy-parser` —— 实现一个禁止拷贝、完全基于引用的高性能日志解析器。
- **阶段十 (元编程)**：`sql-derive` —— 编写宏自动根据结构体生成 SQL 语句。
- **阶段十一 (异步)**：`mini-runtime` —— 手工实现 `Poll` 和 `Waker` 以驱动异步任务。

---

## 🛠️ 学习建议 (Java 开发者视角)
- **关注 `!` 类型**：在 Java 中方法不返回则抛异常；在 Rust 1.92 中利用 `!` 在编译期确信某路径不会进入。
- **内存布局思维**：Java 屏蔽了内存地址，在完成 `my-vec` 和 `zero-copy-parser` 时要时刻思考数据在栈还是堆。
