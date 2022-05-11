fn func_ex_div_some(x: i32, y: i32) -> Option<i32> {
    /*i32: 符号付き32ビット整数型。iが「符号付き」、以降の数字がビット数
      ちなみに、f32は単精度浮動小数点の型。
      Option: 値がないことがあり得るということを示す型。*/
    /*ここで行っている処理: 割り算。
      分母が0になるとエラーになるので、条件分岐でNoneを返すようにしている。*/
    let ans = if y == 0 { None } else { Some(x / y) };
    ans
}

fn func_ex_div_result(x: i32, y: i32) -> Result<i32, &'static str> {
    /*ここで行っている処理: 同様に割り算。
      func_ex_div_someと同じだが、こちらはResult型を返す。
      Result型: エラーハンドリングでよく使う。
      今回はエラーの場合は"Division by zero"を返すので、文字列。
      エラーでない場合は割り算の結果を返す。
    */
    if y == 0 {
        Err("Division by zero") // 失敗したときの返り値はErrで包む
    } else {
        Ok(x / y) // 成功したときの返り値はOkで包む
    }
}

fn func_ex_print_some<T: std::fmt::Display>(ans: Option<T>) {
    if let Some(x) = ans { // Someの中に値が含まれているなら、表示する。
        println!("{}", x)
    } else {
        println!("None")
    }
}

fn func_ex_print_some_match<T: std::fmt::Display>(ans: Option<T>) {
    match ans {
        Some(x) => println!("{}", x),
        None => println!("None"),
    }
}

fn func_ex_print_result<T: std::fmt::Display, E: std::fmt::Display>(ans: Result<T, E>) {
    match ans {
        Ok(res) => println!("{}", res),
        Err(str) => println!("{}", str),
    }
}
fn main() { // main functions
    func_ex_print_some(func_ex_div_some(10, 5)); /* 関数の呼び出し */
    func_ex_print_some(func_ex_div_some(10, 0));
    func_ex_print_some_match(func_ex_div_some(10, 5));
    func_ex_print_some_match(func_ex_div_some(10, 0));
    func_ex_print_result(func_ex_div_result(10, 5));
    func_ex_print_result(func_ex_div_result(10, 0));
}
