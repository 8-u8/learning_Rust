fn myprint<T: std::fmt::Display>(msg: &T) {
    println!("{}", *msg);
}

fn main() {
    let s = "Hello".to_string();
    myprint(&s);
    myprint(&s); // もう一回
}