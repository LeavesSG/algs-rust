pub struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    fn len(&self) -> usize {
        let mut pointer = self;
        let mut len = 0;
        while pointer.next.is_some() {
            len += 1;
        }
        return len;
    }
}
