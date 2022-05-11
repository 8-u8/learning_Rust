fn main() {
    let mut x = 1; // 型は推論される
    x += 1; //error
    println!("x = {}", x);
}
