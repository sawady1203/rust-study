use std::io::stdin;

fn main() {
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // スペースで分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        println!("{:?}", tokens);
        // 式の計算
        let left: f64 = tokens[0].parse().unwrap();
        let right: f64 = tokens[2].parse().unwrap();
        let result = match tokens[1] {
            "+" => left + right,
            "-" => left - right,
            "*" => left * right,
            "/" => left / right,
            _ => {
                // 入力が正しいならここにはこない
                unreachable!()
            }
        };

        // 結果の表示
        print_result(result);
    }
}

fn print_result(value: f64) {
    println!(" => {}", value);
}
