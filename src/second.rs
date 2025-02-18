
pub struct List<T> {
    head: Link<T>,
}

type Link<T> = Option<Box<Node<T>>>;

struct Node<T> {
    elem: T,
    next: Link<T>,
}

pub struct IntoIter<T>(List<T>);

pub struct Iter<'a, T> {
    next: Option<&'a Node<T>>,
}
impl<'a, T> List<T> {
    pub fn new() -> Self {
        List { head: None }
    }
    pub fn iter(&'a self) -> Iter<'a, T> {
        Iter {next: self.head.map(|node| &'a node) }
    }
    pub fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    pub fn push(&mut self, elem: T) {
        let new_node = Box::new(Node {
            elem: elem,
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
impl<T> Iterator for IntoIter<T> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.0.pop()
    }
}
impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &T;
    fn next(&'a mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.map(|node| &'a node);
            &'a node.elem
        })
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
    #[test]
    fn peek_mut() {
        let mut list = List::new();
        assert_eq!(list.peek_mut(), None);
        list.push(1); list.push(2); list.push(3);
        assert_eq!(list.peek_mut(), Some(&mut 3));
        list.peek_mut().map(|value| {
            *value = 42
        });

    }
    #[test]
    fn peek() {
        let mut list = List::new();
        assert_eq!(list.peek(), None);
        list.push(1); list.push(2); list.push(3);
    
        assert_eq!(list.peek(), Some(&3));
    
        assert_eq!(list.peek(), Some(&3));
        assert_eq!(list.pop(), Some(3));
    }
    
    #[test]
    fn into_iter() {
        let mut list = List::new();
        list.push(1); list.push(2); list.push(3);
    
        let mut iter = list.into_iter();
        assert_eq!(iter.next(), Some(3));
        assert_eq!(iter.next(), Some(2));
        assert_eq!(iter.next(), Some(1));
        assert_eq!(iter.next(), None);
    }
}
