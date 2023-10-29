mod medium;

fn main() {
    use leetcode::medium::binary_tree_level_order_traversal::{level_order, TreeNode};
    let root = TreeNode::from_vec(vec![
        Some(3),
        Some(9),
        Some(20),
        None,
        None,
        Some(15),
        Some(7),
    ]);

    let result = level_order(root);
    println!("result: {:?}", result);
}

// Example of a binary heap
// use std::collections::BinaryHeap;

// fn main() {
//     let mut heap = BinaryHeap::new();

//     // Agregamos elementos al heap
//     heap.push(5);
//     heap.push(2);
//     heap.push(7);
//     heap.push(1);

//     // Imprimimos el heap
//     println!("Heap: {:?}", heap);

//     // Eliminamos el elemento con la mayor prioridad
//     let max = heap.pop().unwrap();
//     println!("Elemento con la mayor prioridad: {}", max);

//     // Imprimimos el heap actualizado
//     println!("Heap actualizado: {:?}", heap);
// }
