mod medium;

fn main() {
    use leetcode::medium::binary_tree_right_side_view::{right_side_view, TreeNode};
    let root = TreeNode::from_vec(vec![
        Some(1),
        Some(2),
        Some(3),
        None,
        Some(5),
        None,
        Some(4),
    ]);
    let result = right_side_view(root);
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
