use crate::kprint;
use alloc::boxed::Box;

#[derive(Debug)]
pub struct List {
    head: Link,
}
#[derive(Debug)]
enum Link {
    Empty,
    More(Box<Node>),
}
#[derive(Debug)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Empty }
    }
    pub fn push(self, elem: i32) {
        let new_node = Node {
            elem: elem,
            next: self.head,
        };
    }
}

/*
*
* so when we pass self, we then pass ownership to
* next which is an instance of Link, so the link reference owns self.head
*   which is also a Link reference
*/
pub fn declare_linked_list() {
    let list = List::new();
    list.push(3);
    kprint!("{:?}", list);
}
