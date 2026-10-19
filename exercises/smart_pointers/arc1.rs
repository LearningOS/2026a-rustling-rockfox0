//arc1.rs
//
//在本练习中，我们给出了一个名为“numbers”的 u32 Vec，其中包含值
//范围从 0 到 99 --[ 0, 1, 2, ..., 98, 99 ] 我们想使用这个
//同时在 8 个不同线程内的一组数字。每个线程都是
//获取每八个值的总和，并带有偏移量。
//
//第一个线程（偏移量 0）将对 0, 8, 16, ... 求和
//第二个线程（偏移量 1）将对 1, 9, 17, ... 求和
//第三个线程（偏移量 2）将对 2, 10, 18, ... 求和
//...
//第八个线程（偏移量 7）将对 7, 15, 23, ... 求和
//
//因为我们使用线程，所以我们的值需要是线程安全的。  因此，
//我们正在使用 Arc。  我们需要对这两个 TODO 分别进行更改。
//
//通过填写“shared_numbers”的值来编译此代码，其中
//第一个 TODO 注释是，并为 `child_numbers` 创建一个初始绑定
//第二个 TODO 注释在哪里。尽量不要创建任何副本
//`数字` Vec!
//
//执行 `rusdlingshint arc1` 或使用 `hint` watch 子命令来获取提示。

#![forbid(unused_imports)] // Do not change this, (or the next) line.
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let numbers: Vec<_> = (0..100u32).collect();
    let shared_numbers = Arc::new(Mutex::new(numbers)); // TODO
    let mut joinhandles = Vec::new();

    for offset in 0..8 {
        let child_numbers = Arc::clone(&shared_numbers); // TODO
        joinhandles.push(thread::spawn(move || {
            let sum: u32 = child_numbers.lock().unwrap().iter().filter(|&&n| n % 8 == offset).sum();
            println!("Sum of offset {} is {}", offset, sum);
        }));
    }
    for handle in joinhandles.into_iter() {
        handle.join().unwrap();
    }
}
