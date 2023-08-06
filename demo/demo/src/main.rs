// slot保存键
// data保存值
// cap控制容量
#[derive(Debug, Clone, PartialEq)]
struct HashMap<T> {
    cap: usize,
    slot: Vec<T>,
    data: Vec<T>,
}

impl<T: Clone + PartialEq> HashMap<T> {
    fn new(cap: usize) -> Self {
        // 初始化slot和data
        let mut slot = Vec::with_capacity(cap);
        let mut data = Vec::with_capacity(cap);
        for i in 0..cap {
            slot.push(0);
            data.push(Default::default());
        }
        HashMap { cap, slot, data }
    }

    fn len(&self) -> usize {
        let mut len = 0;
        for &d in self.slot.iter() {
            // 槽中的数据不为0，表示有数据，对len加1
            if 0 != d {
                len += 1;
            }
        }
        len
    }
}

fn main() {}



