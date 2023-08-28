use data_struct::stack::{LinkStack, VecStack};
use algorithm::sort::bubble_sort::*;
use data_struct::queue::Queue;

fn main() {
    println!("Hello");
    // test_bubble_sort();
    // test_vec_stack();
    // test_link_stack();
    test_queue();
}

fn test_bubble_sort(){
    let mut vec = vec![7, 49, 73, 58, 30, 72, 44, 78, 23, 9];
    bubble_sort(&mut vec);
    println!("{:?}",vec);
}

fn test_vec_stack() {
    let mut list = VecStack::new();
    list.push(1);
    list.push(2);
    list.push(3);
    list.push(4);
    list.push(5);
    println!("{:?}",list);
}

fn test_link_stack(){
    let mut list = LinkStack::new();
    list.push(1);
    list.push(3);
    list.push(5);
    list.push(9);
    println!("list empty:{}",list.is_empty());
    for (i,v) in list.iter().enumerate(){
        println!("No.{i}:{v}");
    }
    println!("list peek:{:?}",list.peek());

}

fn test_queue() {
    let mut queue: Queue<u8> = Queue::new();
    queue.enqueue(32);
    queue.enqueue(64);
    println!("{:?}",queue.clone());
    let retrieved_dequeue = queue.dequeue();
    println!("{:?}",retrieved_dequeue);
}

