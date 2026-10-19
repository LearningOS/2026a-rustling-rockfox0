# Options

Type Option 表示一个可选值：每个 Option 要么是 Some 且包含值，要么是 None 且不包含值。
选项类型在 Rust 代码中非常常见，因为它们有多种用途：

-初始值
-未在整个输入范围内定义的函数的返回值（部分函数）
-用于报告简单错误的返回值，其中错误时不返回 None
-可选的结构字段
-可以借出或“拿走”的结构字段
-可选函数参数
-可空指针
-摆脱困境

## Further Information

- [Option Enum Format](https://doc.rust-lang.org/stable/book/ch10-01-syntax.html#in-enum-definitions)
- [Option Module Documentation](https://doc.rust-lang.org/std/option/)
- [Option Enum Documentation](https://doc.rust-lang.org/std/option/enum.Option.html)
- [if let](https://doc.rust-lang.org/rust-by-example/flow_control/if_let.html)
- [while let](https://doc.rust-lang.org/rust-by-example/flow_control/while_let.html)
