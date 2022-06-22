use std::ops::AddAssign;

pub struct ListNode<T> {
    value: T,
    next: Option<Box<ListNode<T>>>,
}

impl<T> ListNode<T> {
    pub fn new(val: T) -> Self {
        ListNode {
            value: val,
            next: None,
        }
    }

    pub fn eat(mut self, node: Self) -> Self {
        self.next = Some(Box::new(node));
        return self;
    }

    pub fn new_from_eat(val: T, node: Self) -> Self {
        Self::new(val).eat(node)
    }

    pub fn append_to(val: T, node: Self) -> Self {
        node.eat(Self::new(val))
    }

    pub fn from_vec(mut vec: Vec<T>) -> Option<Self> {
        let mut popped = vec.pop();
        let mut head: Option<ListNode<T>>;
        match popped {
            Some(l) => {
                head = Some(Self::new(l));
                popped = vec.pop();
                while popped.is_some() {
                    head = Some(ListNode {
                        value: popped.expect("last is None!"),
                        next: Some(Box::new(head.expect("head is None!"))),
                    });
                    popped = vec.pop();
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
