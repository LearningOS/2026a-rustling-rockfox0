//特征1.rs
//
//是时候实现一些特征了！你的任务是实现这个特质
//`AppendBar` 用于类型 `String`。 AppendBar 特性只有一个功能，
//将“Bar”附加到实现此特征的任何对象。
//
//执行 `rusdlingshinttraits1` 或使用 `hint`watch 子命令
//提示。
trait AppendBar {
    fn append_bar(self) -> Self;
}

impl AppendBar for String {
    // TODO: Implement `AppendBar` for type `String`.
fn append_bar(self) -> Self {
        let mut s = self;
        s.push_str("Bar");
        s
    }
}

fn main() {
    let s = String::from("Foo");
    let s = s.append_bar();
    println!("s: {}", s);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_foo_bar() {
        assert_eq!(String::from("Foo").append_bar(), String::from("FooBar"));
    }

    #[test]
    fn is_bar_bar() {
        assert_eq!(
            String::from("").append_bar().append_bar(),
            String::from("BarBar")
        );
    }
}
