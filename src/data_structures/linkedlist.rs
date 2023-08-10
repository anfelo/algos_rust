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

    pub fn pop(&mut self) -> Option<i32> {
        match mem::replace(&mut self.head, Link::Empty) {
            Link::Empty => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Empty);

        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Empty);
        }
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

    #[test]
    fn it_pops_the_first_elem_from_list() {
        let mut list = List::new();
        list.push(42);

        let elem1 = list.pop();

        assert_eq!(list.head, Link::Empty);
        assert_eq!(elem1, Some(42));

        let elem2 = list.pop();

        assert_eq!(elem2, None);
    }
}
