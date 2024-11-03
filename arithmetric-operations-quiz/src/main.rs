use rand::Rng;

fn main() {
    let mut num_of_correct = 0; // 正解の回数を数える
    while num_of_correct < 3 {
        // quiz_mode をランダムに1か2に決める
        let quiz_mode = rand::thread_rng().gen_range(1..=2);
        match quiz_mode {
            // quiz_mode が 1のときは加算クイズ
            1 => {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);
                println!("{} + {} = ??", op1, op2);
                println!("?? の値を入兎力してください:");
                // ユーザーからの回答を保持する変数
                let mut ans_intput = String::new();

                // 標準入力から1行取得し、ans_inputに代入する
                std::io::stdin().read_line(&mut ans_intput).unwrap();
                let ans_input = ans_intput.trim().parse::<i32>().unwrap();
                dbg!(ans_input);

                if dbg!(ans_input == op1 + op2) {
                    println!("正解！");
                    num_of_correct += 1; // 正解したら1回足す // breakする必要がなくなった
                } else {
                    println!("不正解！")
                }
            }
            // quiz_mode が 2のときは減算クイズ
            2 => {
                let op1 = rand::thread_rng().gen_range(0..100);
                let op2 = rand::thread_rng().gen_range(0..100);
                println!("{} - {} = ??", op1, op2);
                println!("?? の値を入兎力してください:");
                // ユーザーからの回答を保持する変数
                let mut ans_intput = String::new();

                // 標準入力から1行取得し、ans_inputに代入する
                std::io::stdin().read_line(&mut ans_intput).unwrap();
                let ans_input = ans_intput.trim().parse::<i32>().unwrap();
                dbg!(ans_input);

                if dbg!(ans_input == op1 - op2) {
                    println!("正解！");
                    num_of_correct += 1; // 正解したら1回足す
                } else {
                    println!("不正解！")
                }
            }
            // その他
            _ => unreachable!(),
        }
    }
    println!("クリア！");
}
