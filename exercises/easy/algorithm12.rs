pub fn is_palindrome(s: String) -> bool {
    // 预处理字符串：转换为小写并移除非字母字符
    let normalized: String = s
        .chars()
        .filter(|c| c.is_ascii_alphabetic()) // 只保留字母字符
        .map(|c| c.to_ascii_lowercase()) // 转换为小写
        .collect();

    // 双指针法检查回文
    let mut left = 0;
    let mut right = normalized.len() as isize - 1;

    while left < right {
        if normalized.chars().nth(left as usize).unwrap() != normalized.chars().nth(right as usize).unwrap() {
            return false;
        }
        left += 1;
        right -= 1;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_palindrome_1() {
        let s = "A man, a plan, a canal, Panama".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_2() {
        let s = "Racecar".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_3() {
        let s = "Hello, World!".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_palindrome_4() {
        let s = "No 'x' in Nixon".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_palindrome_5() {
        let s = "Was it a car or a cat I saw?".to_string();
        let result = is_palindrome(s);
        println!("Is palindrome: {}", result);
        assert_eq!(result, true);
    }
}
