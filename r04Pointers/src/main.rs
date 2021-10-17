
fn main() {
    println!("Referencias");

    // reference counter: muchos dueños, contador de referencias, String y Vec<T>
    
    // Smart pointers son usados implmentados usando structs
    // implmenta los traits Deref y Drop
    
    // Box<T>
    let x:i32 = 2;
    let y = Box::new(2);

    println!("y = {}", y);

    // Linked lists
    enum List {
        Node(i32, Box<List>),
        None,
    }

    let node3 = List::Node(10, Box::new(List::None));
    let node2 = List::Node(10, Box::new(node3));
    let node1 = List::Node(10, Box::new(node2));


}
