// iterators1.rs
//
// 通过填写 `???` 使我编译
//
// 当对集合中的元素执行操作时，迭代器是必不可少的。
// 本模块帮助你熟悉使用迭代器的结构和如何
// 遍历可迭代集合中的元素。
//
// 执行 `rustlings hint iterators1` 或在观察模式下使用 `hint` 子命令来获取提示。

fn main () {
    let my_fav_fruits = vec!["banana", "custard apple", "avocado", "peach", "raspberry"];

    let mut my_iterable_fav_fruits = my_fav_fruits.iter();   // TODO: 步骤 1

    assert_eq!(my_iterable_fav_fruits.next(), Some(&"banana"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"custard apple"));     // TODO: 步骤 2
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"avocado"));
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"peach"));     // TODO: 步骤 3
    assert_eq!(my_iterable_fav_fruits.next(), Some(&"raspberry"));
    assert_eq!(my_iterable_fav_fruits.next(), None);     // TODO: 步骤 4
}
