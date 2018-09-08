// heavily based on chrono-humanize

extern crate git2;
extern crate tera;

use std::time::{Duration, SystemTime, UNIX_EPOCH};

use git2::Time;
use tera::{from_value, ErrorKind, GlobalFn, Value};

pub trait Humanize {
    fn humanize(&self) -> String;
}

pub enum TimePeriod {
    Now,
    Seconds(i64),
    Minutes(i64),
    Hours(i64),
    Days(i64),
    Weeks(i64),
    Months(i64),
    Years(i64),
    Eternity,
}

pub fn get_current_time() -> i64 {
    let start = SystemTime::now();
    start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs() as i64
}

pub fn humanize_time() -> GlobalFn {
    Box::new(move |args| -> tera::Result<Value> {
        let target = match args.get("target") {
            Some(val) => match from_value::<i64>(val.clone()) {
                Ok(v) => v,
                Err(_) => {
                    return Err(tera::Error::from_kind(ErrorKind::Msg(format!(
                        "`target` must be a i64, got {} instead.",
                        val
                    ))))
                }
            },
            None => {
                return Err(tera::Error::from_kind(ErrorKind::Msg(
                    "No target time given.".to_owned(),
                )))
            }
        };
        Ok(target.humanize().into())
    })
}

impl Humanize for Duration {
    fn humanize(&self) -> String {
        let minute = 60u64;
        let hour = minute * 60u64;
        let day = hour * 24u64;
        let week = day * 7u64;
        let month = day * 30u64;
        let year = day * 365u64;

        match self.as_secs() {
            n if n > year * 2 => format!("{} years", n / year),
            n if n > year => format!("a year"),
            n if n > month * 2 => format!("{} months", n / month),
            n if n > month => format!("a month"),
            n if n > week * 2 => format!("{} weeks", n / week),
            n if n > week => format!("a week"),
            n if n > day * 2 => format!("{} days", n / day),
            n if n > day => format!("a day"),
            n if n > hour * 2 => format!("{} hours", n / hour),
            n if n > hour => format!("an hour"),
            n if n > minute * 2 => format!("{} minutes", n / minute),
            n if n > minute => format!("a minute"),
            n if n > 10 => format!("{} seconds", n),
            0...10 => format!("just now"),
            _ => format!("eternity"),
        }.to_owned()
    }
}

impl Humanize for Time {
    fn humanize(&self) -> String {
        self.seconds().humanize()
    }
}

impl Humanize for i64 {
    fn humanize(&self) -> String {
        use std::i64::{MAX, MIN};

        enum Tense {
            Past,
            Present,
            Future,
        };

        let now = get_current_time();
        let target = *self;
        let elapsed = target - now;

        let tense = match elapsed {
            -10...10 => Tense::Present,
            MIN...-1 => Tense::Past,
            1...MAX => Tense::Future,
            _ => Tense::Present,
        };
        let duration = Duration::new(elapsed.abs() as u64, 0);

        match tense {
            Tense::Past => format!("{} ago", duration.humanize()),
            Tense::Present => String::from("just now"),
            Tense::Future => format!("in {}", duration.humanize()),
        }
    }
}
