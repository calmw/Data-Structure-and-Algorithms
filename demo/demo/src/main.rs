fn bubble_sort(nums: &mut [i32]) {
    if nums.len() < 2 {
        return;
    }
    for i in 1..nums.len() {
        for j in 0..nums.len() - 1 {
            if nums[j] > nums[j + 1] {
                nums.swap(j, j + 1)
            }
        }
    }
}

fn main() {
    let mut nums = [12, 78, 789, 22, 56, 86, 2, 45, 213, 789, 124, 6880, 35];
    bubble_sort(&mut nums);
    println!("sorted nums: {:?}", nums)
}



