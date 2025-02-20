use std::collections::HashMap;

pub fn longest_substring_without_repeating_chars(s: String) -> i32 {
    let mut char_index_map: HashMap<char, usize> = HashMap::new(); // 记录字符的最后出现位置
    let mut max_length = 0; // 最大长度
    let mut left = 0; // 滑动窗口的左边界

    for (right, c) in s.chars().enumerate() {
        // 如果字符已经存在，并且其位置在 left 右侧，则更新 left
        if let Some(&prev_index) = char_index_map.get(&c) {
            if prev_index >= left {
                left = prev_index + 1;
            }
        }

        // 更新字符的最后出现位置
        char_index_map.insert(c, right);

        // 计算当前窗口的长度，并更新最大长度
        max_length = max_length.max((right - left + 1) as i32);
    }

    max_length
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_longest_substring_1() {
        let s = "abcabcbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "abc"
    }

    #[test]
    fn test_longest_substring_2() {
        let s = "bbbbb".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 1); // "b"
    }

    #[test]
    fn test_longest_substring_3() {
        let s = "pwwkew".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 3); // "wke"
    }

    #[test]
    fn test_longest_substring_4() {
        let s = "".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 0); // Empty string
    }

    #[test]
    fn test_longest_substring_5() {
        let s = "abcde".to_string();
        let result = longest_substring_without_repeating_chars(s);
        println!("Length of longest substring: {}", result);
        assert_eq!(result, 5); // "abcde"
    }
}
