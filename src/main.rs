mod medium;

fn main() {
    use leetcode::medium::remove_nth_node_from_end_of_list::ListNode;
    let head = ListNode::from_vec(vec![1, 2, 3, 4, 5]);
    let result = leetcode::medium::remove_nth_node_from_end_of_list::remove_nth_from_end(head, 2);
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
