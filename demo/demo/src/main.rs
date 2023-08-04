fn sequential_search_pos(nums: &[i32], num: i32) -> Option<usize> {
    let mut pos: usize = 0;
    let mut found = false;
    while pos < nums.len() {
        if num == nums[pos] {
            found = true;
            return Some(pos)
        } else {
            pos += 1;
        }
    }
    None
}

fn main() {
    let num = 8;
    let nums = [9, 3, 5, 1, 8, 4, 2, 7];
    match sequential_search_pos(&nums, num) {
        Some(pos) => println!("{num}'s index: {pos}"),
        None => println!("nums doesn't contains {num}")
    }
}


