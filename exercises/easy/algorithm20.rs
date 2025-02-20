/*
    Sum of Two Integers
    Given two integers, calculate their sum without using the `+` operator. 
    You need to implement the function `get_sum(a: i32, b: i32) -> i32`.
    The function should return the sum of the two integers `a` and `b`.

    Hint: You can solve this problem using bitwise operations.
*/

use std::fmt::{self, Display, Formatter};

pub fn get_sum(a: i32, b: i32) -> i32 {
    let mut sum = 0;
    let mut carry;
 
    loop {
        // Sum bits without considering carry
        sum = a ^ b;
        // Carry is formed by ANDing bits of a and b and left shifting by one
        carry = (a & b) << 1;
        // If carry is zero, we are done
        if carry == 0 {
            break;
        }
        // Update a and b for next iteration
        a = sum;
        b = carry;
    }
 
    sum
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_1() {
        let result = get_sum(1, 2);
        println!("Sum of 1 and 2: {}", result);
        assert_eq!(result, 3);
    }

    #[test]
    fn test_sum_2() {
        let result = get_sum(-1, 1);
        println!("Sum of -1 and 1: {}", result);
        assert_eq!(result, 0);
    }

    #[test]
    fn test_sum_3() {
        let result = get_sum(100, 200);
        println!("Sum of 100 and 200: {}", result);
        assert_eq!(result, 300);
    }

    #[test]
    fn test_sum_4() {
        let result = get_sum(-50, -50);
        println!("Sum of -50 and -50: {}", result);
        assert_eq!(result, -100);
    }

    #[test]
    fn test_sum_5() {
        let result = get_sum(0, 0);
        println!("Sum of 0 and 0: {}", result);
        assert_eq!(result, 0);
    }
}
