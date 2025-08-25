#[test]
fn test_binary_heap() {
    use std::collections::BinaryHeap;
    let mut heap = BinaryHeap::from(vec![2, 8, 9, 1, 0, 4, 2, 3, 5]);
    heap.push(7);
    assert_eq!(heap.pop(), Some(9));
    assert_eq!(heap.peek(), Some(&8));
    assert_eq!(heap.len(), 9);

    for &val in heap.iter() {
        println!("{}", val);
    }

    while let Some(val) = heap.pop() {
        println!("{}", val);
    }
}
