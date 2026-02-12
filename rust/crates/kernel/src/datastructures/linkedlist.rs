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
    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem: elem,
            next: core::mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
    pub fn pop(&mut self) -> Option<i32> {
        // TODO
        return 0;
    }
}

/*
*
* so when we pass self, we then pass ownership to
* next which is an instance of Link, so the link reference owns self.head
*   which is also a Link reference
*/
pub fn declare_linked_list() {
    let mut list = List::new();
    list.push(3);
    kprint!("{:?}", list);
}
