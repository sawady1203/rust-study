fn main() {
    println!("1 + 1 = ??");
    println!("?? の値を入兎力してください:");
    // ユーザーからの回答を保持する変数
    let mut ans_intput = String::new();

    // 標準入力から1行取得し、ans_inputに代入する
    std::io::stdin().read_line(&mut ans_intput).unwrap();
    let ans_input = ans_intput.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == 1 + 1) {
        println!("正解！")
    } else {
        println!("不正解！")
    }

    println!("1 - 4 = ??");
    println!("?? の値を入兎力してください:");
    // ユーザーからの回答を保持する変数
    let mut ans_intput = String::new();

    // 標準入力から1行取得し、ans_inputに代入する
    std::io::stdin().read_line(&mut ans_intput).unwrap();
    let ans_input = ans_intput.trim().parse::<i32>().unwrap();
    dbg!(ans_input);

    if dbg!(ans_input == 1 - 4) {
        println!("正解！")
    } else {
        println!("不正解！")
    }
    println!("i32が扱えるデータ範囲:{} ~ {}", i32::MIN, i32::MAX);
    println!("u32が扱えるデータ範囲:{} ~ {}", u32::MIN, u32::MAX);
}
