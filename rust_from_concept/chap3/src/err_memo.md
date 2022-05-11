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