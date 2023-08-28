#![allow(dead_code)]
// #![allow(unused_variables)]
use data_struct::stack::Stack;
use algorithm::sort::bubble_sort::*;
use data_struct::linked_list::LinkedList;
use data_struct::queue::Queue;

fn main() {
    println!("Hello");
    // test_bubble_sort();
    // test_stack();
    // test_link_stack();
    //test_queue();
    test_list();
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
    println!("{:?}",list);
}


fn test_queue() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(32);
    queue.enqueue(64);
    println!("{:?}",queue.clone());
    let retrieved_dequeue = queue.dequeue();
    println!("{:?}",retrieved_dequeue);
}

fn test_list() {
    let mut list_str = LinkedList::<String>::new();
    list_str.add("A".to_string());
    list_str.add("B".to_string());
    list_str.add("B".to_string());
    list_str.add("B".to_string());
    println!("Linked List :  {}", list_str);
    let item0 = list_str.get(0).unwrap();
    println!("index0 : {}", item0);
    let item1 = list_str.get(1).unwrap();
    println!("index1 : {}", item1);
}
