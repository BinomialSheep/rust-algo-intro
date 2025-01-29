use rand::Rng;
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

    // ゼロ行列を生成する
    pub fn zero(size: usize) -> Self {
        Self {
            size,
            data: vec![vec![0; size]; size],
        }
    }

    // 行列を4つのサブ行列に分割する
    pub fn split(&self) -> (Self, Self, Self, Self) {
        let half = self.size / 2;
        let mut a11 = vec![vec![0; half]; half];
        let mut a12 = vec![vec![0; half]; half];
        let mut a21 = vec![vec![0; half]; half];
        let mut a22 = vec![vec![0; half]; half];

        for i in 0..half {
            for j in 0..half {
                a11[i][j] = self.data[i][j];
                a12[i][j] = self.data[i][j + half];
                a21[i][j] = self.data[i + half][j];
                a22[i][j] = self.data[i + half][j + half];
            }
        }

        (
            Self::new(half, a11),
            Self::new(half, a12),
            Self::new(half, a21),
            Self::new(half, a22),
        )
    }

    // 4つのサブ行列を1つの行列に結合する
    pub fn merge(&mut self, c11: &Self, c12: &Self, c21: &Self, c22: &Self) {
        let half = self.size / 2;
        for i in 0..half {
            for j in 0..half {
                self.data[i][j] += c11.data[i][j];
                self.data[i][j + half] += c12.data[i][j];
                self.data[i + half][j] += c21.data[i][j];
                self.data[i + half][j + half] += c22.data[i][j];
            }
        }
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

// ランダム行列生成
fn generate_random_matrix(size: usize, min: i32, max: i32) -> SquareMatrix {
    let mut rng = rand::thread_rng();
    let data: Vec<Vec<i32>> = (0..size)
        .map(|_| (0..size).map(|_| rng.gen_range(min..=max)).collect())
        .collect();
    SquareMatrix::new(size, data)
}

// Θ(N^3)の普通の行列乗算
fn naive_matrix_multiply(matrix_a: &SquareMatrix, matrix_b: &SquareMatrix) -> SquareMatrix {
    let size = matrix_a.size;
    let mut result = vec![vec![0; size]; size];
    for i in 0..size {
        for j in 0..size {
            for k in 0..size {
                result[i][j] += matrix_a.data[i][k] * matrix_b.data[k][j];
            }
        }
    }
    SquareMatrix {
        size: size,
        data: result,
    }
}

// 再帰を使ったΘ(N^3)の行列乗算
fn matrix_multiply_recursive(
    matrix_a: &SquareMatrix,
    matrix_b: &SquareMatrix,
    result: &mut SquareMatrix,
    n: usize,
) {
    // 基底段階
    if n == 1 {
        result.data[0][0] += matrix_a.data[0][0] * matrix_b.data[0][0];
        return;
    }

    // 分割
    let (a11, a12, a21, a22) = matrix_a.split();
    let (b11, b12, b21, b22) = matrix_b.split();

    let half = n / 2;
    let mut c11 = SquareMatrix::zero(half);
    let mut c12 = SquareMatrix::zero(half);
    let mut c21 = SquareMatrix::zero(half);
    let mut c22 = SquareMatrix::zero(half);

    // 統治
    matrix_multiply_recursive(&a11, &b11, &mut c11, half);
    matrix_multiply_recursive(&a12, &b21, &mut c11, half);

    matrix_multiply_recursive(&a11, &b12, &mut c12, half);
    matrix_multiply_recursive(&a12, &b22, &mut c12, half);

    matrix_multiply_recursive(&a21, &b11, &mut c21, half);
    matrix_multiply_recursive(&a22, &b21, &mut c21, half);

    matrix_multiply_recursive(&a21, &b12, &mut c22, half);
    matrix_multiply_recursive(&a22, &b22, &mut c22, half);

    // 結合
    result.merge(&c11, &c12, &c21, &c22);
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

    #[test]
    fn test_matrix_multiply_recursive() {
        let matrix1 = SquareMatrix::new(2, vec![vec![1, 2], vec![3, 4]]);
        let matrix2 = SquareMatrix::new(2, vec![vec![5, 6], vec![7, 8]]);
        let expected = SquareMatrix::new(2, vec![vec![19, 22], vec![43, 50]]);

        let mut result = SquareMatrix::zero(2);
        matrix_multiply_recursive(&matrix1, &matrix2, &mut result, 2 as usize);
        assert_eq!(result, expected);
    }

    #[test]
    fn test_random_matrix_multiplication() {
        let size = 16;

        // ランダムな行列を生成
        let matrix1 = generate_random_matrix(size, -10, 10);
        let matrix2 = generate_random_matrix(size, -10, 10);

        // 両方の方法で計算
        let expected = naive_matrix_multiply(&matrix1, &matrix2);
        let mut recursive_result = SquareMatrix::zero(size);
        matrix_multiply_recursive(&matrix1, &matrix2, &mut recursive_result, size);

        // 結果が一致することを確認
        assert_eq!(
            expected, recursive_result,
            "Multiplication results do not match!"
        );
    }
}
