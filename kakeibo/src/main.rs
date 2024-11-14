use std::fs::OpenOptions;

use clap::{Args, Parser, Subcommand};
use csv::Writer;
#[derive(Parser)]
#[clap(version = "1.0")]
struct App {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// 新しい口座をつくる
    New(NewArgs),
    /// 口座に入金する
    Deposit,
    /// 口座から出勤する
    Withdraw,
    /// CSVからインポートする
    Import,
    /// レポートを出力する
    Report,
}
#[derive(Args)] // <- help や suggest などを用意してくれる
struct NewArgs {
    account_name: String,
}

impl NewArgs {
    fn run(&self) {
        let file_name = format!("{}.csv", self.account_name);
        let mut writer = Writer::from_path(file_name).unwrap();
        writer.write_record(["日付", "用途", "金額"]).unwrap(); // ヘッダーを書き込む
        writer.flush().unwrap();
    }
}

fn main() {
    // 構造体Argsで定義した形の引数を受け取ることを期待してparseを行う
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit => deposit(),   // TODO: ここを実装する
        Command::Withdraw => withdraw(), // TODO: ここを実装する
        Command::Import => unreachable!(),
        Command::Report => unreachable!(),
    }
}

fn deposit() {
    // 追記モードでファイルを開く
    let open_option = OpenOptions::new()
        .write(true)
        .append(true)
        .open("accounts.csv")
        .unwrap();
    // open_optionを利用した形でwriterを設定
    let mut writer = Writer::from_writer(open_option);
    writer.write_record(["1", "2", "3"]).unwrap(); // ヘッダーを書き込む
    writer.flush().unwrap();
}

fn withdraw() {
    unimplemented!()
}
