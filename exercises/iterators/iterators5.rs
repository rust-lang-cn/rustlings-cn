// iterators5.rs
// 让我们定义一个简单的模块来追踪 Rustlings 练习的进度。
// 进度会使用哈希表组织。键是练习的名字，值是进度。
// 两个计数函数被创建来数给定进度的练习数。
// 这两个计数函数使用命令式的循环风格。
// 使用迭代器重写这个计数功能。
// 只需要改变两个迭代器方法 (count_iterator 和 count_collection_iterator)
// 执行 `rustlings hint iterators5` 或在观察模式下使用 `hint` 子命令来获取提示。
//
// 让代码可以编译，测试可以通过。

// I AM NOT DONE

use std::collections::HashMap;

#[derive(Clone, Copy, PartialEq, Eq)]
enum Progress {
    None,
    Some,
    Complete,
}

fn count_for(map: &HashMap<String, Progress>, value: Progress) -> usize {
    let mut count = 0;
    for val in map.values() {
        if val == &value {
            count += 1;
        }
    }
    count
}

fn count_iterator(map: &HashMap<String, Progress>, value: Progress) -> usize {
    // map是一个有 String 键和 Progress 值的哈希表。
    // map = { "variables1": Complete, "from_str": None, ... }
    todo!();
}

fn count_collection_for(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    let mut count = 0;
    for map in collection {
        for val in map.values() {
            if val == &value {
                count += 1;
            }
        }
    }
    count
}

fn count_collection_iterator(collection: &[HashMap<String, Progress>], value: Progress) -> usize {
    // collection 是哈希表的切片。
    // collection = [{ "variables1": Complete, "from_str": None, ... },
    //     { "variables2": Complete, ... }, ... ]
    todo!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn count_complete() {
        let map = get_map();
        assert_eq!(3, count_iterator(&map, Progress::Complete));
    }

    #[test]
    fn count_equals_for() {
        let map = get_map();
        assert_eq!(
            count_for(&map, Progress::Complete),
            count_iterator(&map, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_complete() {
        let collection = get_vec_map();
        assert_eq!(
            6,
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    #[test]
    fn count_collection_equals_for() {
        let collection = get_vec_map();
        assert_eq!(
            count_collection_for(&collection, Progress::Complete),
            count_collection_iterator(&collection, Progress::Complete)
        );
    }

    fn get_map() -> HashMap<String, Progress> {
        use Progress::*;

        let mut map = HashMap::new();
        map.insert(String::from("variables1"), Complete);
        map.insert(String::from("functions1"), Complete);
        map.insert(String::from("hashmap1"), Complete);
        map.insert(String::from("arc1"), Some);
        map.insert(String::from("as_ref_mut"), None);
        map.insert(String::from("from_str"), None);

        map
    }

    fn get_vec_map() -> Vec<HashMap<String, Progress>> {
        use Progress::*;

        let map = get_map();

        let mut other = HashMap::new();
        other.insert(String::from("variables2"), Complete);
        other.insert(String::from("functions2"), Complete);
        other.insert(String::from("if1"), Complete);
        other.insert(String::from("from_into"), None);
        other.insert(String::from("try_from_into"), None);

        vec![map, other]
    }
}
