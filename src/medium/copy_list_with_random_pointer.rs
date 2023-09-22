use std::collections::HashMap;

#[derive(Clone)]
pub struct Node {
    pub val: i32,
    pub next: Option<Box<Node>>,
    pub random: Option<Box<Node>>,
}

impl Node {
    #[inline]
    pub fn new(val: i32) -> Self {
        Node {
            next: None,
            random: None,
            val,
        }
    }
}

pub fn copy_random_list(head: Option<Box<Node>>) -> Option<Box<Node>> {
    if head.is_none() {
        return None;
    }

    let mut old_new_mapping: HashMap<usize, Box<Node>> = HashMap::new();
    let mut new_nodes: HashMap<usize, Box<Node>> = HashMap::new();

    let mut current = head.as_ref();
    while let Some(node) = current {
        let id = node.as_ref() as *const Node as usize;
        let cloned_node = Box::new(Node::new(node.val));
        old_new_mapping.insert(id, cloned_node);
        current = node.next.as_ref();
    }

    // Set the 'next' and 'random' pointers for the cloned nodes.
    current = head.as_ref();
    while let Some(node) = current {
        let id = node.as_ref() as *const Node as usize;
        let cloned_node = old_new_mapping.get_mut(&id).unwrap();

        if let Some(next_node) = &node.next {
            let id = next_node.as_ref() as *const Node as usize;
            cloned_node.next = Some({
                if let Some(new_node) = new_nodes.get(&id) {
                    new_node.clone()
                } else {
                    let new_node = old_new_mapping.get(&id).unwrap().clone();
                    new_nodes.insert(id, new_node.clone());
                    new_node
                }
            });
        }

        if let Some(random_node) = &node.random {
            let id = random_node.as_ref() as *const Node as usize;
            cloned_node.random = Some({
                if let Some(new_node) = new_nodes.get(&id) {
                    new_node.clone()
                } else {
                    let new_node = old_new_mapping.get(&id).unwrap().clone();
                    new_nodes.insert(id, new_node.clone());
                    new_node
                }
            });
        }
        current = node.next.as_ref();
    }

    Some(
        old_new_mapping
            .get(&(head.as_ref().unwrap().as_ref() as *const Node as usize))
            .unwrap()
            .clone(),
    )
}
