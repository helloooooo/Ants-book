use std::collections::LinkedList;

pub fn list() -> Vec<char> {
    let mut ll = LinkedList::new();
    ll.push_front('b');
    ll.push_back('c');
    ll.push_front('a');
    ll.pop_front();
    ll.push_back('d');

    ll.into_iter().collect::<Vec<char>>()
}