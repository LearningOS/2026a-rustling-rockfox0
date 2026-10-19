// box1.rs
//
//在编译时，Rust 需要知道一个类型占用了多少空间。这个
//对于递归类型来说会出现问题，其中值可以作为
//本身是同一类型的另一个值。为了解决这个问题，我们可以使用
//`Box` -一个智能指针，用于在堆上存储数据，这也允许我们
//包装递归类型。
//
//我们在本练习中实现的递归类型是 `cons list` -a
//函数式编程语言中常见的数据结构。每个
//cons 列表中的项包含两个元素：当前项的值和
//下一个项目。最后一项是一个名为“Nil”的值。
//
//第 1 步：在枚举定义中使用 `Box` 来使代码编译
//步骤 2：通过替换 `todo!()` 创建空和非空 cons 列表
//
//注意：测试不应更改
//
//执行 `rusdlingshintbox1` 或使用 `hint`watch 子命令来获取提示。
//我还没完成

#[derive(PartialEq, Debug)]
pub enum List {
    Cons(i32, Box<List>),
    Nil,
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list()
    );
}

pub fn create_empty_list() -> List {
    let l:Box<List>=Box::new(List::Nil);
    *l
}

pub fn create_non_empty_list() -> List {
    let l:Box<List>=Box::new(List::Cons(1, Box::new(List::Nil)));
    *l
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(List::Nil, create_empty_list())
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list())
    }
}
