//生命周期1.rs
//
//Rust 编译器需要知道如何检查提供的引用是否是
//有效，这样它可以让程序员知道引用是否有风险
//在使用之前超出范围。请记住，参考文献是借用和做的
//不拥有自己的数据。如果他们的主人超出范围怎么办？
//
//执行 `rusdlingshintlifetimes1` 或使用 `hint`watch 子命令
//提示。


fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is '{}'", result);
}
