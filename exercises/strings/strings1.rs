//字符串1.rs
//
//让我在不改变函数签名的情况下编译！
//
//执行 `rustlingshintstrings1` 或使用 `hint`watch 子命令
//提示。

fn main() {
    let answer = current_favorite_color();
    println!("My current favorite color is {}", answer);
}

fn current_favorite_color() -> String {
    "blue".to_string() // TODO: Return a String instead of a &str
}
