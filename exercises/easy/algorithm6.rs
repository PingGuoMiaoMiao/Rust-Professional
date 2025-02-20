#[derive(Debug)]
struct Stack<T> {
    size: usize,
    data: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Self {
            size: 0,
            data: Vec::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.size == 0
    }

    fn len(&self) -> usize {
        self.size
    }

    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }

    fn push(&mut self, val: T) {
        self.data.push(val);
        self.size += 1;
    }

    fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            self.size -= 1;
            self.data.pop()
        }
    }

    fn peek(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            self.data.get(self.size - 1)
        }
    }

    fn peek_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            self.data.get_mut(self.size - 1)
        }
    }

    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        Iter {
            stack: self.data.iter().collect(),
        }
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        IterMut {
            stack: self.data.iter_mut().collect(),
        }
    }
}

struct IntoIter<T>(Stack<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    stack: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

struct IterMut<'a, T: 'a> {
    stack: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;

    fn next(&mut self) -> Option<Self::Item> {
        self.stack.pop()
    }
}

fn bracket_match(bracket: &str) -> bool {
    let mut stack = Stack::new();
    let bracket_pairs = [('(', ')'), ('[', ']'), ('{', '}')];

    for ch in bracket.chars() {
        match ch {
            // 如果是开括号，入栈
            '(' | '[' | '{' => stack.push(ch),
            // 如果是闭括号，检查是否匹配
            ')' | ']' | '}' => {
                if let Some(&opening) = stack.peek() {
                    if let Some(&(_, close)) = bracket_pairs.iter().find(|&&(open, _)| open == opening) {
                        if close == ch {
                            stack.pop(); // 匹配成功，弹出栈顶元素
                        } else {
                            return false; // 不匹配
                        }
                    }
                } else {
                    return false; // 栈为空，没有匹配的开括号
                }
            }
            _ => continue, // 忽略其他字符
        }
    }

    stack.is_empty() // 栈为空表示所有括号都匹配
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bracket_matching_1() {
        let s = "(2+3){func}[abc]";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_2() {
        let s = "(2+3)*(3-1";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_3() {
        let s = "{{([])}}";
        assert_eq!(bracket_match(s), true);
    }

    #[test]
    fn bracket_matching_4() {
        let s = "{{(}[)]}";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_5() {
        let s = "[[[]]]]]]]]]";
        assert_eq!(bracket_match(s), false);
    }

    #[test]
    fn bracket_matching_6() {
        let s = "";
        assert_eq!(bracket_match(s), true);
    }
}
