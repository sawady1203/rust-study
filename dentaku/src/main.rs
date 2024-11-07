use std::collections::{hash_map::Entry, HashMap};
use std::io::stdin;
use std::result;

fn main() {
    let mut memory = Memory::new();
    let mut prev_result: f64 = 0.0;
    for line in stdin().lines() {
        // 1行読み取って空行なら終了
        let line = line.unwrap();
        if line.is_empty() {
            break;
        }
        // トークン列に分割
        let tokens = Token::split(&line);
        println!("{:?}", tokens);
        // 式の評価
        match &tokens[0] {
            Token::MemoryPlus(memory_name) => {
                // メモリへの加算
                let memory_name = memory_name.to_string();
                let result = memory.add(&memory_name, prev_result);
                println!("{:?}", result);
                // print_result(result);
            }
            Token::MemoryMinus(memory_name) => {
                // メモリへの減算
                let memory_name = memory_name.to_string();
                let result = memory.add(&memory_name, -prev_result);
                println!("{:?}", result);
            }
            _ => {
                // 式の値の計算
                let left = eval_token(&tokens[0], &memory);
                let right = eval_token(&tokens[2], &memory);
                let result = eval_expression(left, &tokens[1], right);
                println!("{:?}", result);
                prev_result = result;
            }
        }
    }
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
    fn add(&mut self, token: &str, prev_result: f64) {
        let slot_name = &token[3..token.len() - 1].to_string();
        match self.slots.entry(slot_name.to_string()) {
            Entry::Occupied(mut entry) => {
                // メモリが見つかったので、値を更新、表示して終了
                *entry.get_mut() += prev_result;
                *entry.get();
            }
            Entry::Vacant(entry) => {
                // メモリが見つからなかったので、要素を追加する
                entry.insert(prev_result);
                prev_result;
            }
        }
    }
    fn get(&self, slot_name: &str) -> f64 {
        self.slots.get(slot_name).copied().unwrap_or(0.0)
    }
}

#[derive(Debug, PartialEq)]
enum Token {
    Number(f64),
    MemoryRef(String),
    MemoryPlus(String),
    MemoryMinus(String),
    Plus,
    Minus,
    Asterisk,
    Slash,
}

impl Token {
    fn parse(value: &str) -> Self {
        match value {
            "+" => Self::Plus,
            "-" => Self::Minus,
            "*" => Self::Asterisk,
            "/" => Self::Slash,
            _ if value.starts_with("mem") => {
                let mut memory_name = value[3..].to_string();
                if value.ends_with("+") {
                    memory_name.pop(); // 末尾の１文字を削除
                    Self::MemoryPlus(memory_name)
                } else if value.ends_with("-") {
                    memory_name.pop();
                    Self::MemoryMinus(memory_name)
                } else {
                    Self::MemoryRef(memory_name)
                }
            }
            _ => Self::Number(value.parse().unwrap()),
        }
    }
    fn split(text: &str) -> Vec<Self> {
        text.split(char::is_whitespace).map(Self::parse).collect()
    }
}

fn eval_token(token: &Token, memory: &Memory) -> f64 {
    match token {
        Token::Number(value) => {
            // 数値
            *value
        }
        Token::MemoryRef(memory_name) => memory.get(memory_name),
        _ => {
            // 入力が正しいなら、ここにこない
            unreachable!()
        }
    }
}

fn eval_expression(left: f64, operator: &Token, right: f64) -> f64 {
    match operator {
        Token::Plus => left + right,
        Token::Minus => left - right,
        Token::Asterisk => left * right,
        Token::Slash => left / right,
        _ => {
            // 入力が正しいならここにはこない
            unreachable!()
        }
    }
}
