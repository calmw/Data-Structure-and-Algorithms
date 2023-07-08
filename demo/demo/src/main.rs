// 枚举
enum Result<T, E> {
    Ok(T),
    Err(E),
}

// 特性Trait
pub trait From {
    fn from() -> Self;
}

// 结构体
struct Rectangle {
    height: i32,
    width: i32,
}

impl Rectangle {
    // 构造函数
    fn new(height: i32, width: i32) -> Self {
        Self { height, width }
    }

    // 函数
    fn calc_area(&self) -> i32 {
        self.width * self.height
    }
}

// 静态变量和常量
static NAME: &str = "kew";
const AGE: i32 = 25;

// 宏定义
macro_rules! add {
    ($a:expr,$b:expr) => {
        $a + $b
    };
}



fn main() {// 变量及宏的使用
    let sun_of_nums = add!(1,2);
    println!("Hello, world!");
    println!("sun_of_nums: {}", sun_of_nums);
}
