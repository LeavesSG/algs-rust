use std::ops::AddAssign;

pub struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        {
            ListNode {
                value: val,
                next: None,
            }
        }
    }

    pub fn from(mut vec: Vec<T>) -> Option<ListNode<T>> {
        let mut last = vec.pop();
        let mut head: Option<ListNode<T>>;
        match last {
            Some(l) => {
                head = Some(ListNode {
                    value: l,
                    next: None,
                });
                last = vec.pop();
                while last.is_some() {
                    head = Some(ListNode {
                        value: last.expect("last is None!"),
                        next: Some(Box::new(head.expect("head is None!"))),
                    });
                    last = vec.pop();
                }

                return head;
            }
            None => return None,
        }
    }

    pub fn len(&self) -> usize {
        let mut pointer = self;
        let mut len = 1;
        while pointer.next.is_some() {
            len += 1;
            pointer = &pointer.next.as_ref().expect("")
        }
        return len;
    }

    pub fn to_vec(self) -> Vec<T> {
        let mut pointer = self;
        let mut value = pointer.value;
        let mut vec = Vec::from([value]);
        while pointer.next.is_some() {
            pointer = *pointer.next.expect("");
            value = pointer.value;
            vec.push(value)
        }
        return vec;
    }
}

impl<T: AddAssign + Copy> ListNode<T> {
    pub fn get_sum(&self) -> T {
        let mut pointer = self;
        let mut counter = pointer.value;
        while pointer.next.is_some() {
            counter += pointer.next.as_ref().expect("").value;
            pointer = pointer.next.as_ref().expect("");
        }
        return counter;
    }
}
