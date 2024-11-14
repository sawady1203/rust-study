use chrono::NaiveDate;
use clap::{Args, Parser, Subcommand};
use csv::{Reader, Writer};
use std::fs::OpenOptions;
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
    Deposit(DepositArgs),
    /// 口座から出勤する
    Withdraw(WithdrawArgs),
    /// CSVからインポートする
    Import(ImportArgs),
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

#[derive(Args)]
struct DepositArgs {
    // 引数の型を定義
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl DepositArgs {
    fn run(&self) {
        // 追記モードでファイルを開く
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // open_optionを利用した形でwriterを設定
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                self.amount.to_string(),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct WithdrawArgs {
    account_name: String,
    date: NaiveDate,
    usage: String,
    amount: u32,
}

impl WithdrawArgs {
    fn run(&self) {
        // 追記モードでファイルを開く
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.account_name))
            .unwrap();
        // open_optionを利用した形でwriterを設定
        let mut writer = Writer::from_writer(open_option);
        writer
            .write_record(&[
                self.date.format("%Y-%m-%d").to_string(),
                self.usage.to_string(),
                format!("-{}", self.amount),
            ])
            .unwrap();
        writer.flush().unwrap();
    }
}

#[derive(Args)]
struct ImportArgs {
    src_file_name: String,
    dst_account_name: String,
}

impl ImportArgs {
    fn run(&self) {
        // 追記モードでファイルを開く
        let open_option = OpenOptions::new()
            .write(true)
            .append(true)
            .open(format!("{}.csv", self.dst_account_name))
            .unwrap();
        let mut writer = Writer::from_writer(open_option);
        let mut reader = Reader::from_path(&self.src_file_name).unwrap();
        for result in reader.records() {
            // Reader は先頭行をヘッダーとして扱うので
            // このループは2行目以降で実行する
            let record = result.unwrap();
            writer.write_record(&record).unwrap();
        }
        writer.flush().unwrap();
    }
}

fn main() {
    // 構造体Argsで定義した形の引数を受け取ることを期待してparseを行う
    let args = App::parse();
    match args.command {
        Command::New(args) => args.run(),
        Command::Deposit(args) => args.run(),
        Command::Withdraw(args) => args.run(),
        Command::Import(args) => args.run(),
        Command::Report => unreachable!(),
    }
}
