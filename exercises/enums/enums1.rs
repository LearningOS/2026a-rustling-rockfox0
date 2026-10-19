// enums1.rs
//
// No hints this time! ;)


#[derive(Debug)]
enum Message {
    //TODO: 定义几种类型的消息，如下所示
    Quit,
    Echo,
    Move,
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Quit);
    println!("{:?}", Message::Echo);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::ChangeColor);
}
