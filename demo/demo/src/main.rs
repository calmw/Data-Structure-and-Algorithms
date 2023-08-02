// 节点链表采用了Box指针（大小是确定的），因为确定大小才能分配内存
type Pointer<T> = Option<Box<Node<T>>>;

// 定义链表
struct List<T> {
    // 链表中的节点数
    size: usize,
    // 数据容器
    head: Node<T>,
}

// 链表节点
struct Node<T> {
    // 数据
    elem: T,
    // 下一个节点
    next: Pointer<T>,
}

impl<T> Deque<T> {
    // 初始化双端队列
    fn new() -> Self {
        Self {
            cap: 0,
            data: vec![],
        }
    }
    fn is_empty(&self) -> bool {
        0 == Self::len(&self)
    }

    fn len(&self) -> usize {
        self.data.len()
    }

    fn is_full(&self) -> bool {
        self.len() == self.cap
    }

    // 清空队列
    fn clear(&mut self) {
        self.data = Vec::with_capacity(self.cap);
    }

    // 向队列中插入数据
    fn add_front(&mut self, val: T) -> Result<(), String> {
        if self.len() >= self.cap {
            return Err("No space available".to_string());
        }
        self.data.push(val);
        Ok(())
    }
    fn add_rear(&mut self, val: T) -> Result<(), String> {
        if self.len() >= self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    // 数据出队
    fn remove_front(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            self.data.pop()
        }
    }
    fn remove_rear(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            Some(self.data.remove(0))
        }
    }

    // 以下为队列实现的迭代功能
    // into_iter,栈改变成为迭代器
    // iter,栈不变，得到不可变迭代器
    // iter_mut,栈不变，得到可变迭代器
    fn into_iter(self) -> IntoIter<T> {
        IntoIter(self)
    }

    fn iter(&self) -> Iter<T> {
        let mut iterator = Iter { deque: Vec::new() };
        for item in self.data.iter() {
            iterator.deque.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { deque: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.deque.push(item);
        }
        iterator
    }
}

// 类元组结构体
struct IntoIter<T>(Deque<T>);

impl<T: Clone> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            Some(self.0.data.remove(0))
        } else {
            None
        }
    }
}

struct Iter<'a, T: 'a> {
    deque: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.deque.remove(0))
    }
}

struct IterMut<'a, T: 'a> {
    deque: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.deque.remove(0))
    }
}

fn main() {}


