use chrono::NaiveDateTime;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::{
    fs::File,
    io::{BufReader, BufWriter},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Schedule {
    // 予定のID
    id: u64,
    // 勉強会の名前
    subject: String,
    // 開始時間
    start: NaiveDateTime,
    // 終了時間
    end: NaiveDateTime,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
struct Calendar {
    schedules: Vec<Schedule>,
}

const SCHEDULE_FILE: &str = "schedule.json";

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// 予定の一覧表示
    List,
    /// 予定の追加
    Add {
        /// タイトル
        subject: String,
        /// 開始時刻
        start: NaiveDateTime,
        /// 終了時刻
        end: NaiveDateTime,
    }
}

fn main() {
    let options = Cli::parse();
    match options.command {
        Commands::List => show_list(),
        Commands::Add {subject, start, end} => add_schedule(subject, start, end)
    }
}

fn show_list() {
    // 予定の読み込み
    let calendar: Calendar = {
        let file = File::open(SCHEDULE_FILE).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };
    // 予定の表示
    println!("ID\tStart\tEND\tSUBJECT");
    for schedule in calendar.schedules {
        println!(
            "{}\t{}\t{}\t{}",
            schedule.id, schedule.start, schedule.end, schedule.subject,
        )
    }
}

fn add_schedule(subject: String, start: NaiveDateTime, end: NaiveDateTime) {
    // 予定の読み込み
    let mut calendar: Calendar = {
        let file = File::open(SCHEDULE_FILE).unwrap();
        let reader = BufReader::new(file);
        serde_json::from_reader(reader).unwrap()
    };

    // 予定の作成
    let id = calendar.schedules.len() as u64;
    let new_schedule = Schedule{
        id, subject, start, end
    };
    // 予定の追加
    calendar.schedules.push(new_schedule);

    // 予定の保存
    {
        let file = File::create(SCHEDULE_FILE).unwrap();
        let writer = BufWriter::new(file);
        serde_json::to_writer(writer, &calendar).unwrap();
    }
    println!("予定を追加しました");
}