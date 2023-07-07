// use std::fmt::Error;
use anyhow::{bail,Error,Result};
use base64::encode;
use hash::merhash::mersenne_hash;

// 密码子（长度100），可随意交换次序和增减字符
const CRYPTO: &str = "!@#$e56876yserw5678oiujwq12345877089uiyhgfdxc%^*OIOLKJHBHNm";

pub fn generate_password(seed: &str, length: usize) -> Result<String, Error> {
    if length < 6 {
        bail!("length must >=6");
    }

    // 计算mer_hash
    let p = match length {
        6..=10 => 1,
        11..=15 => 2,
        16..=20 => 3,
        _ => 3,
    };
    let mut mer_hash = mersenne_hash(seed).pow(p);

    // 由mer_hash求password
    let mut password = String::new();
    let crypto_len = CRYPTO.len();
    while mer_hash > 9 {
        let loc = mer_hash % crypto_len;
        let nthc = CRYPTO.chars().nth(loc).expect("Error while get char!");
        password.push(nthc);
        mer_hash /= crypto_len;
    }

    // 将seed中的字符和password拼接起来
    let interval = password.clone();
    for c in seed.chars() {
        password.push(c);
        password += &interval;
    }

    // 将password编码为base64
    password = encode(password);
    password = password.replace("+", "*").replace("/", "*");

    // 长度不够 interval来凑
    let interval = password.clone();
    while password.len() < length {
        password += &interval;
    }

    // 返回前length个字符作为密码
    Ok(format!("{}: {}", seed, &password[..length]))
}