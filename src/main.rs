mod queue;
use queue::Queue;

fn main() {
    let items = vec![1,2,3,4,5];
    let mut queue = Queue::from_vec(items);
    
    // let mut queue:Queue<i32> = Queue::new();
    // queue.enqueue(5);
    // queue.enqueue(6);
    
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.size());
    
    queue.enqueue(10);
    queue.enqueue(20);
    
    
    
    println!("{}", queue.size());
    println!("{}", queue.dequeue().unwrap());
    println!("{}", queue.peek().unwrap());
    
}
