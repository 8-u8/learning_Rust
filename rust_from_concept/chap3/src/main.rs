// fn myprint<T: std::fmt::Display>(msg: &T) {
//     println!("{}", *msg);
// }

fn myclear(x: &mut String) {
    x.clear();
}

fn main() {
    let mut s = "Hello".to_string();
    println!("s = {}", s);
    let s_ref = &mut s;

    myclear(s_ref);

    println!("s = {}", s);

}