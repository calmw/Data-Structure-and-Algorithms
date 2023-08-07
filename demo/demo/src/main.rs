fn quick_sort2(nums: &mut [i32], low: usize, high: usize) {
    if low > high { return; }
    let mut lm = low;
    let mut rm = high;
    while lm < rm {
        // 将右标记不断左移
        while lm < rm && nums[low] <= nums[rm] {
            rm -= 1;
        }
        // 将左标记不断右移
        while lm < rm && nums[lm] <= nums[low] {
            lm += 1;
        }
        // 交换左右标记处的值
        nums.swap(lm, rm)
    }
    // 交换分割点数据
    nums.swap(low, lm);
    if lm > 1 {
        quick_sort2(nums, low, lm - 1);
    }
    quick_sort2(nums, rm + 1, high);
}


fn main() {
    let mut nums = [12, 78, 789, 22, 56, 86, 2, 45, 213, 789, 124, 6880, 35];
    let high = nums.len() - 1;
    quick_sort2(&mut nums, 0, high);
    println!("sorted nums: {:?}", nums)
}



