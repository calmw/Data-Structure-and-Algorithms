fn main() {
    let nums = vec![1, 2, 3, 4, 5, 6];
    // 顺序遍历
    for num in nums {
        println!("val: {num}");
    }
    //逆序遍历
    for num in nums.iter().rev() {
        println!("val: {num}");
    }
    // while和let也可以组成模式匹配，这样就不用写停止条件了，因为let语句会自动判断，符合条件才会继续执行while循环
    let mut v = vec![1, 2, 3, 4, 5, 6];
    while let Some(x) = v.pop() {
        println!("{x}")
    }
}
