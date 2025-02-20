use std::fmt;

struct Node<T> {
    val: T,
    next: Option<Box<Node<T>>>,
}

impl<T> Node<T> {
    fn new(val: T) -> Self {
        Node { val, next: None }
    }
}

impl<T> fmt::Display for Node<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.val)?;
        if let Some(ref next) = self.next {
            write!(f, ", {}", next)?;
        }
        Ok(())
    }
}

struct LinkedList<T> {
    start: Option<Box<Node<T>>>,
    length: usize,
}

impl<T> LinkedList<T> {
    fn new() -> Self {
        LinkedList { start: None, length: 0 }
    }

    fn add(&mut self, val: T) {
        let new_node = Box::new(Node::new(val));
        let mut current = &mut self.start;
        while let Some(ref mut node) = current {
            current = &mut node.next;
        }
        *current = Some(new_node);
        self.length += 1;
    }

    fn merge(mut list_a: LinkedList<T>, mut list_b: LinkedList<T>) -> LinkedList<T>
    where
        T: Ord,
    {
        let mut result = LinkedList::new();
        while let Some(mut node_a) = list_a.start.take() {
            if let Some(mut node_b) = list_b.start.take() {
                if node_a.val < node_b.val {
                    list_a.start = node_a.next.take();
                    result.add(node_a.val);
                    list_b.start = Some(node_b);
                } else {
                    list_b.start = node_b.next.take();
                    result.add(node_b.val);
                    list_a.start = Some(node_a);
                }
            } else {
                list_a.start = node_a.next.take();
                result.add(node_a.val);
            }
        }
        while let Some(node_b) = list_b.start.take() {
            result.add(node_b.val);
            list_b.start = node_b.next;
        }
        result
    }

    fn get(&self, index: i32) -> Option<&T> {
        let mut current = &self.start;
        let mut i = 0;
        while let Some(ref node) = current {
            if i == index {
                return Some(&node.val);
            }
            current = &node.next;
            i += 1;
        }
        None
    }
}

impl<T> fmt::Display for LinkedList<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(ref node) = self.start {
            write!(f, "{}", node)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![1, 3, 5, 7];
        let vec_b = vec![2, 4, 6, 8];
        let target_vec = vec![1, 2, 3, 4, 5, 6, 7, 8];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }

    #[test]
    fn test_merge_linked_list_2() {
        let mut list_a = LinkedList::<i32>::new();
        let mut list_b = LinkedList::<i32>::new();
        let vec_a = vec![11, 33, 44, 88, 89, 90, 100];
        let vec_b = vec![1, 22, 30, 45];
        let target_vec = vec![1, 11, 22, 30, 33, 44, 45, 88, 89, 90, 100];

        for i in 0..vec_a.len() {
            list_a.add(vec_a[i]);
        }
        for i in 0..vec_b.len() {
            list_b.add(vec_b[i]);
        }
        println!("list a {} list b {}", list_a, list_b);
        let list_c = LinkedList::merge(list_a, list_b);
        println!("merged List is {}", list_c);
        for i in 0..target_vec.len() {
            assert_eq!(target_vec[i], *list_c.get(i as i32).unwrap());
        }
    }
}
