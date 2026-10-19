//枚举2.rs
//
//执行 `rustlingshint enums2` 或使用 `hint` watch 子命令
//提示。

#[derive(Debug)]
enum Message {
    // TODO: define the different variants used below
     Move{ x:i32, y:i32},
     Echo(String),
     ChangeColor(i32,i32,i32),
     Quit,
    }

impl Message {
    fn call(&self) {
        println!("{:?}", self);
    }
}

fn main() {
    let messages = [
        Message::Move { x: 10, y: 30 },
        Message::Echo(String::from("hello world")),
        Message::ChangeColor(200, 255, 255),
        Message::Quit,
    ];

    for message in &messages {
        message.call();
    }
}
