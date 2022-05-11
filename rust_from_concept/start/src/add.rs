fn add_i32(x: i32, y: i32) -> i32 {
    x + y
}

#[test]
fn test1(){
    assert_eq!(add_i32(1, 2), 3); //成功
}

#[test]
fn test2(){
    assert_eq!(add_i32(1, 2), 4); //失敗
}

fn main() {
    println!("{}", add_i32(2, 5));
}