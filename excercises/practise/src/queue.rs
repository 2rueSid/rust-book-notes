#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

#[derive(Debug)]
pub struct Queue<T> {
    head: Option<Box<Node<T>>>,
    tail: Option<Box<Node<T>>>,
    len: usize,
}

impl<T> Queue<T> {
    pub fn new() -> Queue<T> {
        Queue {
            head: None,
            tail: None,
            len: 0,
        }
    }

    pub fn enqueue(&mut self, value: T) {
        let new_tail = Box::new(Node { value, next: None });

        self.len += 1;
        match self.tail.take() {
            None => {
                self.head = Some(new_tail);
                self.tail = self.head.as_mut().map(|node| node.as_mut());
            }
            Some(mut old_tail) => {
                old_tail.next = Some(new_tail);
                self.tail = Some(old_tail);
            }
        }
    }

    pub fn dequeue(&mut self) -> Option<T> {
        match self.head.take() {
            None => None,
            Some(n) => {
                self.len -= 1;
                self.head = n.next;

                Some(n.value)
            }
        }
    }

    pub fn peek(&self) -> Option<&T> {
        match &self.head {
            None => None,
            Some(n) => Some(&n.value),
        }
    }

    pub fn len(&self) -> usize {
        self.len
    }
}
