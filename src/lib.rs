use std::cmp::PartialOrd;
use std::fmt::Display;
use std::mem::replace;

pub trait NodeInner: PartialOrd + Copy + Display {}

// Nightly Rust doesn't need this afaik; 
// it kinda defeats the purpose of traits when I have to name each type it encompasses.
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
#[derive(Debug, PartialEq, Eq, Clone)]
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

    /// This recursively prints all contents of a linked list from the given node.
    ///
    /// Please note that if a node contains a link loop, this will run until
    /// your memory is exhausted, presumably.
    ///
    /// # Example
    ///
    /// ```
    /// use llist::Node;
    /// let mut head = Node::new(0);
    /// head.print_all(); // This will print `0` to console.
    ///
    /// head.next = Some(Box::new(Node::new(1)));
    /// head.print_all(); // This will now print `0` followed by `1`.
    /// ```
    pub fn print_all(&self) {
        print_recursive(Some(Box::new(self.clone())))
    }


    /// Returns the middle node of the list from the head node to the end of the list. Due to using
    /// a two-pointer method, this rounds down when the length of the list is odd.
    pub fn middle_node(self) -> Option<Box<Node<T>>> {
        let mut slow = &Some(Box::new(self));
        let mut fast = &slow.clone();
        
        while fast.is_some() && fast.as_ref().unwrap().next.is_some() {
            slow = &slow.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next.as_ref().unwrap().next;
        }

        slow.clone()
    }
} 

fn print_recursive<T: NodeInner>(scope: Option<Box<Node<T>>>) {
    match scope {
        Some(c) => {
            println!("{}", c.value);
            print_recursive(c.next);
        },
        _ => {}
    }
}

/// Generates a linked list from a vector of type T.
///
/// # Arguments
///
/// * `input` - A Vec of type T, where T is any primitive type.
pub fn vec_to_list<T: NodeInner>(input: &Vec<T>) -> Option<Box<Node<T>>> {
    let mut current: Option<Box<Node<T>>> = None;

    for &val in input.iter().rev() {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = current.take();
        current = Some(new_node);
    }

    current
}

/// Generates a linked list from a vector of type T, consuming the vector in the process. For a
/// nondestructive version, see `vec_to_list()`.
///
/// # Arguments
///
/// * `input` - A Vec of type T, where T is any primitive type.
pub fn vec_into_list<T: NodeInner>(input: Vec<T>) -> Option<Box<Node<T>>> {
    let mut current: Option<Box<Node<T>>> = None;

    for val in input.into_iter().rev() {
        let mut new_node = Box::new(Node::new(val));
        new_node.next = current.take();
        current = Some(new_node);
    }

    current
}

/// Merges two sorted linked lists together recursively.
/// 
/// # Arguments
///
/// * `list1` - the first list to be merged.
/// * `list2` - the second list to be merged.
///
/// Please note that a "list" is in fact an `Option<Box<Node<T>>>` and not just a `Node`.
///
/// # Example
///
/// ```
/// let list_1n = 
///     vec_into_list(vec![1, 2, 4]);
/// let list_1m =
///     vec_into_list(vec![1, 3, 4]);
/// let out_1 = merge_sorted_lists(list_1n, list_1m);
/// assert_eq!(out_1, vec_into_list(vec![1, 1, 2, 3, 4, 4]));
/// ```
pub fn merge_sorted_lists <T: NodeInner> (list1: Option<Box<Node<T>>>, list2: Option<Box<Node<T>>>)
-> Option<Box<Node<T>>> {
    match (list1, list2) {
        (None, None) => None,
        (Some(n), None) | (None, Some(n)) => Some(n),
        (Some(list1), Some(list2)) => {
            if list1.value < list2.value {
                Some(Box::new(Node {
                    value: list1.value,
                    next: merge_sorted_lists(list1.next, Some(list2)),
                }))
            } else {
                Some(Box::new(Node {
                    value: list2.value,
                    next: merge_sorted_lists(Some(list1), list2.next)
                }))
            }
        }
    }
}

/// This reverses the list from the `head` node via memory replacement.
///
/// # Arguments
///
/// * `head` - the head of the linked list.
pub fn reverse<T: NodeInner>(mut head: Option<Box<Node<T>>>) -> Option<Box<Node<T>>> {
    let mut prev = None;
    while let Some(mut node) = head.take() {
        head = replace(&mut node.next, prev);
        prev = Some(node);
    }
    prev
}

