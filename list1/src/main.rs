#![allow(unused)]


/// 链表数字顺序从小到大
/// push(3)
/// 3
/// push(1)
/// 1 -> 3
/// push(2)
/// 1 -> 2 -> 3
/// pop(2)
/// 1 -> 3

pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Clone)]
struct Node {
    val: i32,
    next: Link,
}

impl List {
    fn new() -> Self {
        todo!()
    }

    // insert a value
    // 如果有相同的数字，仍然插入
    fn push(&self, v: i32) {

    }

    // pop 有这个数字 就返回 Some(i32)
    // 没有 就返回None
    // 如果有相同的数字，就删除一个就好了
    fn pop(&self, v: i32) -> Option<i32> {
        todo!()
    }
}

// O(1)
// Iter, IterMut, IntoIterator
// 顺序是从小的数字到大的数字输出


fn main() {}