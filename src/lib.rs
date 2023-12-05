use std::cmp::PartialOrd;
use std::fmt::Display;

pub trait NodeInner: PartialOrd + Copy + Display {}

// Nightly Rust doesn't need this afaik; 
// it kinda defeats the purpose of traits when I have to name each type anyways.
impl NodeInner for i8 {}
impl NodeInner for i16 {}
impl NodeInner for i32 {}
impl NodeInner for i64 {}
impl NodeInner for i128 {}
impl NodeInner for isize {}

impl NodeInner for u8 {}
impl NodeInner for u16 {}
impl NodeInner for u32 {}
impl NodeInner for u64 {}
impl NodeInner for u128 {}
impl NodeInner for usize {}

impl NodeInner for f32 {}
impl NodeInner for f64 {}

impl NodeInner for char {}
impl NodeInner for bool {}

/// A node of a linked list. Type `T` must implement basic ordering traits (`<`, `>`, `==`, etc.)
#[derive(Debug, PartialEq)]
pub struct Node<T: NodeInner> {
    /// The value of the node.
    pub value: T,
    /// An optional pointer to the next node in the list.
    pub next: Option<Box<Node<T>>>,
}

impl<T: NodeInner> Node<T> {
    /// Returns a new node with no `next` field and a given `value`.
    ///
    /// # Arguments
    ///
    /// * `value` - The value of the head node.
    #[inline]
    pub fn new(value: T) -> Self {
        Node {
            next: None,
            value,
        }
    }

    /// This iteratively prints all contents of a linked list from the given node.
    ///
    /// Please note that if a node contains a link loop, this will run until
    /// your memory is exhausted, presumably.
    ///
    /// # Examples
    ///
    /// ```
    /// use llist::Node;
    /// let head = Node::new(0);
    /// head.print_all(); // This will print `0` to console.
    /// ```
    pub fn print_all(&self) {
        while let Some(c) = &self.next {
            println!("{}", c.value);
        }
    }
}

/// Generates a linked list from a vector of i32 files. May make generic to copyable types in the future.
fn vec_to_list<T: NodeInner>(input: &Vec<T>) -> Option<Box<Node<T>>> {
    let mut current: Option<Box<Node<T>>> = None;

    for &val in input.iter().rev() {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = current.take();
        current = Some(new_node);
    }

    current
}

/// Generates a linked list from a vector of T files. May make generic to copyable types in the future.
fn vec_into_list<T: Copy + NodeInner>(input: Vec<T>) -> Option<Box<Node<T>>> {
    let mut current: Option<Box<Node<T>>> = None;

    for &val in input.iter().rev() {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = current.take();
        current = Some(new_node);
    }

    current
}

pub fn merge_lists <T: NodeInner> (list1: Option<Box<Node<T>>>, list2: Option<Box<Node<T>>>)
-> Option<Box<Node<T>>> {
    unimplemented!();
}

#[cfg(test)]
mod tests {
    use crate::{vec_into_list, merge_lists};
    
    #[test]
    fn test_merging() {
        /* Test Case 1 */
        let list_1n = 
            vec_into_list(vec![1, 2, 4]);
    // Written do
        let list_1m =
            vec_into_list(vec![1, 3, 4]);
        let out_1 = merge_lists(list_1n, list_1m);
        assert_eq!(out_1, vec_into_list(vec![1, 1, 2, 3, 4, 4]));

        /* Test Case 2 */
        let list_2n =
            vec_into_list::<i32>(vec![]); // Empty vecs need explicit typing
        let list_2m =    
            vec_into_list::<i32>(vec![]);
        let out_2 = merge_lists(list_2n, list_2m);
        assert_eq!(out_2, vec_into_list(vec![]));

        /* Test Case 3 */
        let list_3n =
            vec_into_list::<i32>(vec![]);
        let list_3m =
            vec_into_list(vec![0]);
        let out_3 = merge_lists(list_3n, list_3m);
        assert_eq!(out_3, vec_into_list(vec![0]));
    }
}
