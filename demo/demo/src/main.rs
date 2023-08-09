fn selection_sort(nums: &mut Vec<i32>) {
    let mut left = nums.len() - 1;// 待排序数据的下标

    while left > 0 {
        let mut pos_max = 0;
        for i in 1..=left {
            if nums[i] > nums[pos_max] {
                pos_max = i;// 选择最大值的下标
            }
        }
        // 交换数据，完成一轮数据的排序，将待排序的数据个数减1
        println!("max_index: {pos_max}");
        nums.swap(left, pos_max);
        left -= 1;
    }
}

fn main() {
    let mut nums = vec![12, 78, 789, 22, 56, 86, 2, 45, 213, 789, 124, 6880, 35];
    selection_sort(&mut nums);
    println!("sorted nums: {:?}", nums)
}



