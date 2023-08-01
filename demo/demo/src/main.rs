struct IntoIter<T>(Stack<T>);

// 类元组结构体
impl<T: Clone> Iterator for IntoIter<T> {
    type Iterm = T;
    fn next(&mut self) -> Option<Self::Item> {
        if !self.0.is_empty() {
            self.0.size -= 1;
            self.0.data.pop()
        } else {
            None
        }
    }
}


struct Stack<T> {
    // 栈大小
    size: usize,
    // 栈数据
    data: Vec<T>,
}

impl<T> Stack<T> {
    // 初始化栈
    fn new() -> Self {
        Self {
            size: 0,
            data: vec![],
        }
    }
    fn is_empty(&self) -> bool {
        0 == self.size
    }
    fn len(&self) -> usize {
        self.size
    }

    // 清空栈
    fn clear(&mut self) {
        self.size = 0;
        self.data.clear();
    }
    // 将数据保存在Vec末尾
    fn push(&mut self, val: T) {
        self.data.push(val)
    }
    // 弹出数据
    fn pop(&mut self) -> Option<T> {
        if 0 == self.size {
            return None;
        }
        self.size -= 1;
        self.data.pop()
    }
    // 返回栈顶数据引用
    fn peek(&self) -> Option<T> {
        if 0 == self.size { return None; }
        return self.data.get(self.size - 1);
    }
    // 返回栈顶数据可变引用
    fn peek_mut(&mut self) -> Option<&mut T> {
        if 0 == self.size { return None; }
        return self.data.get_mut(self.size - 1);
    }
    // 以下为栈实现的迭代功能
    // into_iter,栈改变成为迭代器
    // iter,栈不变，得到不可变迭代器
    // iter_mut,栈不变，得到可变迭代器
    fn iter(self) -> Iter<T> {}
}

fn main() {}


