pub struct List {
    head: Link,
}

type Link = Option<Box<Node>>;

#[derive(Debug, PartialEq)]
struct Node {
    elem: i32,
    next: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: i32) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = self.head.take();

        while let Some(mut boxed_node) = cur_link {
            cur_link = boxed_node.next.take();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_creates_empty_list() {
        let list = List::new();

        assert_eq!(list.head, None);
    }

    #[test]
    fn it_appends_elem_to_list() {
        let mut list = List::new();

        list.push(42);

        assert_eq!(
            list.head,
            Some(Box::new(Node {
                elem: 42,
                next: None
            }))
        );
    }

    #[test]
    fn it_pops_the_first_elem_from_list() {
        let mut list = List::new();
        list.push(42);

        let elem1 = list.pop();

        assert_eq!(list.head, None);
        assert_eq!(elem1, Some(42));

        let elem2 = list.pop();

        assert_eq!(elem2, None);
    }
}
