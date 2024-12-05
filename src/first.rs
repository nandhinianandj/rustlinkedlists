// in first.rs
struct Node {
    elem: i32,
    next: List,
}

// pub says we want people outside this module to be able to use List
pub enum List {
    Empty,
    More(Box<Node>),
}

