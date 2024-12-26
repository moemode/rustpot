pub struct List {
    head: Link,
}

struct Node {
    value: i32,
    next: Link,
}

enum Link {
    None,
    Some(Box<Node>),
}

impl List {
    fn new() -> Self {
        return List { head: Link::None };
    }

    fn push(&mut self, value: i32) {
        let new_node = Box::new(Node {
            value,
            next: std::mem::replace(&mut self.head, Link::None),
        });
        self.head = Link::Some(new_node);
    }

    fn pop(&mut self) -> Option<i32> {
        let first = std::mem::replace(&mut self.head, Link::None);
        match first {
            Link::None => None,
            Link::Some(node) => {
                self.head = node.next;
                Some(node.value)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut curr = std::mem::replace(&mut self.head, Link::None);
        while let Link::Some(mut boxed) = curr {
            curr = std::mem::replace(&mut boxed.next, Link::None);
        }
    }
}

#[cfg(test)]
mod test {
    use super::List;

    #[test]
    fn basics() {
        let mut list = List::new();

        // Check empty list behaves right
        assert_eq!(list.pop(), None);

        // Populate list
        list.push(1);
        list.push(2);
        list.push(3);

        // Check normal removal
        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        // Push some more just to make sure nothing's corrupted
        list.push(4);
        list.push(5);

        // Check normal removal
        assert_eq!(list.pop(), Some(5));
        assert_eq!(list.pop(), Some(4));

        // Check exhaustion
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);
    }
}