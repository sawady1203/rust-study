use std::io::stdin;

fn main() {
    let mut memory: f64 = 0.0;
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // スペースで分割
        let tokens: Vec<&str> = line.split(char::is_whitespace).collect();
        println!("{:?}", tokens);

        // メモリへの書き込み
        if tokens[0] == "mem+" {
            add_and_print_memory(&mut memory, prev_result);
            continue;
        } else if tokens[0] == "mem-" {
            add_and_print_memory(&mut memory, -prev_result);
            continue;
        }

        // 式の計算
        let left: f64 = eval_token(tokens[0], memory);
        let right: f64 = eval_token(tokens[2], memory);
        let result = eval_expression(left, tokens[1], right);

        // 結果の表示
        print_result(result);

        prev_result = result;
        // prevの表示
        println!("prev_result: {:}", prev_result);
    }
}

fn print_result(value: f64) {
    println!("result: => {}", value);
}

fn eval_token(token: &str, memory: f64) -> f64 {
    if token == "mem" {
        println!("token is mem");
        memory
    } else {
        token.parse().unwrap()
    }
}

fn eval_expression(left: f64, operator: &str, right: f64) -> f64 {
    match operator {
        "+" => left + right,
        "-" => left - right,
        "*" => left * right,
        "/" => left / right,
        _ => {
            // 入力が正しいならここにはこない
            println!("unreachble");
            unreachable!()
        }
    }
}

fn add_and_print_memory(memory: &mut f64, prev_result: f64) {
    *memory += prev_result;
    print_result(*memory);
}
