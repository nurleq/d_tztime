//rust timezone display time
use std::env;
use std::time::{SystemTime, Instant};
use std::fmt;

use chrono::{Utc, Datelike, Local, TimeZone};
use dotenv::dotenv;

const TZDatabase: &[&str] = include_str!("tz_database.txt");

fn get_timezone_names() -> Vec<String> {
    let mut timezone_names = vec![];
    for tz in TDB_DATABASE.get().unwrap() {
        if !timezone_names.contains(&tz) {
            timezone_names.push(tz.to_string());
        }
    }

    timezone_names
}

fn parse_args() -> (Option<&str>, Option<Vec<String>>) {
    let matches = env::args().collect::<Vec<_>>();
    let tz_all = matches.iter().find(|m| m.starts_with("-all"));
    if let Some(tz_all) = tz_all {
        return (Some(&tz_all[1]), None);
    }

    let list_tz = matches.iter().find(|m| m.starts_with("-list"));
    if let Some(list_tz) = list_tz {
        return (None, Some(get_timezone_names()));
    }

    match env::var("TZ") {
        Ok(tz) => {
            let tz_codes = get_timezone_codes(&tz);
            let tz_args = matches.iter().map(|m| m.split('=').nth(1).unwrap_or_default());
            if let Some(tz_args) = tz_args.next() {
                return (Some(&tz_args), Some(vec![tz]));
            } else {
                return (None, None);
            }
        },
        Err(_) => {
            dotenv().ok();
            return (None, None);
        }
    }
}

fn get_timezone_codes(tz: &str) -> Vec<String> {
    let mut tz_codes = vec![];
    for line in TDB_DATABASE.get().unwrap() {
        if line.starts_with(tz) {
            tz_codes.push(line.to_string());
        }
    }

    tz_codes
}

fn parse_time(time_str: Option<&str>) -> Option<(i32, i64)> {
    match time_str {
        Some(time_str) => {
            let mut parts = time_str.split(':');
            if parts.len() != 3 {
                return None;
            }

            let hours = parts.next().unwrap_or_default().parse::<i32>().ok()?;
            let minutes = parts.next().unwrap_or_default().parse::<i64>().ok()?;
            let seconds = parts.next().unwrap_or_default().parse::<i64>().ok()?;

            Some((hours, minutes))
        },
        None => {
            let now = SystemTime::now();
            let nowInstant = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
            Some((nowInstant / 3600, (nowInstant % 3600) / 60))
        }
    }
}

fn get_time(timezone: &str) -> Option<(i32, i64)> {
    let tz = TimeZone::from_str(timezone).ok()?;
    let now = SystemTime::now();
    let nowInstant = now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() as i64;
    let local = Local::from_utc(nowInstant, &tz);
    Some((local.hour(), local.minute()))

fn main() {
    let (list_tz, tz_args) = parse_args();
    if list_tz.is_some() || tz_args.is_none() {
        println!("{}", list_tz.unwrap_or(""));

        match tz_args {
            None => return,
            Some(tz_args) => {
                for tz in tz_args {
                    let time = get_time(&tz).unwrap_or((0, 0));
                    println!("{}: {:02}:{:02} AM/PM", time.0, time.1, time.1);
                }
            },
        }

        return;
    }

    if !list_tz.is_some() {
        let tz = tz_args.get(0).unwrap_or_default();
        println!("Current Time in {}: {:02}:{:02} AM/PM", tz, get_time(tz).unwrap().0, get_time(tz).unwrap().1);

        return;
    }

    let mut times = vec![];
    for tz in tz_args {
        if let Ok(time) = get_time(&tz) {
            match time.1 {
                0 => println!("{}: {:02}:{:02} AM/PM", tz, time.0, time.1),
                _ => println!("{}: {:02}:{:02}", tz, time.0, time.1)
            };
        } else {
            let now = SystemTime::now();
            match now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() % 86400 {
                0 | 7200 => println!("{}: {:02}:{:02} AM/PM", tz, time.0, time.1),
                _ => println!("{}: {:02}:{:02}", tz, time.0, time.1)
            };
        }
    }

    match env::var("TIME_FORMAT") {
        Ok(format) => {
            let mut times = vec![];
            for tz in tz_args {
                if let Ok(time) = get_time(&tz) {
                    match format.as_str().to_lowercase().contains("am") {
                        true => println!("{}: {:02}:{:02} AM/PM", tz, time.0, time.1),
                        false => println!("{}: {:02}:{:02}", tz, time.0, time.1)
                    };
                } else {
                    let now = SystemTime::now();
                    match now.duration_since(SystemTime::UNIX_EPOCH).unwrap().as_secs() % 86400 {
                        0 | 7200 => println!("{}: {:02}:{:02} AM/PM", tz, time.0, time.1),
                        _ => println!("{}: {:02}:{:02}", tz, time.0, time.1)
                    };
                }
            }

            return;
        },
        Err(_) => {}
    }
}
