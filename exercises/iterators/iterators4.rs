// iterators4.rs
//
// Execute `rustlings hint iterators4` or use the `hint` watch subcommand for a
// hint.


pub fn factorial(num: u64) -> u64 {
//完成此函数以返回 num 的阶乘
    //不要使用：
    //-返回
    //尽量不要使用：
    //-命令式循环（for、while）
    //-附加变量
    //对于额外的挑战，不要使用：
    //-递归
    //执行 `rusdlingshintiterators4` 以获得提示。
    let result = (1..=num).product();
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn factorial_of_0() {
        assert_eq!(1, factorial(0));
    }

    #[test]
    fn factorial_of_1() {
        assert_eq!(1, factorial(1));
    }
    #[test]
    fn factorial_of_2() {
        assert_eq!(2, factorial(2));
    }

    #[test]
    fn factorial_of_4() {
        assert_eq!(24, factorial(4));
    }
}
