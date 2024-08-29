fn main() {
    let a = String::from("hello");
    let mut b = &a;
    println!("b: {}", b);
    let binding = "world".to_string();
    b = &binding;
    println!("b: {}", b);
}
