use clap::Parser;
#[derive(Parser)]
struct Args {
    arg1: String,
    arg2: String,
}

fn main() {
    // 構造体Argsで定義した形の引数を受け取ることを期待してparseを行う
    let _args = Args::parse();
    let command_name = std::env::args().nth(0).unwrap_or("CLI".to_string());
    let name = std::env::args().nth(1).unwrap_or("World".to_string());
    println!("Hello {} via {}!", name, command_name);
}
