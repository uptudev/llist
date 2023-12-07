use llist::{vec_into_list, merge_sorted_lists};

/* Test Case 1 */
#[test]
fn test_merge_standard() {
    let list_1n = 
        vec_into_list(vec![1, 2, 4]);
    let list_1m =
        vec_into_list(vec![1, 3, 4]);
    let out_1 = merge_sorted_lists(list_1n, list_1m);
    assert_eq!(out_1, vec_into_list(vec![1, 1, 2, 3, 4, 4]));
}

/* Test Case 2 */
#[test]
fn test_merge_both_empty() {
    let list_2n =
        vec_into_list::<i32>(vec![]); // Empty vecs need explicit typing
    let list_2m =    
        vec_into_list::<i32>(vec![]);
    let out_2 = merge_sorted_lists(list_2n, list_2m);
    assert_eq!(out_2, vec_into_list(vec![]));
}

/* Test Case 3 */
#[test]
fn test_merge_one_empty() {
    let list_3n =
        vec_into_list::<i32>(vec![]);
    let list_3m =
        vec_into_list(vec![0]);
    let out_3 = merge_sorted_lists(list_3n, list_3m);
    assert_eq!(out_3, vec_into_list(vec![0]));
}

/* Test Case 4 */
#[test]
fn test_merge_order_long() {
    let list4n =
        vec_into_list(vec![0, 1, 2, 3, 4, 5, 6, 7, 8]);
    let list4m =
        vec_into_list(vec![0, 2, 4, 6, 8, 10, 12, 14, 16]);
    let out_4 = merge_sorted_lists(list4n, list4m);
    assert_eq!(out_4, vec_into_list(vec![
        0, 0, 1, 2, 2, 3, 4, 4, 5, 6, 6, 7, 8, 8, 10, 12, 14, 16
    ]));
}
