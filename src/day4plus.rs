#[macro_use]
extern crate lazy_static;
extern crate chrono;
extern crate regex;

use chrono::prelude::*;
use regex::Regex;
use std::io::{self, prelude::*};
use std::collections::HashMap;

lazy_static! {
    static ref RE: Regex =
        Regex::new("[\\[](\\d{4})-(\\d{2})-(\\d{2}) (\\d{2}):(\\d{2})[\\]] (.*)").unwrap();
    static ref ID: Regex =
        Regex::new("[\\#](\\d+)").unwrap();
}

#[derive(Debug)]
struct Log {
    year: DateTime<Utc>,
    log: String,
}

#[derive(Debug)]
struct Slept{
    minutes: Vec<u32>,
    times: u32
}

fn main() -> Result<(), failure::Error> {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input)?;

    let mut logs: Vec<Log> = Vec::new();
    let mut guard_big: HashMap<u32, Slept> = HashMap::new();

    for line in input.lines() {
        let time_info = RE.captures(&line).unwrap();
        logs.push(Log {
            year: Utc
                .ymd(
                    time_info[1].parse::<i32>().unwrap(),
                    time_info[2].parse::<u32>().unwrap(),
                    time_info[3].parse::<u32>().unwrap(),
                )
                .and_hms(
                    time_info[4].parse::<u32>().unwrap(),
                    time_info[5].parse::<u32>().unwrap(),
                    0,
                ),
            log: time_info[6].to_string(),
        });
    }
    logs.sort_by_key(|x| x.year);

    let mut id = 0;
    let mut sleep_date: DateTime<Utc> = Utc::now();
    let mut is_asleep = false;
    for log in &logs {
        let mut meter = Slept{ minutes: vec![], times: 0 };
        if log.log.contains("#") {
            let parsed_id = &ID.captures(&log.log).unwrap()[1];
            id = parsed_id.parse::<u32>().unwrap();
            guard_big.entry(id).or_insert(meter);
        } else if log.log.contains("f"){
            is_asleep = true;
            sleep_date = log.year;
            let mut slept = guard_big.entry(id).or_insert(meter);
            slept.times += 1;
        }else {
            let mut slept = guard_big.entry(id).or_insert(meter);
            let time_asleep = log.year.minute() - sleep_date.minute();
            for i in sleep_date.minute()..log.year.minute() {
                slept.minutes.push(i);
            }
        }
    }

    let mut max_value = 0;
    let mut bigger_guard = 0;
    for (key, value) in &guard_big {
        if value.times > max_value {
            max_value = value.times;
            bigger_guard = *key;
        }
    }

    let mut guard_minute_times: HashMap<u32, (u32, u32)> = HashMap::new();
    for (id, slept) in guard_big {
        let mut repeated_minutes: HashMap<u32, u32> = HashMap::new();
        for minute in slept.minutes {
            *repeated_minutes.entry(minute).or_insert(0) += 1;
        }
        let (mut minute, mut highest): (u32, u32) = (0,0);
        for (minute2, value) in &repeated_minutes {
            if value > &highest {
                minute = *minute2;
                highest = *value;
            }
        }
        guard_minute_times.insert(id.clone(), (minute, highest));

    }
    let (mut guard, mut minute, mut highest): (u32, u32 ,u32) = (0,0,0);
    for (id, (minute2, times)) in guard_minute_times {
        if times > highest {
            guard = id;
            minute = minute2;
            highest = times;
        }
    }
    println!("{:?}", guard*minute);
    Ok(())
}
