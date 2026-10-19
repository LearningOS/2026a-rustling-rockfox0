// tests5.rs
//
//Rust 中的“不安全”充当合约。
//
//当在项声明上标记“不安全”时，例如函数，
//一个特质等等，它旁边声明了一个契约。然而，
//合约的内容不能仅用单个关键字来表达。
//因此，您有责任在`# Safety`中手动声明它
//文档中对该项目的评论部分。
//
//当大括号括起来的代码块上标记了“不安全”时，
//它声明遵守某些合同，例如某些合同的有效性
//指针参数，某个内存地址的所有权。然而，就像
//上面的文字，你还需要说明如何遵守合约
//代码块的注释。
//
//注意：所有注释都是为了可读性和可维护性
//你的代码，而 Rust 编译器则信任你的代码的健全性
//给自己写代码！如果你不能证明内存的安全性和健全性
//你自己的代码，退后一步并使用安全代码！
//
//执行 `rustlingshinttests5` 或使用 `hint`watch 子命令
//提示。

// I AM NOT DONE

/// # Safety
///
/// The `address` must contain a mutable reference to a valid `u32` value.
unsafe fn modify_by_address(address: usize) {//TODO: 在下面的代码块中填写您的安全注意事项以匹配您的
    //代码的行为和该函数的契约。您可以使用
    //下面测试的注释作为您的格式参考。
    unsafe {
        let ptr=address as *mut u32;
        *ptr = 0xAABBCCDD;

    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        let mut t: u32 = 0x12345678;
        // SAFETY: The address is guaranteed to be valid and contains
        // a unique reference to a `u32` local variable.
        unsafe { modify_by_address(&mut t as *mut u32 as usize) };
        assert!(t == 0xAABBCCDD);
    }
}
