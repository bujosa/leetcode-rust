#![allow(dead_code)]
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub neighbors: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32, neighbors: Vec<Rc<RefCell<Node>>>) -> Self {
        Node { val, neighbors }
    }
}

pub fn clone_graph(node: Option<Rc<RefCell<Node>>>) -> Option<Rc<RefCell<Node>>> {
    fn dfs(
        node: Rc<RefCell<Node>>,
        visited: &mut HashMap<i32, Rc<RefCell<Node>>>,
    ) -> Rc<RefCell<Node>> {
        if let Some(v) = visited.get(&node.borrow().val) {
            return v.clone();
        }
        let clone = Rc::new(RefCell::new(Node::new(node.borrow().val, vec![])));
        visited.insert(node.borrow().val, clone.clone());
        for n in &node.borrow().neighbors {
            clone.borrow_mut().neighbors.push(dfs(n.clone(), visited));
        }
        clone
    }

    let mut visited = HashMap::new();
    node.map(|n| dfs(n, &mut visited))
}
