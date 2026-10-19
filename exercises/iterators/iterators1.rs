// iterators1.rs
//
//当对集合中的元素执行操作时，迭代器
//必需的。该模块帮助您熟悉使用
//迭代器以及如何遍历可迭代集合中的元素。
//
//通过填写 `???` 来让我编译
//
//执行 `rustlingshintiterators1` 或使用 `hint`watch 子命令
//提示。


fn main() {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();  // TODO: Step 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));  // TODO: Step 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: Step 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: Step 4
}
