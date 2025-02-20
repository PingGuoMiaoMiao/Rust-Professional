use std::collections::HashMap;

pub fn are_anagrams(s1: String, s2: String) -> bool {
    // 归一化函数：去除非字母字符并转换为小写
    let normalize = |s: String| -> String {
        s.chars()
            .filter(|c| c.is_alphabetic()) // 只保留字母字符
            .map(|c| c.to_ascii_lowercase()) // 转换为小写
            .collect()
    };

    // 对两个字符串进行归一化
    let s1_normalized = normalize(s1);
    let s2_normalized = normalize(s2);

    // 如果归一化后的字符串长度不同，直接返回 false
    if s1_normalized.len() != s2_normalized.len() {
        return false;
    }

    // 使用 HashMap 统计字符频率
    let mut char_count: HashMap<char, i32> = HashMap::new();

    // 统计 s1 的字符频率
    for c in s1_normalized.chars() {
        *char_count.entry(c).or_insert(0) += 1;
    }

    // 减去 s2 的字符频率
    for c in s2_normalized.chars() {
        *char_count.entry(c).or_insert(0) -= 1;
    }

    // 如果所有字符的频率差为 0，则是变位词
    char_count.into_values().all(|count| count == 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_anagram_1() {
        let s1 = "listen".to_string();
        let s2 = "silent".to_string();
        let result = are_anagrams(s1, s2);
        println!("是变位词: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_2() {
        let s1 = "evil".to_string();
        let s2 = "vile".to_string();
        let result = are_anagrams(s1, s2);
        println!("是变位词: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_3() {
        let s1 = "hello".to_string();
        let s2 = "world".to_string();
        let result = are_anagrams(s1, s2);
        println!("是变位词: {}", result);
        assert_eq!(result, false);
    }

    #[test]
    fn test_anagram_4() {
        let s1 = "Clint Eastwood".to_string();
        let s2 = "Old West Action".to_string();
        let result = are_anagrams(s1, s2);
        println!("是变位词: {}", result);
        assert_eq!(result, true);
    }

    #[test]
    fn test_anagram_5() {
        let s1 = "Astronomer".to_string();
        let s2 = "Moon starer".to_string();
        let result = are_anagrams(s1, s2);
        println!("是变位词: {}", result);
        assert_eq!(result, true);
    }
}
