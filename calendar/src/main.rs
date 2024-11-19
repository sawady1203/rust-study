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

impl Schedule {
    fn intersects(&self, other: &Schedule) -> bool {
        self.start < other.end
    }
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

    // 予定の重複判定
    for schedule in &calendar.schedules {
        if schedule.intersects(&new_schedule){
            println!("エラー：予定が重複しています");
            return;
        }
    }


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

#[cfg(test)]
mod tests{
    use super::*;

    fn naive_date_time(
        year: i32,
        month: u32,
        day: u32,
        hour: u32,
        minute: u32,
        second: u32,
    ) -> NaiveDateTime {
        chrono::NaiveDate::from_ymd_opt(year, month, day)
            .unwrap()
            .and_hms_opt(hour, minute, second)
            .unwrap()
    }

    #[test]
    fn test_schedule_intersects_1() {
        // 2024年1月1日 18:15 ~ 19:15 までの既存予定:1
        let schedule = Schedule {
            id: 1,
            subject: "既存予定 1".to_string(),
            start: naive_date_time(2024, 1, 1, 18, 15, 00),
            end: naive_date_time(2024, 1, 1, 19, 15, 00),
        };
        // 2024年1月1日 19:00 ~ 20:00 までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 00, 00),
            end: naive_date_time(2024, 1, 1, 20, 00, 00),
        };
        // 既存予定1と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }
    #[test]
    fn test_schedule_intersects_2() {
        // 2024年1月1日 19:45 ~ 20:45 までの既存予定:1
        let schedule = Schedule {
            id: 1,
            subject: "既存予定 2".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 45, 00),
            end: naive_date_time(2024, 1, 1, 20, 15, 00),
        };
        // 2024年1月1日 19:00 ~ 20:00 までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定2".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 00, 00),
            end: naive_date_time(2024, 1, 1, 20, 00, 00),
        };
        // 既存予定1と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }
    #[test]
    fn test_schedule_intersects_3() {
        // 2024年1月1日 18:30 ~ 20:15 までの既存予定:1
        let schedule = Schedule {
            id: 1,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 18, 30, 00),
            end: naive_date_time(2024, 1, 1, 20, 15, 00),
        };
        // 2024年1月1日 19:00 ~ 20:00 までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 00, 00),
            end: naive_date_time(2024, 1, 1, 20, 00, 00),
        };
        // 既存予定1と新規予定は重複している
        assert!(schedule.intersects(&new_schedule));
    }
    #[test]
    fn test_schedule_intersects_4() {
        // 2024年1月1日 20:15 ~ 20:45 までの既存予定:1
        let schedule = Schedule {
            id: 1,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 20, 15, 00),
            end: naive_date_time(2024, 1, 1, 20, 45, 00),
        };
        // 2024年1月1日 19:00 ~ 20:00 までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 1, 19, 00, 00),
            end: naive_date_time(2024, 1, 1, 20, 00, 00),
        };
        // 既存予定1と新規予定は重複しない
        assert!(!schedule.intersects(&new_schedule));
    }
    #[test]
    fn test_schedule_intersects_5() {
        // 2024年1月1日 18:15 ~ 19:15 までの既存予定:1
        let schedule = Schedule {
            id: 1,
            subject: "既存予定".to_string(),
            start: naive_date_time(2024, 1, 1, 18, 15, 00),
            end: naive_date_time(2024, 1, 1, 19, 15, 00),
        };
        // 2024年1月2日 19:00 ~ 20:00 までの新規予定
        let new_schedule = Schedule {
            id: 999,
            subject: "新規予定".to_string(),
            start: naive_date_time(2024, 1, 2, 19, 00, 00),
            end: naive_date_time(2024, 1, 2, 20, 00, 00),
        };
        // 既存予定1と新規予定は重複しない
        assert!(!schedule.intersects(&new_schedule));
    }
}