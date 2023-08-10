pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

#[derive(Debug, PartialEq)]
struct Node<T> {
    elem: T,
    next: Link<T>,
}

impl<T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem,
            next: self.head.take(),
        });

        self.head = Some(new_node);
    }

    pub fn pop(&mut self) -> Option<T> {
        self.head.take().map(|node| {
            self.head = node.next;
            node.elem
        })
    }

    pub fn peek(&self) -> Option<&T> {
        self.head.as_ref().map(|node| {
            &node.elem
        })
    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {
        self.head.as_mut().map(|node| {
            &mut node.elem
        })
    }
}

impl<T> Drop for List<T> {
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
        let list: List<i32> = List::new();

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

    #[test]
    fn it_peeks_the_elem_in_the_head_of_list() {
        let mut list = List::new();
        list.push(42);

        let elem = list.peek();

        assert_eq!(elem, Some(&42));
    }

    #[test]
    fn it_mutates_the_elem_in_the_head_of_list() {
        let mut list = List::new();
        list.push(42);

        let elem = list.peek_mut();

        assert_eq!(elem, Some(&mut 42));

        list.peek_mut().map(|value| {
            *value = 69
        });

        assert_eq!(list.peek(), Some(&69));
    }
}
