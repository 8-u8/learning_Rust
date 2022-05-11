# 3.1 compile error
```rust
error[E0384]: cannot assign twice to immutable variable `x`
「イミュータブルな変数xは2回割当できません」
 --> src/main.rs:3:5
  |
2 |     let x = 1;
  |         -
  |         |
  |         first assignment to `x`
  |         help: consider making this binding mutable: `mut x`
            「`let`ではなく、ミュータブルな変数`mut`として作成してみては？」
3 |     x = x + 1; //error
  |     ^^^^^^^^^ cannot assign twice to immutable variable

For more information about this error, try `rustc --explain E0384`.
error: could not compile `chap3` due to previous error
```

# 3.3
```rust
fn main() {
    let s = "Hello".to_string();
    let t = s; // 所有権を移す(ムーブセマンティクス)
    /* "Hello"の所有権がsからtに移動したので、sから"Hello"にアクセスできなくなる */
    println!("{}", t);
    println!("{}", s);
}


error[E0382]: borrow of moved value: `s`
 --> src/main.rs:6:20
  |
2 |     let s = "Hello".to_string();
  |         - move occurs because `s` has type `String`, which does not implement the `Copy` trait
              sはString型で、String型はCopyトレイトで実装しない
3 |     let t = s;
  |             - value moved here
...
6 |     println!("{}", s);
  |                    ^ value borrowed here after move
                          「値が『移動』のあとに『借用=参照』されている」
  |
  = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
          「このエラーは$crate::format_args_nlマクロの中に起因する」

For more information about this error, try `rustc --explain E0382`.
error: could not compile `chap3` due to previous error
```
## 問題: 関数内の変数に移動した所有権
```rust
fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    myprint(s);
    myprint(s); // msgにsの所有権が移動しているのでエラー。
}

```

## 回避方法A: コピーを置く
```rust
fn myprint<T: std::fmt::Display>(msg: T) {
    println!("{}", msg);
}

fn main() {
    let s = "Hello".to_string();
    let ss = s.clone();
    myprint(s);
    myprint(ss); // もう一回
}
```

## 回避方法B: リファレンスによる借用
```rust
fn myprint<T: std::fmt::Display>(msg: &T) {
    // リファレンスでmsgを受け取る
    println!("{}", *msg);
}

fn main() {
    let s = "Hello".to_string();
    myprint(&s); // &が参照するための演算子
    myprint(&s); // t = &sは、tにsの所有権を貸す。
}
```