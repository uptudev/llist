# llist

`llist` is a Rust library for working with linked lists. It provides a simple `Node` structure and functions to perform common operations on linked lists, such as printing all elements, finding the middle node, converting a vector to a linked list, merging sorted linked lists, and reversing a linked list.

## Usage

### Node

A `Node` represents an element in the linked list, containing a value and an optional pointer to the next node.

```rust
use llist::Node;

let mut head = Node::new(0);
head.next = Some(Box::new(Node::new(1)));

// Print all elements in the linked list
head.print_all();

// Find the middle node of the linked list
let middle = head.middle_node();
```

### Conversion from Vec to Linked List

The library provides two functions to convert a vector into a linked list: `vec_to_list` and `vec_into_list`. The former takes a reference to the vector, while the latter consumes the vector.

```rust
use llist::{vec_to_list, vec_into_list};

let vector = vec![1, 2, 3];
let linked_list_ref = vec_to_list(&vector);
let linked_list_owned = vec_into_list(vector);
```

### Merging Sorted Linked Lists

You can merge two sorted linked lists using the `merge_sorted_lists` function.

```rust
use llist::{vec_into_list, merge_sorted_lists};

let list1 = vec_into_list(vec![1, 2, 4]);
let list2 = vec_into_list(vec![1, 3, 4]);
let merged_list = merge_sorted_lists(list1, list2);
```

### Reversing a Linked List

To reverse a linked list, use the `reverse` function.

```rust
use llist::{vec_into_list, reverse};

let original_list = vec_into_list(vec![1, 2, 3]);
let reversed_list = reverse(original_list);
```

## Examples

Check the examples provided in the code comments for detailed usage examples of each function.

## Contribution

Feel free to contribute to the development of this library by opening issues or submitting pull requests on [GitHub](https://github.com/uptudev/llist).

## License

This library is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

