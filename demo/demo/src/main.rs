

fn main() {
    trait A{ fn foo(&self) -> Self; }
    Box<Vec<dyn A>>
}


