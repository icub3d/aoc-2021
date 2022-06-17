#[derive(Eq, PartialEq, Clone)]
enum Type {
    Amber,
    Bronze,
    Copper,
    Desert,
}

#[derive(Eq, PartialEq, Clone)]
enum State {
    Empty,
    Occupied(Type),
}

#[derive(Eq, PartialEq, Clone)]
struct Node {
    expected: State,
    current: State,
    links: Vec<Node>,
}

fn main() {
    println!("Hello, world!");
}
