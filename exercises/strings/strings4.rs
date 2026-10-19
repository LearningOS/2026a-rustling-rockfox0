// strings4.rs
////好吧，这里有一堆值——有些是 `String`，有些是 `&str`。你的
//任务是根据不同的值对每个值调用这两个函数之一
//你认为每个值都是。也就是说，添加 `string_slice` 或 `string`
//每行括号之前。如果你是对的，它就会编译！
//
// No hints this time!


fn string_slice(arg: &str) {
    println!("{}", arg);
}
fn string(arg: String) {
    println!("{}", arg);
}

fn main() {
    string_slice("blue");
    string("red".to_string());
    string(String::from("hi"));
    string("rust is fun!".to_owned());
    string("nice weather".into());
    string(format!("Interpolation {}", "Station"));
    string_slice(&String::from("abc")[0..1]);
    string_slice("  hello there ".trim());
    string("Happy Monday!".to_string().replace("Mon", "Tues"));
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
