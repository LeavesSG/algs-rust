mod list_node;
use list_node::ListNode;

pub fn test() {
    let a = ListNode::from([1, 2, 3, 4, 5, 6, 7, 8, 9, 10].to_vec());
    println!("{:?}", a.expect("").to_vec())
}
