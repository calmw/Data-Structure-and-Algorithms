struct Queue<T> {
    // 容量
    cap: usize,
    // 数据容器
    data: Vec<T>,
}

impl<T> Queue<T> {
    // 初始化队列
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
    fn enqueue(&mut self, val: T) -> Result<(), String> {
        if self.len() >= self.cap {
            return Err("No space available".to_string());
        }
        self.data.insert(0, val);
        Ok(())
    }
    // 数据出队
    fn dequeue(&mut self) -> Option<T> {
        if self.len() == 0 {
            None
        } else {
            self.data.pop()
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
        let mut iterator = Iter { queue: Vec::new() };
        for item in self.data.iter() {
            iterator.queue.push(item);
        }
        iterator
    }

    fn iter_mut(&mut self) -> IterMut<T> {
        let mut iterator = IterMut { queue: Vec::new() };
        for item in self.data.iter_mut() {
            iterator.queue.push(item);
        }
        iterator
    }
}

// 类元组结构体
struct IntoIter<T>(Queue<T>);

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
    queue: Vec<&'a T>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.queue.remove(0))
    }
}

struct IterMut<'a, T: 'a> {
    queue: Vec<&'a mut T>,
}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    fn next(&mut self) -> Option<Self::Item> {
        Some(self.queue.remove(0))
    }
}

fn main() {}


