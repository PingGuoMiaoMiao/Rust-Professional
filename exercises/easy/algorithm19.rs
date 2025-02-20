pub fn fib(n: i32) -> i32 {
    if n <= 1 {
        return n;
    }
    matrix_pow(n - 1)[0][0]
}

// 定义矩阵乘法
fn matrix_mult(a: [[i32; 2]; 2], b: [[i32; 2]; 2]) -> [[i32; 2]; 2] {
    let mut result = [[0; 2]; 2];
    for i in 0..2 {
        for j in 0..2 {
            result[i][j] = a[i][0] * b[0][j] + a[i][1] * b[1][j];
        }
    }
    result
}

// 定义矩阵快速幂
fn matrix_pow(n: i32) -> [[i32; 2]; 2] {
    let mut result = [[1, 0], [0, 1]]; // 单位矩阵
    let mut base = [[1, 1], [1, 0]]; // 基础矩阵
    let mut n = n;
    while n > 0 {
        if n % 2 == 1 {
            result = matrix_mult(result, base);
        }
        base = matrix_mult(base, base);
        n /= 2;
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fib_1() {
        let result = fib(0);
        println!("Fibonacci of 0: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_fib_2() {
        let result = fib(1);
        println!("Fibonacci of 1: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_3() {
        let result = fib(2);
        println!("Fibonacci of 2: {}", result);
        assert_eq!(result, 1);
    }

    #[test]
    fn test_fib_4() {
        let result = fib(3);
        println!("Fibonacci of 3: {}", result);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_fib_5() {
        let result = fib(10);
        println!("Fibonacci of 10: {}", result);
        assert_eq!(result, 55);
    }

    #[test]
    fn test_fib_6() {
        let result = fib(20);
        println!("Fibonacci of 20: {}", result);
        assert_eq!(result, 6765);
    }
}
