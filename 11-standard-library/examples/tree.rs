use standard_library::tree::Node;

fn main() {
    let tree = Node::new(1);
    tree.add_child(Node::new(3));
    tree.add_child(Node::new(5));
    println!("Корень: {}, дети: {:?}", tree.value(), tree.child_values());
    println!("Finishing program now");
}
