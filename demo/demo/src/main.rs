// 普通二分查找
fn binary_search1(nums: &[i32], num: &i32) -> bool {
    let mut low = 0;
    let mut high = nums.len() - 1;
    let mut found = false;

    while low <= high && !found {
        let mid: usize = (low + high) >> 1;
        println!("mid: {mid},{low}-{high}");
        // 若low + high可能溢出，则转换为减法
        // let mid: usize = low + ((high - low) >> 1);
        if *num == nums[mid] {
            found = true;
        } else if num < &nums[mid] {
            // num 小于中间值，省去后半部分的数据
            high = mid - 1;
        } else {
            // mid 大于或等于中间值，省去前半部分的数据
            low = mid + 1;
        }
    }
    found
}

// 递归法二分查找
fn binary_search2(nums: &[i32], num: &i32) -> bool {
    // 基本情况1：目标不存在
    if nums.len() == 0 { return false; }

    // 基本情况2: 目标存在
    let mid: usize = nums.len() >> 1;
    return if *num == nums[mid] {
        true
    } else if num < &nums[mid] {
        binary_search2(&nums[..mid], num)
    } else {
        binary_search2(&nums[mid + 1..], num)
    }
}

fn main() {
    let target = 8;
    let nums = [1, 2, 3, 5, 6, 8, 9, 11, 20, 24, 27, 32, 51, 68, 77, 98];
    let found = binary_search1(&nums, &target);
    println!("nums contains {target}: {found}");

    let found = binary_search2(&nums, &target);
    println!("nums contains {target}: {found}")
}



