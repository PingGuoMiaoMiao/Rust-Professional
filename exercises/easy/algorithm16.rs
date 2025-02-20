pub fn rotate_matrix_90_degrees(matrix: &mut Vec<Vec<i32>>) {
    let n = matrix.len(); // 行数
    if n == 0 {
        return; // 空矩阵直接返回
    }
    let m = matrix[0].len(); // 列数

    // 如果是方阵，直接旋转
    if n == m {
        // 分层旋转
        for layer in 0..n / 2 {
            let first = layer;
            let last = n - 1 - layer;
            for i in first..last {
                let offset = i - first;
                // 保存上边的元素
                let top = matrix[first][i];
                // 将左边的元素移到上边
                matrix[first][i] = matrix[last - offset][first];
                // 将下边的元素移到左边
                matrix[last - offset][first] = matrix[last][last - offset];
                // 将右边的元素移到下边
                matrix[last][last - offset] = matrix[i][last];
                // 将上边的元素移到右边
                matrix[i][last] = top;
            }
        }
    } else {
        // 非方阵，需要创建一个新的矩阵
        let mut rotated = vec![vec![0; n]; m]; // 新矩阵大小为 m x n
        for i in 0..n {
            for j in 0..m {
                rotated[j][n - 1 - i] = matrix[i][j];
            }
        }
        *matrix = rotated; // 将旋转后的矩阵赋值给原矩阵
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rotate_matrix_1() {
        let mut matrix = vec![
            vec![1, 2, 3],
            vec![4, 5, 6],
            vec![7, 8, 9],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![7, 4, 1],
            vec![8, 5, 2],
            vec![9, 6, 3],
        ]);
    }

    #[test]
    fn test_rotate_matrix_2() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![3, 1],
            vec![4, 2],
        ]);
    }

    #[test]
    fn test_rotate_matrix_3() {
        let mut matrix = vec![
            vec![1],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![1],
        ]);
    }

    #[test]
    fn test_rotate_matrix_4() {
        let mut matrix = vec![
            vec![1, 2],
            vec![3, 4],
            vec![5, 6],
        ];
        rotate_matrix_90_degrees(&mut matrix);
        println!("Rotated matrix: {:?}", matrix);
        assert_eq!(matrix, vec![
            vec![5, 3, 1],
            vec![6, 4, 2],
        ]);
    }
}
