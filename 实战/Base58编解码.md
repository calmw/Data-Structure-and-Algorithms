#### base58编码

- 示例代码

```rust
// 最大转换进制
const BIG_RADIX: u32 = 58;

// 前置0用1代替
const ALPHABET_INDEX_0: char = '1';

// Base58编码字符
const ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

// 定义编解码的错误类型
pub enum DecodeError {
    Invalid,
    InvalidLength,
    InvalidCharacter(char, usize),
}

// 定义编解码trait
pub trait Encoder {
    fn encode_to_base58(&self) -> String;
}

pub trait Decoder {
    fn decode_to_base58(&self) -> Result<String, DecodeError>;
}

// 实现Base58编码
impl Encoder for str {
    fn encode_to_base58(&self) -> String {
        // 转换为字节以方便处理
        let str_u8 = self.as_bytes();
        // 统计前置0的个数
        let zero_count = str_u8.iter().take_while(|&&x| x == 0).count();
        // 转换后所需的空间：log(256)/log(58),约为原来的1.38倍
        // 前置0不需要，所以删除
        let size = (str_u8.len() - zero_count) * 138 / 100 + 1;
        // 字符进制转换
        let mut i = zero_count;
        let mut high = size - 1;
        let mut buffer = vec![0u8; size];
        while i < str_u8.len() {
            // j 为逐渐减小的下标，对应从后往前
            let mut j = size - 1;
            // carry 为从前往后读取的字符
            let mut carry = str_u8[i] as u32;
            // 将转换后的数据从后往前依次存放
            while j > high || carry != 0 {
                carry += 256 * buffer[j] as u32;
                buffer[j] = (carry % BIG_RADIX) as u8;
                carry /= BIG_RADIX;
                if j > 0 {
                    j -= 1;
                }
            }
            i += 1;
            high = j;
        }
        // 处理多个前置0
        let mut b58_str = String::new();
        for _ in 0..zero_count {
            b58_str.push(ALPHABET_INDEX_0);
        }
        // 获取编码后的字符并拼接成字符串
        let mut j = buffer.iter().take_while(|&&x| x == 0).count();
        while j < size {
            b58_str.push(ALPHABET[buffer[j] as usize] as char);
            j += 1;
        }
        // 返回编码后的字符串
        b58_str
    }
}


// 进制映射关系
const DIGITS_MAP: &'static [u8] = &[
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255,
    255, 0, 1, 2, 3, 4, 5, 6, 7, 8, 255, 255, 255, 255, 255, 255,
    255, 9, 10, 11, 12, 13, 14, 15, 16, 255, 17, 18, 19, 20, 21, 255,
    22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 18, 19, 20, 21, 255,
    255, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 255, 44, 45, 46,
    47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 255, 255, 255, 255, 255,
];


fn main() {
    println!("{:#?}", "abc".encode_to_base58())
}
```

#### base58解码
