use data_struct::stack::Stack;
use algorithm::sort::bubble_sort::*;
fn main() {
    println!("Hello");
    test_bubble_sort();
    test_stack();
}

fn test_bubble_sort(){
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    bubble_sort(&mut vec);
    println!("{:?}",vec);
}

fn test_stack() {
    let mut list = Stack::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    assert_eq!(list.is_empty(), false);
    assert_eq!(list.pop(), Ok(5));
    assert_eq!(list.pop(), Ok(4));
    assert_eq!(list.pop(), Ok(3));
    assert_eq!(list.pop(), Err("Stack is empty"));
    assert_eq!(list.is_empty(), true);
}
