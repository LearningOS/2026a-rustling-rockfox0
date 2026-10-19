// errors5.rs
//
//该程序使用了errors4 中代码的修改版本。
//
//本练习使用了一些概念，我们要到后面的部分才能了解这些概念
//当然，就像 `Box` 和 `From` 特征。理解并不重要
//现在就详细介绍它们，但如果您愿意，可以继续阅读。现在，想想
//`Box<dyn ???>` 类型作为“我想要任何能 ???”的东西类型，其中，
//考虑到 Rust 通常的运行时安全标准，你应该会觉得
//有点宽松！
//
//简而言之，盒子的这个特殊用例是当你想拥有一个盒子时
//值，你只关心它是一个实现特定的类型
//特质。为此，将 Box 声明为 Box<dyn Trait> 类型，其中 Trait 是
//编译器在该上下文中使用的任何值上查找的特征。为此
//练习，该上下文是可以在 a 中返回的潜在错误
//结果。
//
//我们可以用什么来描述这两个错误？换句话说，有没有一个特质
//这两个错误都实现了哪一个？
//
//执行 `rusdlingshinterrors5` 或使用 `hint`watch 子命令
//提示。


use std::error;
use std::fmt;
use std::num::ParseIntError;

// TODO: update the return type of `main()` to make this compile.
fn main() -> Result<(), Box<dyn error::Error>> {
    let pretend_user_input = "42";
    let x: i64 = pretend_user_input.parse()?;
    println!("output={:?}", PositiveNonzeroInteger::new(x)?);
    Ok(())
}

// Don't change anything below this line.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        match value {
            x if x < 0 => Err(CreationError::Negative),
            x if x == 0 => Err(CreationError::Zero),
            x => Ok(PositiveNonzeroInteger(x as u64)),
        }
    }
}

// This is required so that `CreationError` can implement `error::Error`.
impl fmt::Display for CreationError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let description = match *self {
            CreationError::Negative => "number is negative",
            CreationError::Zero => "number is zero",
        };
        f.write_str(description)
    }
}

impl error::Error for CreationError {}
