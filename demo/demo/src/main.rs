fn insertion_sort(nums: &mut [i32]) {
    if nums.len() < 2 { return; }
    for i in 1..nums.len() {
        let mut pos = i;
        let curr = nums[i];
        while pos > 0 && curr < nums[pos - 1] {
            nums[pos] = nums[pos - 1];// 向右移动数据
            pos -= 1;
        }
        println!("pos:{pos}");
        nums[pos] = curr; // 插入数据
    }
}

fn insertion_sort2(nums: &mut [i32]) {
    if nums.len() < 2 { return; }
    let len_nums = nums.len() - 1;
    for i in 1..len_nums {
        // nums[0..i]相当于较低位的有序序列
        for j in 0..i {
            if nums[i] < nums[j] {
                nums[j + 1] = nums[j];
                nums[j] = nums[i];

            // } else {
            //     nums[j + 1] = nums[j];
            }
        }
    }
}

fn main() {
    let mut nums = [12, 78, 789, 22, 56, 86, 2, 45, 213, 789, 124, 6880, 35];
    insertion_sort(&mut nums);
    // insertion_sort2(&mut nums);
    println!("sorted nums: {:?}", nums)
}



