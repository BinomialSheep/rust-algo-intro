use std::ops::Add;

// 正方行列を二次元配列で表現する
#[derive(Debug, Clone, PartialEq)]
struct SquareMatrix {
    size: usize,
    data: Vec<Vec<i32>>,
}

impl SquareMatrix {
    fn new(size: usize, data: Vec<Vec<i32>>) -> Self {
        assert_eq!(data.len(), size, "Matrices must have the same size.");
        for row in &data {
            assert_eq!(row.len(), size, "Matrices must have the same size.")
        }
        Self { size, data }
    }
}

impl Add for SquareMatrix {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        assert_eq!(self.size, other.size);
        let mut result = vec![vec![0; self.size]; self.size];
        for i in 0..self.size {
            for j in 0..self.size {
                result[i][j] = self.data[i][j] + other.data[i][j];
            }
        }
        Self {
            size: self.size,
            data: result,
        }
    }
}

fn naive_matrix_multiply(matrix1: &SquareMatrix, matrix2: &SquareMatrix) -> SquareMatrix {
    let size = matrix1.size;
    // Θ(N^2)
    let mut result = vec![vec![0; size]; size];
    // Θ(N^3)
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += matrix1.data[i][k] * matrix2.data[k][j];
            }
        }
    }
    SquareMatrix {
        size: size,
        data: result,
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_addition() {
        let matrix1 = SquareMatrix::new(2, vec![vec![1, 2], vec![3, 4]]);
        let matrix2 = SquareMatrix::new(2, vec![vec![5, 6], vec![7, 8]]);
        let expected = SquareMatrix::new(2, vec![vec![6, 8], vec![10, 12]]);
        assert_eq!(matrix1 + matrix2, expected);
    }

    #[test]
    #[should_panic]
    fn test_addition_different_sizes() {
        let matrix1 = SquareMatrix::new(2, vec![vec![1, 2], vec![3, 4]]);
        let matrix2 = SquareMatrix::new(3, vec![vec![5, 6, 7], vec![8, 9, 10], vec![11, 12, 13]]);
        let _ = matrix1 + matrix2; // サイズが異なるためパニックを期待
    }

    #[test]
    fn test_matrix_multiplication() {
        let matrix1 = SquareMatrix::new(2, vec![vec![1, 2], vec![3, 4]]);
        let matrix2 = SquareMatrix::new(2, vec![vec![5, 6], vec![7, 8]]);
        let expected = SquareMatrix::new(2, vec![vec![19, 22], vec![43, 50]]);
        assert_eq!(naive_matrix_multiply(&matrix1, &matrix2), expected);
    }
}
