#![allow(dead_code)]
pub fn generate(num_rows: i32) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();

    for i in 0..num_rows as usize {
        let mut row = Vec::new();

        for j in 0..=i {
            if j == 0 || j == i {
                row.push(1);
            } else {
                row.push(result[i - 1][j - 1] + result[i - 1][j]);
            }
        }

        result.push(row);
    }
    result
}

/*
 Algorithm - Dynamic Programming - 118. Pascal's Triangle
   1. Create a vector of vector of i32
   2. Iterate from 0 to num_rows
   3. Create a vector of i32
   4. Iterate from 0 to i
   5. If j == 0 or j == i, push 1 to row
   6. Else push result[i - 1][j - 1] + result[i - 1][j] to row
   7. Push row to result
   8. Return result

  Complexity
    Time - O(n^2)
    Space - O(n^2)
*/

pub fn get_row(row_index: i32) -> Vec<i32> {
    let mut result = generate(row_index + 1);

    result.pop().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate() {
        let result = generate(5);
        assert_eq!(
            result,
            vec![
                vec![1],
                vec![1, 1],
                vec![1, 2, 1],
                vec![1, 3, 3, 1],
                vec![1, 4, 6, 4, 1]
            ]
        );
    }

    #[test]
    fn test_get_row() {
        let result = get_row(3);
        assert_eq!(result, vec![1, 3, 3, 1]);
    }
}
