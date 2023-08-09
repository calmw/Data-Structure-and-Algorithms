macro_rules! parent { // 计算父节点的下标
    ($child:ident) => {
        $child >> 1
    };
}

macro_rules! left_child { // 计算左子节点的下标
    ($parent:ident) => {
        $parent << 1
    };
}

macro_rules! right_child { // 计算右字节点的下标
    ($parent:ident) => {
        ($parent<<1) + 1
    };
}

fn heap_sort(nums: &mut [i32]) {
    if nums.len() < 2 { return; }

    let len = nums.len() - 1;
    let last_parent = parent!(len);
    for i in (1..last_parent).rev() {
        move_down(nums, i); // 第一次构建小顶堆，下标从1开始
    }
    for end in (1..nums.len()).rev() {
        nums.swap(1, end);
        move_down(&mut nums[..end], 1)// 重建堆
    }
}

fn move_down(nums: &mut [i32], mut parent: usize) {
    let last = nums.len() - 1;

    loop {
        let left = left_child!(parent);
        let right = right_child!(parent);
        if left > last { break; }

        // right<=last,保存在右字节点
        let child = if right <= last && nums[left] < nums[right] {
            right
        } else {
            left
        };
        // 子节点大于父节点，交换数据
        if nums[child] > nums[parent] {
            nums.swap(parent, child);
        }
        // 更新父子关系
        parent = child
    }
}

fn main() {
    let mut nums = [12, 78, 789, 22, 56, 86, 2, 45, 213, 789, 124, 6880, 35];
    heap_sort(&mut nums);
    println!("sorted nums: {:?}", nums)
}



