#![allow(dead_code)]
use std::collections::BinaryHeap;

pub fn k_closest(points: Vec<Vec<i32>>, k: i32) -> Vec<Vec<i32>> {
    let k = k as usize;
    let mut pq = BinaryHeap::with_capacity(k);
    for p in &points {
        let d = p[0] * p[0] + p[1] * p[1];
        pq.push((d, vec![p[0], p[1]]));
        if pq.len() > k {
            pq.pop();
        }
    }
    pq.into_iter().map(|(_, p)| p).collect()
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    fn compare_vec_of_vecs(a: Vec<Vec<i32>>, b: Vec<Vec<i32>>) -> bool {
        let mut a = a;
        let mut b = b;
        a.sort();
        b.sort();
        a == b
    }

    #[test]
    fn test_1() {
        let points = vec![vec![1, 3], vec![-2, 2]];
        let k = 1;
        let expected = vec![vec![-2, 2]];
        let result = k_closest(points, k);
        assert!(compare_vec_of_vecs(result, expected));
    }

    #[test]
    fn test_2() {
        let points = vec![vec![3, 3], vec![5, -1], vec![-2, 4]];
        let k = 2;
        let expected = vec![vec![3, 3], vec![-2, 4]];
        let result = k_closest(points, k);
        assert!(compare_vec_of_vecs(result, expected));
    }
}
