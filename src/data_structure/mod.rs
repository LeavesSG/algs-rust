mod list_node;
use list_node::ListNode;

pub fn test() {
    let a = ListNode::from_vec([1, 2, 3, 4, 5].to_vec());
    let b = ListNode::new_from_eat(6, a.expect(""));
    println!("{:?}", b.to_vec());
}
