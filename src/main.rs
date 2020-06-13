mod stack;
use stack::Stack;

fn main() {
    let mut stack = Stack::new();
    println!("Size : {}", stack.size());
    
    stack.add(1);
    stack.add(2);
    stack.add(3);
    
    println!("Size : {}", stack.size());
    println!("Item : {:?}", stack.remove());
    println!("Item : {:?}", stack.remove());
    println!("Item : {:?}", stack.remove());
    
    println!("Peeking {:?}", stack.peek());
    println!("Size : {}", stack.size());
}
