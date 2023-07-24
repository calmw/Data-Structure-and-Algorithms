fn main() {
    let a ;
    {
        let b = 10;
        a = &b;
    }
    println!("{a}")
}
