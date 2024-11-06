use std::collections::{hash_map::Entry, HashMap};
use std::io::stdin;

fn main() {
    let mut memory = Memory::new();
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
        let is_memory = tokens[0].starts_with("mem");
        if is_memory && tokens[0].ends_with("+") {
            memory.add_and_print(tokens[0], prev_result);
            continue;
        } else if is_memory && tokens[0].ends_with("-") {
            memory.add_and_print(tokens[0], -prev_result);
            continue;
        }

        // 式の計算
        // memoriesを参照渡しにする
        let left: f64 = memory.eval_token(tokens[0]);
        let right: f64 = memory.eval_token(tokens[2]);
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

struct Memory {
    // メモリの名前と値の組を辞書で保存する
    slots: HashMap<String, f64>,
}

impl Memory {
    fn new() -> Self {
        Self {
            slots: HashMap::new(),
        }
    }
    fn add_and_print(&mut self, token: &str, prev_result: f64) {
        let slot_name = &token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name.to_string()) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新、表示して終了
                *entry.get_mut() += prev_result;
                print_result(*entry.get());
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、要素を追加する
                entry.insert(prev_result);
                print_result(prev_result);
            }
        }
    }

    fn eval_token(&self, token: &str) -> f64 {
        if token.starts_with("mem") {
            let slot_name = &token[3..];
            // すべてのメモリを探索する
            self.slots.get(slot_name).copied().unwrap_or(0.0)
        } else {
            token.parse().unwrap()
        }
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
