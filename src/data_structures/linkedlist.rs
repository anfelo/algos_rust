use std::mem;

pub struct List {
    head: Link,
}

#[derive(Debug, PartialEq)]
enum Link {
    Empty,
    More(Box<Node>),
}

#[derive(Debug, PartialEq)]
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
            elem,
            next: mem::replace(&mut self.head, Link::Empty),
        });

        self.head = Link::More(new_node);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_empty_list() {
        let list = List::new();

        assert_eq!(list.head, Link::Empty);
    }

    #[test]
    fn it_appends_elem_to_list() {
        let mut list = List::new();

        list.push(42);

        assert_eq!(
            list.head,
            Link::More(Box::new(Node {
                elem: 42,
                next: Link::Empty
            }))
        );
    }
}
